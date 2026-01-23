# Plan: Decrypt and Display GPG Secrets

## Overview

This plan outlines the implementation of GPG secret decryption and display functionality for Defira. The goal is to allow users to:
1. Click on a `.gpg` file in the file tree
2. Enter their password in the existing password prompt modal
3. Have the file decrypted and displayed in the UI

## Current State

### What Already Exists
- **Password prompt modal** (`password_prompt.rs`) - Collects password and target file path
- **Text editor panel** - Right-side panel for displaying file content
- **Error popup modal** (`error_popup.rs`) - Displays error messages
- **File detection** - `.gpg` files are detected and trigger the password prompt (`file_explorer.rs:63`)

### What's Missing
- **GPG library** - No cryptographic dependencies in `Cargo.toml`
- **Decryption logic** - TODO at `file_explorer.rs:174`
- **Integration** - Password captured but not used for decryption

---

## Implementation Phases

### Phase 1: Add GPG/PGP Library

**Objective:** Add a Rust library for GPG decryption.

**Options:**

| Library | Pros | Cons |
|---------|------|------|
| `sequoia-pgp` | Pure Rust, modern, well-maintained | Large dependency tree |
| `gpgme` | Uses system GPG, full feature set | Requires system libgpgme |
| `pgp` | Pure Rust, lighter weight | Less feature-complete |

**Recommendation:** Use `sequoia-pgp` or `gpgme` depending on whether you want pure Rust or system GPG integration.

**Changes:**
- `Cargo.toml` - Add chosen GPG library dependency

---

### Phase 2: Create Crypto Module

**Objective:** Create a dedicated module for cryptographic operations.

**New File:** `src/crypto.rs`

**Functions to implement:**

```rust
/// Decrypt a GPG-encrypted file using a password
///
/// # Arguments
/// * `encrypted_data` - The raw encrypted file bytes
/// * `password` - The password/passphrase for decryption
///
/// # Returns
/// * `Ok(String)` - The decrypted plaintext content
/// * `Err(CryptoError)` - Decryption failure (wrong password, corrupted file, etc.)
pub fn decrypt_with_password(
    encrypted_data: &[u8],
    password: &str,
) -> Result<String, CryptoError>;
```

**Error types to define:**

```rust
pub enum CryptoError {
    WrongPassword,
    CorruptedFile,
    UnsupportedFormat,
    IoError(std::io::Error),
    Other(String),
}
```

**Changes:**
- Create `src/crypto.rs`
- Add `mod crypto;` to `src/main.rs` or make it part of the library

---

### Phase 3: Integrate Decryption with Password Prompt

**Objective:** Connect the password prompt submission to actual decryption.

**Location:** `src/file_explorer.rs:170-177`

**Current code:**
```rust
Message::Submit => {
    if let Some(prompt) = state.password_prompt.take() {
        log::debug!("Password entered (length: {})", prompt.password.len());
        // TODO: Use password to decrypt the file
        // For now, we just close the modal
    }
}
```

**Proposed flow:**

1. Read encrypted file bytes from `prompt.target_path`
2. Call `crypto::decrypt_with_password(&bytes, &prompt.password)`
3. On success: Display decrypted content in editor
4. On failure: Show error popup with appropriate message

**Pseudocode:**
```rust
Message::Submit => {
    if let Some(prompt) = state.password_prompt.take() {
        // Read encrypted file
        match std::fs::read(&prompt.target_path) {
            Ok(encrypted_bytes) => {
                // Attempt decryption
                match crypto::decrypt_with_password(&encrypted_bytes, &prompt.password) {
                    Ok(plaintext) => {
                        // Display in editor
                        state.opened_file = Some(prompt.target_path);
                        state.editor_content = Some(text_editor::Content::with_text(&plaintext));
                    }
                    Err(crypto::CryptoError::WrongPassword) => {
                        state.error_popup = Some(error_popup::State::new(
                            "Decryption Failed",
                            "Incorrect password. Please try again.",
                        ));
                    }
                    Err(e) => {
                        state.error_popup = Some(error_popup::State::new(
                            "Decryption Error",
                            &format!("Failed to decrypt file: {}", e),
                        ));
                    }
                }
            }
            Err(e) => {
                state.error_popup = Some(error_popup::State::new(
                    "File Read Error",
                    &format!("Could not read file: {}", e),
                ));
            }
        }
    }
}
```

