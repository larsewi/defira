use iced::widget::{button, container, row, svg, text, Column};
use iced::{Element, Fill};
use log::debug;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
enum FileAction {
    Edit(String),
    Delete(String),
    Clipboard(String),
    DirectoryToggle(String),
    AddUser(String),
}

struct State {
    expanded_directories: HashSet<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            expanded_directories: HashSet::new(),
        }
    }
}

const CLIPBOARD_LOGO: &'static [u8] = include_bytes!("../assets/clipboard.svg");
const DELETE_LOGO: &'static [u8] = include_bytes!("../assets/delete.svg");
const EDIT_LOGO: &'static [u8] = include_bytes!("../assets/edit.svg");
const CHEVRON_RIGHT_LOGO: &'static [u8] = include_bytes!("../assets/chevron-right.svg");
const ADD_USER_LOGO: &'static [u8] = include_bytes!("../assets/add-user.svg");

fn update(state: &mut State, action: FileAction) {
    match action {
        FileAction::Edit(path) => debug!("Edit clicked for path '{}'", path),
        FileAction::Delete(path) => debug!("Delete clicked for path '{}'", path),
        FileAction::Clipboard(path) => {
            debug!("Clipboard clicked for path '{}'", path)
        }
        FileAction::DirectoryToggle(path) => {
            println!("Directory toggle clicked for: '{}'", path);
            if state.expanded_directories.contains(&path) {
                state.expanded_directories.remove(&path);
            } else {
                state.expanded_directories.insert(path);
            }
        }
        FileAction::AddUser(path) => println!("Add user clicked for path '{}'", path),
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

fn view(_state: &State) -> Element<'_, FileAction> {
    let mut buttons: Vec<Element<FileAction>> = Vec::new();

    let filename_width = 120;
    let button_height = 42;

    let entries = fs::read_dir("./").unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        if let Some(filename) = path.file_name() {
            let filename_str = filename.display().to_string();

            debug!(
                "Creating row for {} {}",
                if path.is_dir() { "directory" } else { "file" },
                filename_str
            );
            if path.is_dir() {
                let chevron = create_svg_button(
                    CHEVRON_RIGHT_LOGO,
                    FileAction::DirectoryToggle(filename_str.clone()),
                    button_height,
                );
                let filename_text = text(filename_str.clone()).width(filename_width);
                let add_user = create_svg_button(
                    ADD_USER_LOGO,
                    FileAction::AddUser(filename_str),
                    button_height,
                );
                let listing =
                    row![chevron, filename_text, add_user].align_y(iced::Alignment::Center);

                buttons.push(listing.into());
            } else {
                let filename_text = text(filename_str.clone()).width(filename_width);
                let edit = create_svg_button(
                    EDIT_LOGO,
                    FileAction::Edit(filename_str.clone()),
                    button_height,
                );
                let delete = create_svg_button(
                    DELETE_LOGO,
                    FileAction::Delete(filename_str.clone()),
                    button_height,
                );
                let clipboard = create_svg_button(
                    CLIPBOARD_LOGO,
                    FileAction::Clipboard(filename_str),
                    button_height,
                );
                let listing =
                    row![filename_text, edit, delete, clipboard].align_y(iced::Alignment::Center);

                buttons.push(listing.into());
            }
        }
    }

    let file_list = Column::from_vec(buttons);
    container(file_list)
        .padding(10)
        .center_x(Fill)
        .center_y(Fill)
        .into()
}

fn main() -> iced::Result {
    env_logger::init();
    defira::print_hello();
    iced::run("defira", update, view)
}
