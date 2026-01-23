# Plan: Generic Error Message Popup

## Overview

Implement a reusable error message popup component with an OK button that displays errors to users. The first use case will be showing file read failures in `open_file_in_editor`.

## Architecture

Follow the existing `password_prompt.rs` pattern which uses:
- A dedicated module with `State`, `Message`, `view()`, and `create_backdrop()`
- Integration via a nested message variant in `FileAction`
- Modal rendering using `iced::widget::Stack`

## Implementation Steps

### Step 1: Create `error_popup.rs` Module

Create `src/error_popup.rs` with:

```rust
pub struct State {
    pub title: String,
    pub message: String,
}

pub enum Message {
    Dismiss,
}
```

**Functions:**
- `State::new(title: impl Into<String>, message: impl Into<String>) -> Self`
- `create_backdrop<M>(on_dismiss: M) -> impl Into<Element<M>>` - semi-transparent overlay
- `view<M>(state: &State, on_message: fn(Message) -> M) -> impl Into<Element<M>>` - the popup UI

**UI Layout:**
- Centered container with white background, border, and shadow
- Title text (bold/larger)
- Message text
- "OK" button that emits `Message::Dismiss`

### Step 2: Add Module Declaration

In `main.rs`, add:
```rust
mod error_popup;
```

### Step 3: Extend `FileAction` Enum

In `file_explorer.rs`, add variant:
```rust
pub enum FileAction {
    // ... existing variants
    ErrorPopup(error_popup::Message),
}
```

### Step 4: Add State Field

In `file_explorer.rs`, extend `State`:
```rust
pub struct State {
    // ... existing fields
    pub error_popup: Option<error_popup::State>,
}
```

Initialize as `None` in `State::new()`.

### Step 5: Handle Error Popup Messages

In `update()` function, add match arm:
```rust
FileAction::ErrorPopup(msg) => match msg {
    error_popup::Message::Dismiss => {
        state.error_popup = None;
    }
}
```

### Step 6: Trigger Popup on File Read Error

In `open_file_in_editor()`, modify the error handling (around line 71):

**Before:**
```rust
Err(e) => {
    log::error!("Failed to read file: {}", e);
    state.editor_content = None;
}
```

**After:**
```rust
Err(e) => {
    log::error!("Failed to read file: {}", e);
    state.editor_content = None;
    state.error_popup = Some(error_popup::State::new(
        "Error",
        format!("Failed to read file: {}", e),
    ));
}
```

### Step 7: Render the Error Popup

In `view()` function, modify the final return to check for error popup (similar to password_prompt):

```rust
if let Some(error_state) = &state.error_popup {
    let backdrop = error_popup::create_backdrop(FileAction::ErrorPopup(error_popup::Message::Dismiss));
    let modal = error_popup::view(error_state, FileAction::ErrorPopup);

    widget::Stack::new()
        .push(split_layout)  // or current content
        .push(backdrop)
        .push(modal)
        .into()
} else if let Some(prompt_state) = &state.password_prompt {
    // existing password prompt handling
} else {
    // existing default case
}
```

## File Changes Summary

| File | Change |
|------|--------|
| `src/error_popup.rs` | New file - popup component |
| `src/main.rs` | Add `mod error_popup;` |
| `src/file_explorer.rs` | Add `ErrorPopup` variant, state field, update handler, view integration |

## Testing

1. Attempt to open a file that doesn't exist or lacks read permissions
2. Verify popup appears with error message
3. Click OK button - popup should dismiss
4. Click backdrop - popup should dismiss
5. Verify popup styling matches password_prompt modal
