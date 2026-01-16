# Password Prompt Implementation Plan

## Overview

Implement a generic, reusable password prompt dialog for decrypting `.gpg` files in the file explorer. The prompt will be triggered when opening encrypted files (line 56 of `file_explorer.rs`).

## Approach: Modal Overlay

Instead of opening a new OS window, we'll use a **modal overlay** approach (similar to how `context_menu.rs` works). This is the idiomatic way to handle dialogs in iced and provides:

- Consistent look and feel with the rest of the application
- No platform-specific window management issues
- Simpler state management within the TEA pattern

**Alternative considered:** iced does support multi-window applications, but it requires more complex setup with `Application` trait, window IDs, and inter-window messaging. A modal overlay is simpler and sufficient for a password prompt.

---

## Implementation Steps

### 1. Create `password_prompt.rs` Module

A new generic module that can be reused throughout the application.

**File: `src/password_prompt.rs`**

```rust
// Generic password prompt component
// - State struct for the prompt (input value, visibility toggle)
// - Message enum for user interactions (TextChanged, Submit, Cancel, ToggleVisibility)
// - view() function to render the modal
// - Helper functions for creating the overlay
```

**Components:**
- `State` struct:
  - `password: String` - current input value
  - `show_password: bool` - toggle for password visibility (optional)
  - `target_path: PathBuf` - the file being decrypted (for context)

- `Message` enum:
  - `PasswordChanged(String)` - text input changed
  - `Submit` - user pressed Enter or clicked OK
  - `Cancel` - user clicked Cancel or pressed Escape
  - `ToggleVisibility` - show/hide password (optional feature)

- Public functions:
  - `view()` - renders the modal overlay with text input, buttons
  - `create_backdrop()` - semi-transparent background that blocks interaction

### 2. Update `file_explorer.rs`

**Changes to `State` struct:**
```rust
pub struct State {
    // ... existing fields ...
    password_prompt: Option<password_prompt::State>,
    pending_decrypt_path: Option<PathBuf>,  // File waiting for password
}
```

**Changes to `FileAction` enum:**
```rust
pub enum FileAction {
    // ... existing variants ...
    PasswordPrompt(password_prompt::Message),
    OpenPasswordPrompt(PathBuf),
}
```

**Changes to `update()` function:**
- Handle `OpenPasswordPrompt` to show the modal
- Handle `PasswordPrompt(msg)` to delegate to password prompt logic
- On `Submit`: retrieve password from state, call decryption, clear prompt
- On `Cancel`: clear the password prompt state

**Changes to `view()` function:**
- When `password_prompt` is `Some`, render the modal overlay on top using `Stack`

### 3. Update `open_file_in_editor()` Function

At line 55-57, replace the comment with actual logic:

```rust
fn open_file_in_editor(state: &mut State, path: &PathBuf) -> Option<FileAction> {
    if path.extension().is_some_and(|ext| ext == "gpg") {
        // Return an action to open the password prompt
        return Some(FileAction::OpenPasswordPrompt(path.clone()));
    }
    // ... rest of the function
}
```

**Note:** This requires changing the function signature to return an optional action, or changing the approach to set state directly.

### 4. Update `main.rs`

Add the new module:
```rust
mod password_prompt;
```

---

## Detailed Design

### Password Prompt UI Layout

```
┌─────────────────────────────────────────┐
│           Enter Password                │
│                                         │
│  Decrypting: filename.gpg               │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │ ●●●●●●●●                        │👁  │
│  └─────────────────────────────────┘   │
│                                         │
│              [Cancel]  [OK]             │
└─────────────────────────────────────────┘
```

- Centered modal with semi-transparent backdrop
- Title "Enter Password"
- Shows filename being decrypted
- Password text input (secure/masked)
- Optional visibility toggle button
- Cancel and OK buttons
- Pressing Enter submits, Escape cancels

### State Flow

1. User clicks on `file.gpg` in file explorer
2. `FileAction::Select(path)` triggers `open_file_in_editor()`
3. Function detects `.gpg` extension → sets `password_prompt` state
4. `view()` detects `password_prompt.is_some()` → renders modal overlay
5. User types password, clicks OK (or presses Enter)
6. `FileAction::PasswordPrompt(Submit)` handled in `update()`
7. Password retrieved: `let password = state.password_prompt.as_ref().unwrap().password.clone();`
8. Password prompt state cleared
9. Decryption logic called with password (future implementation)
10. File content displayed in editor

### Generic Design Considerations

The `password_prompt` module should be generic enough to:
- Be reused for other password prompts (e.g., creating new encrypted files)
- Accept a custom title/message
- Optionally show a "confirm password" field for creation scenarios
- Return the password via callback/message pattern

---

## Files to Create/Modify

| File | Action | Description |
|------|--------|-------------|
| `src/password_prompt.rs` | **Create** | New generic password prompt module |
| `src/file_explorer.rs` | Modify | Add password prompt state and handling |
| `src/main.rs` | Modify | Add `mod password_prompt;` declaration |

---

## Open Questions

1. **Password visibility toggle:** Should we include a button to show/hide the password? (Recommend: yes, for usability)

2. **Error handling:** How should we display decryption errors (wrong password)? Options:
   - Show error text in the modal and let user retry
   - Close modal and show error toast/notification

3. **Remember password:** Should there be an option to remember the password for the session? (Can be added later)

4. **Keyboard shortcuts:** Implement Enter to submit and Escape to cancel? (Recommend: yes)

---

## Estimated Scope

- **password_prompt.rs**: ~150-200 lines
- **file_explorer.rs changes**: ~50-70 lines
- **main.rs changes**: 1 line

---

## Next Steps After Approval

1. Create `src/password_prompt.rs` with State, Message, and view()
2. Update `src/file_explorer.rs` to integrate the password prompt
3. Update `src/main.rs` to include the new module
4. Test the modal overlay behavior
5. (Future) Integrate with actual GPG decryption logic
