use crate::assets;
use iced::widget::*;
use iced::{Element, Length};
use log::{debug, error, trace};
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
pub enum FileAction {
    Edit(String),
    Delete(String),
    Clipboard(String),
    DirectoryToggle(String),
    AddUser(String),
    NewFile(String),
}

pub struct State {
    expanded_directories: HashSet<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            expanded_directories: HashSet::new(),
        }
    }
}

pub fn update(state: &mut State, action: FileAction) {
    match action {
        FileAction::Edit(path) => debug!("Edit clicked for path '{}'", path),
        FileAction::Delete(path) => debug!("Delete clicked for path '{}'", path),
        FileAction::Clipboard(path) => {
            debug!("Clipboard clicked for path '{}'", path)
        }
        FileAction::DirectoryToggle(path) => {
            debug!("Directory toggle clicked for: '{}'", path);
            if state.expanded_directories.contains(&path) {
                debug!("Directory '{}' is collapsed", path);
                state.expanded_directories.remove(&path);
            } else {
                debug!("Directory '{}' is expanded", path);
                state.expanded_directories.insert(path);
            }
        }
        FileAction::AddUser(path) => debug!("Add user clicked for path '{}'", path),
        FileAction::NewFile(path) => debug!("New file clicked for path '{}'", path),
    }
}

fn create_svg_button(
    svg_data: &'static [u8],
    action: FileAction,
    size: u16,
) -> button::Button<'static, FileAction> {
    let icon = svg(svg::Handle::from_memory(svg_data))
        .width(size)
        .height(size);
    button(icon)
        .on_press(action)
        .height(size)
        .width(size)
        .style(button::text)
}

fn create_directory_row(
    filename: &str,
    full_path: &str,
    button_size: u16,
    is_expanded: bool,
    indent_level: u16,
) -> Element<'static, FileAction> {
    let chevron = if is_expanded {
        assets::CHEVRON_DOWN_LOGO
    } else {
        assets::CHEVRON_RIGHT_LOGO
    };
    let chevron = create_svg_button(
        chevron,
        FileAction::DirectoryToggle(full_path.to_string()),
        button_size,
    );
    let filename_text = text(filename.to_string()).width(Length::Fill);
    let new_file = create_svg_button(
        assets::NEW_FILE_LOGO,
        FileAction::NewFile(full_path.to_string()),
        button_size,
    );
    let add_user = create_svg_button(
        assets::ADD_USER_LOGO,
        FileAction::AddUser(full_path.to_string()),
        button_size,
    );
    let delete = create_svg_button(
        assets::DELETE_LOGO,
        FileAction::Delete(full_path.to_string()),
        button_size,
    );

    let indent = Space::with_width(button_size * indent_level);
    row![indent, chevron, filename_text, new_file, add_user, delete]
        .align_y(iced::Alignment::Center)
        .width(Length::Fill)
        .into()
}

fn create_file_row(
    filename: &str,
    full_path: &str,
    button_size: u16,
    indent_level: u16,
) -> Element<'static, FileAction> {
    let text = text(filename.to_string()).width(Length::Fill);
    let clipboard = create_svg_button(
        assets::CLIPBOARD_LOGO,
        FileAction::Clipboard(full_path.to_string()),
        button_size,
    );
    let edit = create_svg_button(
        assets::EDIT_LOGO,
        FileAction::Edit(full_path.to_string()),
        button_size,
    );
    let delete = create_svg_button(
        assets::DELETE_LOGO,
        FileAction::Delete(full_path.to_string()),
        button_size,
    );

    // Files are indented one level more than directories (to account for chevron button)
    let indent = Space::with_width(button_size * (indent_level + 1));
    row![indent, text, clipboard, edit, delete]
        .align_y(iced::Alignment::Center)
        .width(Length::Fill)
        .into()
}

fn render_directory_contents(
    path: &std::path::Path,
    state: &State,
    indent_level: u16,
    button_size: u16,
    buttons: &mut Vec<Element<FileAction>>,
) -> std::io::Result<()> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry_path = entry?.path();
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

            if entry_path.is_dir() {
                let is_expanded = state.expanded_directories.contains(&full_path);
                let row = create_directory_row(
                    &filename,
                    &full_path,
                    button_size,
                    is_expanded,
                    indent_level,
                );
                buttons.push(row);

                // If directory is expanded, recursively render its contents
                if is_expanded {
                    render_directory_contents(
                        &entry_path,
                        state,
                        indent_level + 1,
                        button_size,
                        buttons,
                    )?;
                }
            } else if entry_path.is_file() {
                let row = create_file_row(&filename, &full_path, button_size, indent_level);
                buttons.push(row);
            }
        }
    }

    Ok(())
}

pub fn view(state: &State) -> Element<'_, FileAction> {
    let mut buttons: Vec<Element<FileAction>> = Vec::new();

    let button_height = 42;

    // Render contents of current directory (starting at indent level 0)
    if let Err(err) = render_directory_contents(
        &std::path::Path::new("."),
        state,
        0,
        button_height,
        &mut buttons,
    ) {
        error!("Error rendering directory contents: {}", err);
    }

    let file_list = Column::from_vec(buttons).width(Length::Fill);
    let scrollable_list = scrollable(file_list);
    container(scrollable_list)
        .padding(10)
        .width(Length::Fill)
        .into()
}
