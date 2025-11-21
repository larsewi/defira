# Context Menu Refactoring Plan

## Overview

Extract the context menu implementation from `src/explorer.rs` into a separate, reusable `src/context_menu.rs` module to enable context menu functionality throughout the application.

**Core Design Principle**: The context menu is a pure UI component that knows **nothing** about what it's displaying a menu for. It doesn't know about files, directories, paths, text selections, shapes, or any other domain concepts. It only knows:
- Where to render (position)
- What to render (menu items)
- How to render (styling)

Each module using the context menu maintains its own state about what was right-clicked, selected, or focused.

## Current State

The context menu code is currently embedded in `src/explorer.rs` with the following components:

- **ContextMenuState struct** (lines 17-23): Stores target path, directory flag, and position
- **FileAction enum variants** (lines 11-14): `ContextMenu`, `CloseContextMenu`, `DeleteItem`, `CursorMoved`
- **view_context_menu function** (lines 230-287): Renders the context menu UI
- **update function handlers** (lines 65-111): Handles context menu actions
- **Integration in view function** (lines 304-320): Stacks dismiss layer and menu over main content

## Goals

1. **Reusability**: Make context menu usable in any part of the application for any purpose
2. **Complete genericity**: Context menu knows nothing about files, paths, or any specific domain
3. **Pure UI component**: Only handles rendering and positioning
4. **Flexibility**: Support different menu items for different contexts
5. **Maintainability**: Single source of truth for context menu UI behavior

## Refactoring Strategy

### Phase 1: Create New Module Structure

**File**: `src/context_menu.rs`

#### 1.1 Create Generic MenuItem Structure

**Important**: Do NOT extract `ContextMenuState` - it contains domain-specific data (target_path, is_directory) that belongs in the calling module, not the generic context menu.

#### 1.2 Define MenuItem Structure

```rust
pub struct MenuItem<'a, Message> {
    pub label: String,
    pub icon: Option<&'a [u8]>,
    pub action: Message,
}
```

This allows any module to create custom menu items with their own message types. The context menu knows nothing about what the menu items represent - it just renders them.

#### 1.3 Create Pure Rendering Function

Create a generic `view_context_menu` function:

```rust
pub fn view_context_menu<'a, Message>(
    position: &'a iced::Point,
    items: Vec<MenuItem<Message>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a
```

**Changes from current implementation**:
- Accept position as reference (&iced::Point) instead of domain-specific state
- Position reference allows Element lifetime to be non-static
- Accept menu items as parameter for complete flexibility
- Generic over `Message` type instead of hardcoded `FileAction`
- No knowledge of files, directories, paths, or any domain concepts
- Pure UI component - only knows how to render a menu

#### 1.4 Extract Dismiss Layer Helper

```rust
pub fn create_dismiss_layer<Message>(
    on_dismiss: Message
) -> Element<'static, Message>
```

This helper creates the click-outside-to-dismiss layer that can be reused.

### Phase 2: Update explorer.rs

#### 2.1 Add Module Import

If modules are declared in main.rs:
```rust
use crate::context_menu::{MenuItem, view_context_menu, create_dismiss_layer};
```

**Important**: Do NOT import `ContextMenuState` from context_menu - it won't exist there. Keep the existing `ContextMenuState` in explorer.rs since it contains file-specific data.

#### 2.2 Update ContextMenuState (Keep in explorer.rs)

The `ContextMenuState` struct stays in `explorer.rs` because it contains domain-specific information about what was right-clicked. It's not part of the generic context menu.

Keep as-is:
```rust
#[derive(Debug, Clone)]
pub struct ContextMenuState {
    pub target_path: String,
    pub is_directory: bool,
    pub position: iced::Point,
}
```

#### 2.3 Remove Only Extracted Code

Delete from `explorer.rs`:
- Lines 230-287: `view_context_menu` function ONLY

Keep in `explorer.rs`:
- Lines 17-23: `ContextMenuState` struct (file-specific state)

#### 2.4 Update view Function

Replace the current context menu rendering (lines 304-320) with:

```rust
let content: Element<'_, FileAction> = if let Some(menu_state) = &state.context_menu {
    let menu_items = vec![
        MenuItem::new("Delete", FileAction::DeleteItem(menu_state.target_path.clone()))
            .with_icon(assets::DELETE_LOGO),
    ];

    let dismiss_layer = create_dismiss_layer(FileAction::CloseContextMenu);
    let menu = view_context_menu(&menu_state.position, menu_items);

    widget::Stack::new()
        .push(main_content)
        .push(dismiss_layer)
        .push(menu)
        .into()
} else {
    main_content.into()
};
```

### Phase 3: Module Registration

Update `src/main.rs` to declare the new module:

```rust
mod assets;
mod context_menu;  // NEW
mod explorer;
```

## API Design

### Public API of context_menu module

The context_menu module is a pure UI component with no domain-specific state. It provides only generic rendering utilities.

```rust
// Menu item builder
pub struct MenuItem<Message> {
    pub label: String,
    pub icon: Option<&'static [u8]>,
    pub action: Message,
}

impl<Message> MenuItem<Message> {
    pub fn new(label: impl Into<String>, action: Message) -> Self;
    pub fn with_icon(mut self, icon: &'static [u8]) -> Self;
}

// Rendering functions
pub fn view_context_menu<'a, Message>(
    position: &'a iced::Point,
    items: Vec<MenuItem<Message>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a;

pub fn create_dismiss_layer<Message>(
    on_dismiss: Message
) -> Element<'static, Message>
where
    Message: Clone + 'static;
```

