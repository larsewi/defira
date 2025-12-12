use crate::assets;
use crate::context_menu;
use iced::widget;
use iced::{Element, Length};
use log::{debug, error, trace};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum FileAction {
    Select(PathBuf),
    ContextMenu(PathBuf),
    CloseContextMenu,
    EditItem(PathBuf),
    DeleteItem(PathBuf),
    CursorMoved(iced::Point),
}

#[derive(Debug, Clone)]
pub struct ContextMenuState {
    pub target_path: PathBuf,
    pub position: iced::Point,
}

pub struct State {
    expanded: HashSet<PathBuf>,
    selected: HashSet<PathBuf>,
    context_menu: Option<ContextMenuState>,
    cursor_position: iced::Point,
}

impl Default for State {
    fn default() -> Self {
        Self {
            expanded: HashSet::new(),
            selected: HashSet::new(),
            context_menu: None,
            cursor_position: iced::Point::ORIGIN,
        }
    }
}

pub fn update(state: &mut State, action: FileAction) {
    match action {
        FileAction::Select(path) => {
            if path.is_dir() {
                if state.expanded.contains(&path) {
                    debug!("Directory '{}' is collapsed", path.display());
                    state.expanded.remove(&path);
                } else {
                    debug!("Directory '{}' is expanded", path.display());
                    state.expanded.insert(path.clone());
                }
            }

            if !state.selected.contains(&path) {
                debug!("Path '{}' is selected", path.display());
                state.selected.clear();
                state.selected.insert(path);
            }

            // Close context menu on any selection
            state.context_menu = None;
        }
        FileAction::ContextMenu(path) => {
            debug!(
                "Context menu opened for {} '{}' at position ({}, {})",
                if path.is_dir() { "directory" } else { "secret" },
                path.display(),
                state.cursor_position.x,
                state.cursor_position.y
            );
            state.context_menu = Some(ContextMenuState {
                target_path: path,
                position: state.cursor_position,
            });
        }
        FileAction::CloseContextMenu => {
            debug!("Context menu closed");
            state.context_menu = None;
        }
        FileAction::EditItem(path) => {
            debug!("Edit secret: {}", path.display());
        }
        FileAction::DeleteItem(path) => {
            if path.is_dir() {
                debug!("Deleting directory: {}", path.display());
            } else {
                debug!("Deleting secret: {}", path.display());
            };

            // Remove from selected set if it was selected
            state.selected.remove(&path);
            // Remove from expanded set if it was expanded
            state.expanded.remove(&path);
            // Close context menu
            state.context_menu = None;
        }
        FileAction::CursorMoved(position) => {
            state.cursor_position = position;
        }
    }
}

fn create_row<'a>(
    path: PathBuf,
    indent_width: u16,
    indent_level: u16,
    is_directory: bool,
    is_selected: bool,
) -> Element<'a, FileAction> {
    let indent = widget::Space::with_width(indent_width * indent_level);
    let asset = widget::svg::Handle::from_memory(if is_directory {
        assets::FOLDER_LOGO
    } else {
        assets::SECRET_LOGO
    });

    let filename = path.file_name().unwrap_or_default();
    let icon = widget::svg(asset).width(20);
    let space = widget::Space::with_width(10);
    let text = widget::text!["{}", filename.display()].width(Length::Fill);
    let row = widget::row![indent, icon, space, text]
        .align_y(iced::Alignment::Center)
        .width(Length::Fill);

    let button = widget::button(row)
        .on_press(FileAction::Select(path.clone()))
        .style(move |theme: &iced::Theme, status| {
            let selected_bg = iced::Background::Color(iced::Color::from_rgba(0.3, 0.5, 0.8, 0.5));
            let hover_bg = iced::Background::Color(iced::Color::from_rgba(0.3, 0.5, 0.8, 0.3));

            let base = widget::button::Style {
                background: if is_selected { Some(selected_bg) } else { None },
                text_color: theme.palette().text,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            };
            match status {
                widget::button::Status::Hovered => widget::button::Style {
                    background: Some(hover_bg),
                    ..base
                },
                _ => base,
            }
        })
        .width(Length::Fill);

    widget::mouse_area(button)
        .on_right_press(FileAction::ContextMenu(path))
        .into()
}

fn render_directory_contents(
    path: &std::path::Path,
    state: &State,
    indent_level: u16,
    indent_width: u16,
    buttons: &mut Vec<Element<FileAction>>,
) {
    let entries = match fs::read_dir(path) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to read directory '{}': {}", path.display(), e);
            return;
        }
    };

    for entry in entries {
        let entry_path = match entry {
            Ok(v) => v.path(),
            Err(e) => {
                error!(
                    "Failed to get entry from directory '{}': {}",
                    path.display(),
                    e
                );
                continue;
            }
        };

        if let Some(filename) = entry_path.file_name() {
            trace!(
                "Creating row for {} {} at indent level {}",
                if entry_path.is_dir() {
                    "directory"
                } else {
                    "file"
                },
                filename.display(),
                indent_level
            );

            let is_expanded = state.expanded.contains(&entry_path);
            let is_selected = state.selected.contains(&entry_path);
            let row = create_row(
                entry_path.clone(),
                indent_width,
                indent_level,
                entry_path.is_dir(),
                is_selected,
            );
            buttons.push(row);

            if entry_path.is_dir() {
                // If directory is expanded, recursively render its contents
                if is_expanded {
                    render_directory_contents(
                        &entry_path,
                        state,
                        indent_level + 1,
                        indent_width,
                        buttons,
                    );
                }
            }
        }
    }
}

pub fn view(state: &State) -> Element<'_, FileAction> {
    const INDENT_LEVEL: u16 = 0;
    const INDENT_WIDTH: u16 = 24;
    const CONTENT_PADDING: u16 = 10;

    let home = std::env::var("HOME").unwrap_or(".".to_string());
    let dir = std::path::PathBuf::from(format!("{}/ntech/mystiko", home));
    let mut buttons: Vec<Element<FileAction>> = Vec::new();

    render_directory_contents(&dir, state, INDENT_LEVEL, INDENT_WIDTH, &mut buttons);

    let file_list = widget::Column::from_vec(buttons).width(Length::Fill);
    let scrollable_list = widget::scrollable(file_list);
    let main_content = widget::container(scrollable_list)
        .padding(CONTENT_PADDING)
        .width(Length::Fill);

    // If context menu is open, render it on top
    let content: Element<'_, FileAction> = if let Some(menu_state) = &state.context_menu {
        // Build menu items for file explorer context
        let menu_items = vec![
            context_menu::MenuItem::new(
                "Edit",
                FileAction::EditItem(menu_state.target_path.clone()),
            )
            .with_icon(assets::EDIT_LOGO),
            context_menu::MenuItem::new(
                "Delete",
                FileAction::DeleteItem(menu_state.target_path.clone()),
            )
            .with_icon(assets::DELETE_LOGO),
        ];

        // Create dismiss layer and menu using generic context_menu module
        let dismiss_layer = context_menu::create_dismiss_layer(FileAction::CloseContextMenu);
        let menu = context_menu::view_context_menu(&menu_state.position, menu_items);

        // Stack: main content, dismiss layer, context menu
        widget::Stack::new()
            .push(main_content)
            .push(dismiss_layer)
            .push(menu)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    } else {
        main_content.into()
    };

    // Wrap everything in a mouse_area to track cursor position
    widget::mouse_area(content)
        .on_move(FileAction::CursorMoved)
        .into()
}
