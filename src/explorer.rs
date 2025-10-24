use crate::assets;
use iced::widget;
use iced::{Element, Length};
use log::{debug, error, trace};
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
pub enum FileAction {
    DirectoryToggle(String),
    Select(String),
    ContextMenu(String),
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
        FileAction::Select(path) => {
            debug!["Path '{}' clicked", path];
            if state.selected.contains(&path) {
                debug!("Path '{}' is unselected", path);
                state.selected.remove(&path);
            } else {
                debug!("Path '{}' is selected", path);
                state.selected.insert(path);
            }
        }
        FileAction::ContextMenu(path) => debug!("Context menu clicked for path '{}'", path),
    }
}

fn create_svg_button(
    svg_data: &'static [u8],
    action: FileAction,
    size: u16,
) -> widget::button::Button<'static, FileAction> {
    let icon = widget::svg(widget::svg::Handle::from_memory(svg_data));
    widget::button(icon)
        .on_press(action)
        .width(size)
        .style(widget::button::text)
}

fn create_row(
    filename: &str,
    full_path: &str,
    button_size: u16,
    is_directory: bool,
    is_expanded: bool,
    indent_level: u16,
) -> Element<'static, FileAction> {
    let mut row = widget::row![]
        .align_y(iced::Alignment::Center)
        .width(Length::Fill);

    if is_directory {
        row = row.push(widget::Space::with_width(button_size * indent_level));
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
        row = row.push(chevron);
    } else {
        /* Files are indented one level more than directories (to account for no
         * chevron button at the beginning  */
        row = row.push(widget::Space::with_width(
            (button_size * indent_level) + (button_size * 1),
        ));
    }
    row = row.push(widget::text!["{}", filename].width(Length::Fill));

    widget::mouse_area(
        widget::button(row)
            .on_press(FileAction::Select(full_path.to_string()))
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
            .width(Length::Fill),
    )
    .on_right_press(FileAction::ContextMenu(full_path.to_string()))
    .into()
}

fn render_directory_contents(
    path: &std::path::Path,
    state: &State,
    indent_level: u16,
    indent_size: u16,
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

            let is_expanded = state.expanded.contains(&full_path);
            let row = create_row(
                &filename,
                &full_path,
                indent_size,
                entry_path.is_dir(),
                is_expanded,
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
                        indent_size,
                        buttons,
                    )?;
                }
            }
        }
    }

    Ok(())
}

pub fn view(state: &State) -> Element<'_, FileAction> {
    const INDENT_LEVEL: u16 = 0;
    let dir = std::path::Path::new(".");
    let button_height = 32;
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