---

### Phase 4: Security Considerations

**Objective:** Ensure decrypted content is handled securely.

**Considerations:**

1. **Memory clearing** - Consider using `zeroize` crate to clear password from memory after use
2. **Read-only mode** - Decrypted content should initially be read-only to prevent accidental modification
3. **No caching** - Don't persist decrypted content to disk
4. **Clear on close** - Zero out decrypted content when editor is closed
5. **Timeout** - Consider auto-locking after period of inactivity (optional)

**Optional dependency:** `zeroize = "1.x"` for secure memory clearing

---

### Phase 5: UI Enhancements (Optional)

**Objective:** Improve the display of decrypted secrets.

**Option A: Use Existing Editor**
- Simplest approach
- Reuse `text_editor::Content` with decrypted plaintext
- Add visual indicator that content is decrypted (e.g., lock icon in title)

**Option B: Create Dedicated Secret Viewer Modal**
- More secure - content isolated in modal
- Add "Copy to Clipboard" button for individual secret values
- Auto-close after timeout
- Hide content by default with "Reveal" button

**Recommendation:** Start with Option A, iterate to Option B if needed.

---

## File Changes Summary

| File | Action | Description |
|------|--------|-------------|
| `Cargo.toml` | Modify | Add GPG library dependency |
| `src/crypto.rs` | Create | New module for decryption logic |
| `src/main.rs` | Modify | Add `mod crypto;` declaration |
| `src/file_explorer.rs` | Modify | Integrate decryption at line 170-177 |

---

## Testing Strategy

1. **Unit tests for crypto module**
   - Test successful decryption with correct password
   - Test failure with wrong password
   - Test handling of corrupted files
   - Test handling of non-GPG files

2. **Integration tests**
   - Create test `.gpg` file with known password
   - Verify decryption produces expected plaintext

3. **Manual testing**
   - Test with real GPG-encrypted files
   - Test UI flow end-to-end
   - Test error scenarios (wrong password, missing file)

---

## Implementation Order

1. [ ] Add GPG library to `Cargo.toml`
2. [ ] Create `src/crypto.rs` with `decrypt_with_password()` function
3. [ ] Add `mod crypto;` to `src/main.rs`
4. [ ] Write unit tests for crypto module
5. [ ] Integrate decryption in `file_explorer.rs` Submit handler
6. [ ] Test end-to-end flow
7. [ ] Add security enhancements (zeroize, read-only mode)
8. [ ] (Optional) Create dedicated secret viewer modal

---

## Open Questions

1. **Which GPG library to use?**
   - `sequoia-pgp` (pure Rust) vs `gpgme` (system GPG)
   - Depends on deployment requirements and whether users have GPG installed

2. **Should decrypted content be editable?**
   - Read-only is more secure
   - Editing requires re-encryption on save (additional complexity)

3. **Should we support key-based decryption?**
   - Current plan assumes password/passphrase-based encryption
   - Key-based would require accessing GPG keyring

4. **Auto-lock timeout?**
   - Should decrypted content auto-hide after inactivity?
   - What timeout value is appropriate?

---

## Estimated Complexity

- **Phase 1 (Library):** Low - just adding a dependency
- **Phase 2 (Crypto module):** Medium - understanding GPG library API
- **Phase 3 (Integration):** Low - straightforward code changes
- **Phase 4 (Security):** Medium - requires careful handling
- **Phase 5 (UI):** Low-Medium - depends on chosen approach

---

## References

- [Sequoia-PGP Documentation](https://docs.sequoia-pgp.org/)
- [GPGME Rust Bindings](https://docs.rs/gpgme/)
- [Iced Framework](https://iced.rs/)
- Existing modals: `password_prompt.rs`, `error_popup.rs`
