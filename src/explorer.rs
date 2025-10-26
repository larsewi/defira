use crate::assets;
use iced::widget;
use iced::{Element, Length};
use log::{debug, error, trace};
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
pub enum FileAction {
    Select(String, bool),
    ContextMenu(String, bool),
    CloseContextMenu,
    DeleteItem(String),
    CursorMoved(iced::Point),
}

#[derive(Debug, Clone)]
pub struct ContextMenuState {
    pub target_path: String,
    #[allow(dead_code)]
    pub is_directory: bool,
    pub position: iced::Point,
}

pub struct State {
    expanded: HashSet<String>,
    selected: HashSet<String>,
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
        FileAction::Select(path, is_dir) => {
            if is_dir {
                if state.expanded.contains(&path) {
                    debug!("Directory '{}' is expanded", path);
                    state.expanded.remove(&path);
                } else {
                    debug!("Directory '{}' is collapsed", path);
                    state.expanded.insert(path.clone());
                }
            }

            if !state.selected.contains(&path) {
                debug!("Path '{}' is selected", path);
                state.selected.clear();
                state.selected.insert(path);
            }

            // Close context menu on any selection
            state.context_menu = None;
        }
        FileAction::ContextMenu(path, is_dir) => {
            debug!(
                "Context menu opened for {} '{}' at position ({}, {})",
                if is_dir { "directory" } else { "secret" },
                path,
                state.cursor_position.x,
                state.cursor_position.y
            );
            state.context_menu = Some(ContextMenuState {
                target_path: path,
                is_directory: is_dir,
                position: state.cursor_position,
            });
        }
        FileAction::CloseContextMenu => {
            debug!("Context menu closed");
            state.context_menu = None;
        }
        FileAction::DeleteItem(path) => {
            debug!("Deleting: {}", path);
            let path_obj = std::path::Path::new(&path);
            let result = if path_obj.is_dir() {
                fs::remove_dir_all(&path)
            } else {
                fs::remove_file(&path)
            };

            match result {
                Ok(_) => {
                    debug!("Successfully deleted: {}", path);
                    // Remove from selected set if it was selected
                    state.selected.remove(&path);
                    // Remove from expanded set if it was expanded
                    state.expanded.remove(&path);
                }
                Err(e) => {
                    error!("Failed to delete '{}': {}", path, e);
                }
            }

            // Close context menu
            state.context_menu = None;
        }
        FileAction::CursorMoved(position) => {
            state.cursor_position = position;
        }
    }
}

fn create_row<'a>(
    filename: &str,
    full_path: &str,
    indent_width: u16,
    is_directory: bool,
    indent_level: u16,
) -> Element<'a, FileAction> {
    let indent = widget::Space::with_width(indent_width * indent_level);
    let asset = widget::svg::Handle::from_memory(if is_directory {
        assets::FOLDER_LOGO
    } else {
        assets::SECRET_LOGO
    });
    let icon = widget::svg(asset).width(20);
    let space = widget::Space::with_width(10);
    let text = widget::text!["{}", filename].width(Length::Fill);
    let row = widget::row![indent, icon, space, text]
        .align_y(iced::Alignment::Center)
        .width(Length::Fill);

    let button = widget::button(row)
        .on_press(FileAction::Select(full_path.to_string(), is_directory))
        .style(|theme: &iced::Theme, status| {
            let base = widget::button::Style {
                background: None,
                text_color: theme.palette().text,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            };
            match status {
                widget::button::Status::Hovered => widget::button::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(
                        0.3, 0.5, 0.8, 0.3,
                    ))),
                    ..base
                },
                _ => base,
            }
        })
        .width(Length::Fill);

    widget::mouse_area(button)
        .on_right_press(FileAction::ContextMenu(full_path.to_string(), is_directory))
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

        let full_path = entry_path.display().to_string();

        if let Some(filename) = entry_path.file_name() {
            let filename = filename.display().to_string();

            trace!(
                "Creating row for {} {} at indent level {}",
                if entry_path.is_dir() {
                    "directory"
                } else {
                    "file"
                },
                filename,
                indent_level
            );

            let is_expanded = state.expanded.contains(&full_path);
            let row = create_row(
                &filename,
                &full_path,
                indent_width,
                entry_path.is_dir(),
                indent_level,
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

fn view_context_menu(menu_state: &ContextMenuState) -> Element<'_, FileAction> {
    // Create the delete button with icon
    let delete_icon = widget::svg(widget::svg::Handle::from_memory(assets::DELETE_LOGO)).width(16);
    let delete_text = widget::text("Delete").size(14);
    let delete_row = widget::row![delete_icon, widget::Space::with_width(8), delete_text]
        .align_y(iced::Alignment::Center)
        .padding(8);

    let delete_button = widget::button(delete_row)
        .on_press(FileAction::DeleteItem(menu_state.target_path.clone()))
        .style(|theme: &iced::Theme, status| {
            let base = widget::button::Style {
                background: Some(iced::Background::Color(iced::Color::WHITE)),
                text_color: theme.palette().text,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            };
            match status {
                widget::button::Status::Hovered => widget::button::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(
                        0.3, 0.5, 0.8, 0.3,
                    ))),
                    ..base
                },
                _ => base,
            }
        })
        .width(Length::Fill);

    // Create the menu container
    let menu = widget::container(widget::column![delete_button])
        .padding(4)
        .style(|theme: &iced::Theme| widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::WHITE)),
            text_color: Some(theme.palette().text),
            border: iced::Border {
                color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                offset: iced::Vector::new(2.0, 2.0),
                blur_radius: 8.0,
            },
        })
        .width(150);

    // Position the menu using Space widgets so it doesn't block clicks
    widget::column![
        widget::Space::with_height(menu_state.position.y),
        widget::row![
            widget::Space::with_width(menu_state.position.x),
            menu,
        ],
    ]
    .into()
}

pub fn view(state: &State) -> Element<'_, FileAction> {
    const INDENT_LEVEL: u16 = 0;
    let dir = std::path::Path::new(".");
    let indent_width = 24;
    let mut buttons: Vec<Element<FileAction>> = Vec::new();

    render_directory_contents(&dir, state, INDENT_LEVEL, indent_width, &mut buttons);

    let file_list = widget::Column::from_vec(buttons).width(Length::Fill);
    let scrollable_list = widget::scrollable(file_list);
    let main_content = widget::container(scrollable_list)
        .padding(10)
        .width(Length::Fill);

    // If context menu is open, render it on top
    let content: Element<'_, FileAction> = if let Some(menu_state) = &state.context_menu {
        // Create a click-outside-to-dismiss layer
        let dismiss_layer = widget::mouse_area(widget::container(widget::Space::new(
            Length::Fill,
            Length::Fill,
        )))
        .on_press(FileAction::CloseContextMenu);

        // Stack: main content, dismiss layer, context menu
        widget::Stack::new()
            .push(main_content)
            .push(dismiss_layer)
            .push(view_context_menu(menu_state))
            .into()
    } else {
        main_content.into()
    };

    // Wrap everything in a mouse_area to track cursor position
    widget::mouse_area(content)
        .on_move(FileAction::CursorMoved)
        .into()
}
