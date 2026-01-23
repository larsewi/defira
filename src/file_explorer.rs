use crate::assets;
use crate::context_menu;
use crate::error_popup;
use crate::password_prompt;
use iced::widget;
use iced::widget::text_editor;
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
    CloseEditor,
    EditorAction(text_editor::Action),
    PasswordPrompt(password_prompt::Message),
    ErrorPopup(error_popup::Message),
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
    opened_file: Option<PathBuf>,
    editor_content: Option<text_editor::Content>,
    password_prompt: Option<password_prompt::State>,
    error_popup: Option<error_popup::State>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            expanded: HashSet::new(),
            selected: HashSet::new(),
            context_menu: None,
            cursor_position: iced::Point::ORIGIN,
            opened_file: None,
            editor_content: None,
            password_prompt: None,
            error_popup: None,
        }
    }
}

fn open_file_in_editor(state: &mut State, path: &PathBuf) {
    debug!("Opening file in editor: {}", path.display());

    // This file needs to be decrypted - open password prompt
    if path.extension().is_some_and(|ext| ext == "gpg") {
        debug!("File is encrypted, opening password prompt");
        state.password_prompt = Some(password_prompt::State::new(path.clone()));
        return;
    }

    // Otherwise, this is a regular file
    match fs::read_to_string(path) {
        Ok(content) => {
            state.opened_file = Some(path.clone());
            state.editor_content = Some(text_editor::Content::with_text(&content));
        }
        Err(e) => {
            error!("Failed to read file '{}': {}", path.display(), e);
            state.opened_file = None;
            state.editor_content = None;
            state.error_popup = Some(error_popup::State::new(
                "Error",
                format!("Failed to read file: {}", e),
            ));
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
            } else {
                debug!("Opened '{}' in editor", path.display());
                open_file_in_editor(state, &path);
            }

            if !state.selected.contains(&path) {
                debug!("Path '{}' is selected", path.display());
                state.selected.clear();
                state.selected.insert(path);
            }
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
            open_file_in_editor(state, &path);
            state.context_menu = None;
        }
        FileAction::CloseEditor => {
            debug!("Closing editor");
            state.opened_file = None;
            state.editor_content = None;
        }
        FileAction::EditorAction(action) => {
            if let Some(ref mut content) = state.editor_content {
                content.perform(action);
            }
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
        FileAction::PasswordPrompt(msg) => {
            match msg {
                password_prompt::Message::PasswordChanged(password) => {
                    if let Some(ref mut prompt) = state.password_prompt {
                        prompt.password = password;
                    }
                }
                password_prompt::Message::ToggleVisibility => {
                    if let Some(ref mut prompt) = state.password_prompt {
                        prompt.show_password = !prompt.show_password;
                    }
                }
                password_prompt::Message::Submit => {
                    if let Some(prompt) = state.password_prompt.take() {
                        let path = prompt.target_path;
                        debug!("Password submitted for file '{}'", path.display());
                        // TODO: Use password to decrypt the file
                        // For now, just store it so the caller can access it
                        // The decryption logic will be added here later
                    }
                }
                password_prompt::Message::Cancel => {
                    debug!("Password prompt cancelled");
                    state.password_prompt = None;
                }
            }
        }
        FileAction::ErrorPopup(msg) => match msg {
            error_popup::Message::Dismiss => {
                debug!("Error popup dismissed");
                state.error_popup = None;
            }
        },
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

    let asset = if is_directory {
        assets::FOLDER_LOGO
    } else if path.extension().is_some_and(|ext| ext == "gpg") {
        assets::SECRET_LOGO
    } else {
        assets::FILE_LOGO
    };
    let asset = widget::svg::Handle::from_memory(asset);

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
            // Skip hidden files (starting with .)
            if filename.to_string_lossy().starts_with('.') {
                continue;
            }

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

fn view_editor_panel(state: &State) -> Element<'_, FileAction> {
    const CONTENT_PADDING: u16 = 10;

    if let Some(file) = &state.opened_file {
        let filename = file
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        // Header with filename and close button
        let title = widget::text(filename).size(16);
        let close_button =
            widget::button(widget::text("X").size(14)).on_press(FileAction::CloseEditor);

        let header = widget::row![title, widget::horizontal_space(), close_button]
            .align_y(iced::Alignment::Center);
        let header = widget::container(header).width(Length::Fill);

        if let Some(content) = &state.editor_content {
            let editor = widget::text_editor(&content)
                .on_action(FileAction::EditorAction)
                .height(Length::Fill);
            let editor = widget::container(editor).padding(CONTENT_PADDING);

            return widget::column![header, editor]
                .height(Length::Fill)
                .width(Length::Fill)
                .into();
        }
    }

    let placeholder = widget::text("Select a file to view its contents...");
    widget::container(placeholder)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

pub fn view(state: &State) -> Element<'_, FileAction> {
    const INDENT_LEVEL: u16 = 0;
    const INDENT_WIDTH: u16 = 24;
    const CONTENT_PADDING: u16 = 10;
    const EXPLORER_FILL_PORTION: u16 = 1;
    const EDITOR_FILL_PORTION: u16 = 2;

    let home = std::env::var("HOME").unwrap_or(".".to_string());
    let dir = std::path::PathBuf::from(format!("{}/ntech/mystiko", home));
    let mut buttons: Vec<Element<FileAction>> = Vec::new();

    render_directory_contents(&dir, state, INDENT_LEVEL, INDENT_WIDTH, &mut buttons);

    let file_list = widget::Column::from_vec(buttons).width(Length::Fill);
    let scrollable_list = widget::scrollable(file_list);
    let file_explorer_panel = widget::container(scrollable_list)
        .padding(CONTENT_PADDING)
        .width(Length::FillPortion(EXPLORER_FILL_PORTION));

    // Editor panel on the right
    let editor_panel = widget::container(view_editor_panel(state))
        .width(Length::FillPortion(EDITOR_FILL_PORTION))
        .height(Length::Fill);

    // Split layout: file explorer left, editor right
    let split_layout = widget::row![file_explorer_panel, editor_panel]
        .height(Length::Fill)
        .width(Length::Fill);

    // If error popup is open, render it on top (highest priority)
    let content: Element<'_, FileAction> = if let Some(error_state) = &state.error_popup {
        let backdrop =
            error_popup::create_backdrop(FileAction::ErrorPopup(error_popup::Message::Dismiss));
        let modal = error_popup::view(error_state, FileAction::ErrorPopup);

        widget::Stack::new()
            .push(split_layout)
            .push(backdrop)
            .push(modal)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    } else if let Some(prompt_state) = &state.password_prompt {
        // Password prompt has second highest priority
        let backdrop = password_prompt::create_backdrop(FileAction::PasswordPrompt(
            password_prompt::Message::Cancel,
        ));
        let modal = password_prompt::view(prompt_state, FileAction::PasswordPrompt);

        widget::Stack::new()
            .push(split_layout)
            .push(backdrop)
            .push(modal)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    } else if let Some(menu_state) = &state.context_menu {
        // If context menu is open, render it on top
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
            .push(split_layout)
            .push(dismiss_layer)
            .push(menu)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    } else {
        split_layout.into()
    };

    // Wrap everything in a mouse_area to track cursor position
    widget::mouse_area(content)
        .on_move(FileAction::CursorMoved)
        .into()
}
