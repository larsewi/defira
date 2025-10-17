use iced::widget::{button, container, row, svg, text, Column, Space};
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
    AddFile(String),
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
const ADD_FILE_LOGO: &'static [u8] = include_bytes!("../assets/add-file.svg");

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
        FileAction::AddFile(path) => println!("Add file clicked for path '{}'", path),
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
    filename: String,
    text_width: u16,
    button_size: u16,
) -> Element<'static, FileAction> {
    let chevron = create_svg_button(
        CHEVRON_RIGHT_LOGO,
        FileAction::DirectoryToggle(filename.clone()),
        button_size,
    );
    let filename_text = text(filename.clone()).width(text_width);
    let add_file = create_svg_button(ADD_FILE_LOGO, FileAction::AddFile(filename.clone()), button_size);
    let add_user = create_svg_button(ADD_USER_LOGO, FileAction::AddUser(filename), button_size);
    row![
        chevron,
        filename_text,
        Space::with_width(button_size),
        add_file,
        add_user
    ]
    .align_y(iced::Alignment::Center)
    .into()
}

fn create_file_row(
    filename: String,
    text_width: u16,
    button_size: u16,
) -> Element<'static, FileAction> {
    let filename_text = text(filename.clone()).width(text_width);
    let clipboard = create_svg_button(
        CLIPBOARD_LOGO,
        FileAction::Clipboard(filename.clone()),
        button_size,
    );
    let edit = create_svg_button(EDIT_LOGO, FileAction::Edit(filename.clone()), button_size);
    let delete = create_svg_button(DELETE_LOGO, FileAction::Delete(filename), button_size);
    row![
        Space::with_width(button_size),
        filename_text,
        clipboard,
        edit,
        delete
    ]
    .align_y(iced::Alignment::Center)
    .into()
}

fn view(_state: &State) -> Element<'_, FileAction> {
    let mut buttons: Vec<Element<FileAction>> = Vec::new();

    let filename_width = 512;
    let button_height = 42;

    let entries = fs::read_dir("./").unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        if let Some(filename) = path.file_name() {
            let filename = filename.display().to_string();

            debug!(
                "Creating row for {} {}",
                if path.is_dir() { "directory" } else { "file" },
                filename
            );

            let row = if path.is_dir() {
                create_directory_row(filename, filename_width, button_height)
            } else {
                create_file_row(filename, filename_width, button_height)
            };

            buttons.push(row);
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
