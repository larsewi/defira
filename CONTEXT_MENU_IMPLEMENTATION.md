# Context Menu Implementation Plan

## Overview

This document outlines the plan for implementing a right-click context menu for files and directories in the Defira file explorer.

## Current State

- **Framework**: Iced 0.13.1 (Rust GUI framework using Elm architecture)
- **Main file**: `src/explorer.rs` (286 lines)
- **Already implemented**: Right-click detection via `FileAction::ContextMenu(String, bool)`
- **Assets ready**: `delete.svg`, `edit.svg`, and other SVG icons already embedded in `src/assets.rs`

### Key Finding

The context menu foundation is already partially implemented! The right-click event handler exists at `src/explorer.rs:97` and calls `FileAction::ContextMenu`, but currently only logs debug messages.

## Implementation Phases

### Phase 1: State Management & Positioning

**Goal**: Track context menu state and capture mouse position

#### 1.1 Extend the State struct (`src/explorer.rs:24`)

```rust
pub struct State {
    expanded: HashSet<String>,
    selected: HashSet<String>,
    context_menu: Option<ContextMenuState>,  // NEW
}

pub struct ContextMenuState {
    target_path: String,
    is_directory: bool,
    position: (f32, f32),  // x, y coordinates
}
```

#### 1.2 Capture mouse position

- Modify the `on_right_press()` handler to capture cursor position
- Iced provides position through event data that needs to be extracted
- Update `FileAction::ContextMenu` to include position coordinates

#### 1.3 Add menu actions to FileAction enum

```rust
#[derive(Debug, Clone)]
pub enum FileAction {
    Select(String, bool),
    ContextMenu(String, bool, (f32, f32)),  // Add position
    CloseContextMenu,                        // NEW
    DeleteItem(String),                      // NEW
    // Future actions: Rename, Copy, etc.
}
```

### Phase 2: Context Menu UI Component

**Goal**: Create the visual context menu that appears on right-click

#### 2.1 Create context menu rendering function

```rust
fn view_context_menu<'a>(state: &ContextMenuState) -> Element<'a, FileAction> {
    // Build a widget::container() with absolute positioning
    // Style with border, shadow, and background
    // Position at right-click coordinates
}
```

**Styling requirements**:
- Semi-transparent or solid background
- Border for definition
- Drop shadow for depth
- Rounded corners (consistent with app style)

#### 2.2 Add menu items

**Initial implementation** (single item):
- "Delete" button with `delete.svg` icon
- Style buttons with hover effects (consistent with existing UI at line 76-87)
- Structure as vertical list using `widget::Column`

**Button structure**:
```
[Icon] Delete
```

#### 2.3 Implement click-outside-to-dismiss

- Use `widget::mouse_area()` covering full window
- Place behind context menu in z-order
- Triggers `CloseContextMenu` action when clicked
- Also dismiss on ESC key press

#### 2.4 Integrate into main view

- Render main file list as usual
- Conditionally render context menu overlay on top (if `state.context_menu.is_some()`)
- Use `widget::Stack` or layering approach for z-positioning

### Phase 3: Delete Functionality

**Goal**: Implement actual file/directory deletion

#### 3.1 Implement delete operation in update function

Location: `src/explorer.rs:40-62` (update function)

```rust
FileAction::DeleteItem(path) => {
    if let Err(e) = if Path::new(&path).is_dir() {
        fs::remove_dir_all(&path)
    } else {
        fs::remove_file(&path)
    } {
        log::error!("Failed to delete {}: {}", path, e);
    } else {
        log::info!("Deleted: {}", path);
    }

    // Clear context menu
    self.context_menu = None;

    // Clear selection if deleted item was selected
    self.selected.remove(&path);

    // Refresh directory contents (automatic via view re-render)
}
```

#### 3.2 Add error handling

- Log errors using existing `log::error!()` infrastructure
- Consider adding user-visible error messages (future enhancement)
- Handle permission denied, file not found, directory not empty, etc.

#### 3.3 Update UI after deletion

- Context menu automatically closes
- Selection cleared if deleted item was selected
- Directory list refreshes automatically via Iced's reactive model

### Phase 4: Conditional Menu Items

**Goal**: Different menu items for files vs directories

#### 4.1 Create menu item builder function

```rust
fn build_menu_items<'a>(
    is_directory: bool,
    target_path: &str,
) -> Element<'a, FileAction> {
    if is_directory {
        // Directory menu items
        column![
            menu_button("Delete", DELETE_LOGO, FileAction::DeleteItem(target_path)),
            // Future: "Rename", "New File", "New Folder", etc.
        ]
    } else {
        // File menu items
        column![
            menu_button("Delete", DELETE_LOGO, FileAction::DeleteItem(target_path)),
            // Future: "Rename", "Copy", "Open", etc.
        ]
    }
}
```

#### 4.2 Menu item differences

**For Files**:
- Delete
- Rename (future)
- Copy (future)
- Open/Edit (future)

**For Directories**:
- Delete
- Rename (future)
- New File (future)
- New Folder (future)
- Copy Path (future)

## Technical Architecture

### Iced Elm Architecture Pattern