## Benefits of This Design

1. **Completely generic**: Works for files, text selections, canvas objects, or anything else
2. **Zero domain knowledge**: Context menu knows nothing about what it's displaying a menu for
3. **Pure UI component**: Only responsible for rendering and positioning
4. **Flexible lifetimes**: Returns Element<'a, Message> not Element<'static, Message>
5. **Generic over Message types**: Any module can use it with their own action enum
6. **Flexible menu items**: Easily add/remove items, icons are optional
7. **Calling code owns context**: Explorer manages file-specific state, editor manages text selection state, etc.
8. **Easy to extend**: Add new menu items without touching core rendering logic
9. **Consistent styling**: All menus in the app will look the same

## Example Usage in Other Modules

### Example 1: Text Editor Context Menu

```rust
use crate::context_menu::{MenuItem, view_context_menu, create_dismiss_layer};

// Text editor manages its own state about selected text
struct EditorState {
    selected_text: Option<String>,
    selection_start: usize,
    selection_end: usize,
    menu_position: Option<iced::Point>,
}

enum EditorAction {
    Cut,
    Copy,
    Paste,
    CloseMenu,
}

fn view_editor_menu(position: &iced::Point) -> Element<'_, EditorAction> {
    let items = vec![
        MenuItem::new("Cut", EditorAction::Cut).with_icon(assets::CUT_ICON),
        MenuItem::new("Copy", EditorAction::Copy).with_icon(assets::COPY_ICON),
        MenuItem::new("Paste", EditorAction::Paste).with_icon(assets::PASTE_ICON),
    ];

    view_context_menu(position, items)
}
```

### Example 2: Canvas/Drawing Application

```rust
use crate::context_menu::{MenuItem, view_context_menu};

// Canvas manages its own state about selected shape
struct CanvasState {
    selected_shape_id: Option<usize>,
    shapes: Vec<Shape>,
    menu_position: Option<iced::Point>,
}

enum CanvasAction {
    BringToFront,
    SendToBack,
    Duplicate,
    Delete,
    CloseMenu,
}

fn view_canvas_menu(position: &iced::Point) -> Element<'_, CanvasAction> {
    let items = vec![
        MenuItem::new("Bring to Front", CanvasAction::BringToFront),
        MenuItem::new("Send to Back", CanvasAction::SendToBack),
        MenuItem::new("Duplicate", CanvasAction::Duplicate),
        MenuItem::new("Delete", CanvasAction::Delete).with_icon(assets::DELETE_ICON),
    ];

    view_context_menu(position, items)
}
```

### Example 3: List/Table Context Menu

```rust
use crate::context_menu::{MenuItem, view_context_menu};

// List widget manages its own state about selected row
struct ListState<T> {
    selected_row: Option<usize>,
    items: Vec<T>,
    menu_position: Option<iced::Point>,
}

enum ListAction {
    Edit,
    Delete,
    MoveUp,
    MoveDown,
    CloseMenu,
}

fn view_list_menu(position: &iced::Point) -> Element<'_, ListAction> {
    let items = vec![
        MenuItem::new("Edit", ListAction::Edit).with_icon(assets::EDIT_ICON),
        MenuItem::new("Delete", ListAction::Delete).with_icon(assets::DELETE_ICON),
        MenuItem::new("Move Up", ListAction::MoveUp),
        MenuItem::new("Move Down", ListAction::MoveDown),
    ];

    view_context_menu(position, items)
}
```

**Key Pattern**: Each module maintains its own state structure containing:
- Domain-specific data (selected text, shape ID, file path, etc.)
- Menu position (when menu should be shown)
- The module creates MenuItems with its own Message type
- The generic context_menu just renders what it's given

## Testing Checklist

After refactoring, verify:

- [ ] Context menu still appears on right-click in explorer
- [ ] Delete button still works correctly
- [ ] Click-outside-to-dismiss still works
- [ ] Menu positioning is correct
- [ ] Styling (borders, shadows, hover effects) unchanged
- [ ] No compilation errors
- [ ] No clippy warnings

## Implementation Steps

1. Create `src/context_menu.rs` with extracted code
2. Make code generic over Message type
3. Update `src/main.rs` to declare new module
4. Update `src/explorer.rs` to use the new module
5. Remove duplicated code from `src/explorer.rs`
6. Test all functionality
7. Run cargo fmt and cargo clippy

## Files Modified

- **NEW**: `src/context_menu.rs` (~120 lines) - Pure UI component, no domain logic
- **MODIFIED**: `src/explorer.rs` (reduced by ~60 lines) - Keeps ContextMenuState, removes rendering
- **MODIFIED**: `src/main.rs` (add 1 line) - Module declaration

## Responsibility Split

### context_menu module (Pure UI - Domain Agnostic)
**Responsibilities:**
- Render menu at given position
- Style menu items (borders, shadows, hover effects)
- Provide dismiss layer functionality
- Handle no state, only render what it's told

**Does NOT know about:**
- Files, directories, paths
- What was right-clicked
- What actions mean
- Domain logic of any kind

### Calling modules (Domain Logic - Context Aware)
**Responsibilities:**
- Track what was right-clicked/selected
- Store domain-specific state (file path, text range, object ID, etc.)
- Decide which menu items to show
- Define what actions mean
- Handle action execution

**Example for explorer.rs:**
- Tracks `target_path` and `is_directory`
- Decides to show "Delete" menu item
- Executes file deletion when action is triggered

## Backwards Compatibility

This is a pure refactoring with no behavior changes. All existing functionality remains identical from a user perspective.
