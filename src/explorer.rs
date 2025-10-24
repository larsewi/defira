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
}

pub struct State {
    expanded: HashSet<String>,
    selected: HashSet<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            expanded: HashSet::new(),
            selected: HashSet::new(),
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
        }
        FileAction::ContextMenu(path, is_dir) => debug!(
            "Context menu clicked for {} '{}'",
            if is_dir { "directory" } else { "secret" },
            path
        ),
    }
}

fn create_row(
    filename: &str,
    full_path: &str,
    indent_width: u16,
    is_directory: bool,
    indent_level: u16,
) -> Element<'static, FileAction> {
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

pub fn view(state: &State) -> Element<'_, FileAction> {
    const INDENT_LEVEL: u16 = 0;
    let dir = std::path::Path::new(".");
    let indent_width = 24;
    let mut buttons: Vec<Element<FileAction>> = Vec::new();

    render_directory_contents(&dir, state, INDENT_LEVEL, indent_width, &mut buttons);

    let file_list = widget::Column::from_vec(buttons).width(Length::Fill);
    let scrollable_list = widget::scrollable(file_list);
    widget::container(scrollable_list)
        .padding(10)
        .width(Length::Fill)
        .into()
}