```
User Action (Right-click)
    ↓
FileAction::ContextMenu event
    ↓
update() modifies State
    ↓
view() renders new UI with context menu
    ↓
User clicks "Delete"
    ↓
FileAction::DeleteItem event
    ↓
update() performs deletion & updates State
    ↓
view() re-renders without deleted item
```

### Key Iced Widgets Used

- `widget::container()` - Context menu container with positioning
- `widget::button()` - Menu item buttons
- `widget::mouse_area()` - Click detection (both right-click and dismiss)
- `widget::Column` - Vertical menu item list
- `widget::row!()` - Icon + text layout
- `widget::svg()` - Menu item icons
- `widget::Stack` - Layering context menu over file list (if needed)

### Positioning Strategy

1. **Basic**: Display at cursor position (x, y)
2. **Edge detection** (future enhancement):
   - If menu would extend beyond right edge, flip to left of cursor
   - If menu would extend beyond bottom edge, flip to above cursor

## File Structure Impact

All changes will be contained in **`src/explorer.rs`**:

| Lines | Change Description |
|-------|-------------------|
| 24-26 | Extend `State` struct with `context_menu` field |
| ~30-35 | Add `ContextMenuState` struct definition |
| 31-36 | Extend `FileAction` enum with new actions |
| 40-62 | Update `update()` function with delete logic and context menu handling |
| 97-100 | Enhance right-click handler to capture position |
| ~150-200 | New function: `view_context_menu()` (~50 lines) |
| ~200-220 | New function: `build_menu_items()` (~20 lines) |
| 64-93 | Integrate menu overlay in `view()` function |

**No other files need modification** for the initial implementation.

## Advantages of This Approach

- ✅ Minimal changes to existing code (right-click already wired)
- ✅ Follows existing Iced patterns in the codebase
- ✅ Extensible for future menu items
- ✅ Assets (SVG icons) already embedded and ready
- ✅ Clean separation: state → update → view
- ✅ Type-safe with Rust's enum-based message passing
- ✅ No external dependencies needed

## Testing Strategy

### Manual Testing Checklist

- [ ] Right-click on file shows context menu
- [ ] Right-click on directory shows context menu
- [ ] Context menu appears at cursor position
- [ ] Click outside context menu dismisses it
- [ ] ESC key dismisses context menu
- [ ] Clicking "Delete" on file deletes the file
- [ ] Clicking "Delete" on directory deletes the directory
- [ ] Deleted items disappear from UI
- [ ] Deleting selected item clears selection
- [ ] Error logging works for failed deletions
- [ ] Menu has correct styling (border, background, hover effects)

### Edge Cases to Consider

- Deleting currently expanded directory
- Deleting parent of currently selected item
- Permission denied scenarios
- Very long filenames in menu
- Context menu near screen edges
- Rapid right-clicking
- Right-click while another context menu is open

## Future Enhancements

### Phase 5+ (Post-Initial Implementation)

1. **Additional menu items**:
   - Rename (with inline editing)
   - Copy/Cut/Paste
   - New File/New Folder
   - Properties/Info

2. **Confirmation dialogs**:
   - "Are you sure?" modal for delete operations
   - Especially important for non-empty directories

3. **Keyboard shortcuts**:
   - Delete key for selected items
   - Ctrl+C, Ctrl+V for copy/paste
   - F2 for rename

4. **Multi-selection support**:
   - Context menu on multiple selected items
   - Bulk delete operation

5. **Smart positioning**:
   - Detect screen edges and flip menu position
   - Sub-menus for nested options

6. **Undo functionality**:
   - Move to trash instead of permanent delete
   - Undo stack for recent operations

7. **Platform integration**:
   - Native file operations
   - Integration with system trash/recycle bin

## Implementation Status

✅ **COMPLETED** - All phases 1-3 implemented and tested

### Key Implementation Details

**Cursor Position Tracking:**
- Global cursor position is tracked in `State.cursor_position`
- Entire view is wrapped in `widget::mouse_area` with `on_move(FileAction::CursorMoved)`
- Context menu uses the tracked cursor position when opened
- This ensures the menu appears exactly where you right-click

**Dismiss Layer Fix:**
- Context menu no longer uses full-screen container with padding
- Uses `widget::column` and `widget::row` with `Space` widgets for positioning
- This allows the dismiss layer underneath to properly receive clicks
- Clicking anywhere outside the menu now correctly closes it

## Implementation Timeline Estimate

- **Phase 1** (State & Positioning): ✅ Complete
- **Phase 2** (UI Component): ✅ Complete
- **Phase 3** (Delete Functionality): ✅ Complete
- **Phase 4** (Conditional Items): Pending (ready to implement)

## References

- **Iced Documentation**: https://docs.rs/iced/
- **Iced Examples**: https://github.com/iced-rs/iced/tree/master/examples
- **Current Implementation**: `src/explorer.rs:97` (right-click handler)
- **Assets Location**: `src/assets.rs`, `assets/` directory

## Notes

- The existing codebase already has excellent logging infrastructure (`log` crate)
- All necessary SVG icons are already embedded
- The functional reactive pattern makes this implementation very clean
- Consider adding `#[allow(dead_code)]` if new structs trigger warnings initially
