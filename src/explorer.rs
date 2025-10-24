use crate::assets;
use iced::widget;
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
    Highlight(String),
}

pub struct State {
    expanded: HashSet<String>,
    highlighted: HashSet<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            expanded: HashSet::new(),
            highlighted: HashSet::new(),
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
            if state.expanded.contains(&path) {
                debug!("Directory '{}' is collapsed", path);
                state.expanded.remove(&path);
            } else {
                debug!("Directory '{}' is expanded", path);
                state.expanded.insert(path);
            }
        }
        FileAction::AddUser(path) => debug!("Add user clicked for path '{}'", path),
        FileAction::NewFile(path) => debug!("New file clicked for path '{}'", path),
        FileAction::Highlight(path) => {
            debug!["Path '{}' clicked", path];
            if state.highlighted.contains(&path) {
                debug!("Path '{}' is unselected", path);
                state.highlighted.remove(&path);
            } else {
                debug!("Path '{}' is selected", path);
                state.highlighted.insert(path);
            }
        }
    }
}

fn create_svg_button(
    svg_data: &'static [u8],
    action: FileAction,
    size: u16,
) -> widget::button::Button<'static, FileAction> {
    let icon = widget::svg(widget::svg::Handle::from_memory(svg_data))
        .width(size)
        .height(size);
    widget::button(icon)
        .on_press(action)
        .height(size)
        .width(size)
        .style(widget::button::text)
}

fn create_directory_row(
    filename: &str,
    full_path: &str,
    button_size: u16,
    is_expanded: bool,
    indent_level: u16,
) -> Element<'static, FileAction> {
    let indent = widget::Space::with_width(button_size * indent_level);
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
    let text = widget::text!["{}", filename].width(Length::Fill);
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
    widget::button(
        widget::row![indent, chevron, text, new_file, add_user, delete]
            .align_y(iced::Alignment::Center)
            .width(Length::Fill),
    )
    .on_press(FileAction::Highlight(full_path.to_string()))
    .width(Length::Fill)
    .style(widget::button::text)
    .into()
}

fn create_file_row(
    filename: &str,
    full_path: &str,
    button_size: u16,
    indent_level: u16,
) -> Element<'static, FileAction> {
    // Files are indented one level more than directories (to account for no chevron button)
    let indent = widget::Space::with_width(button_size * (indent_level + 1));
    let text = widget::text!["{}", filename].width(Length::Fill);
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
    widget::button(
        widget::row![indent, text, clipboard, edit, delete]
            .align_y(iced::Alignment::Center)
            .width(Length::Fill),
    )
    .on_press(FileAction::Highlight(full_path.to_string()))
    .width(Length::Fill)
    .style(widget::button::text)
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
                let is_expanded = state.expanded.contains(&full_path);
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
    const INDENT_LEVEL: u16 = 0;
    let dir = std::path::Path::new(".");
    let button_height = 42;
    let mut buttons: Vec<Element<FileAction>> = Vec::new();

    if let Err(err) =
        render_directory_contents(&dir, state, INDENT_LEVEL, button_height, &mut buttons)
    {
        error!("Error rendering directory contents: {}", err);
    }

    let file_list = widget::Column::from_vec(buttons).width(Length::Fill);
    let scrollable_list = widget::scrollable(file_list);
    widget::container(scrollable_list)
        .padding(10)
        .width(Length::Fill)
        .into()
}
