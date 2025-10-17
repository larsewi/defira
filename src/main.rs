use defira::assets::*;
use iced::widget::*;
use iced::{Element, Length};
use log::{debug, trace};
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
enum FileAction {
    Edit(String),
    Delete(String),
    Clipboard(String),
    DirectoryToggle(String),
    AddUser(String),
    NewFile(String),
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

fn update(state: &mut State, action: FileAction) {
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
    filename: String,
    button_size: u16,
    is_expanded: bool,
) -> Element<'static, FileAction> {
    let chevron_logo = if is_expanded {
        CHEVRON_DOWN_LOGO
    } else {
        CHEVRON_RIGHT_LOGO
    };
    let chevron = create_svg_button(
        chevron_logo,
        FileAction::DirectoryToggle(filename.clone()),
        button_size,
    );
    let filename_text = text(filename.clone()).width(Length::Fill);
    let add_file = create_svg_button(
        ADD_FILE_LOGO,
        FileAction::NewFile(filename.clone()),
        button_size,
    );
    let add_user = create_svg_button(
        ADD_USER_LOGO,
        FileAction::AddUser(filename.clone()),
        button_size,
    );
    let delete = create_svg_button(DELETE_LOGO, FileAction::Delete(filename), button_size);
    row![chevron, filename_text, add_file, add_user, delete]
        .align_y(iced::Alignment::Center)
        .width(Length::Fill)
        .into()
}

fn create_file_row(filename: String, button_size: u16) -> Element<'static, FileAction> {
    let filename_text = text(filename.clone()).width(Length::Fill);
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
    .width(Length::Fill)
    .into()
}

fn view(state: &State) -> Element<'_, FileAction> {
    let mut buttons: Vec<Element<FileAction>> = Vec::new();

    let button_height = 42;

    let entries = fs::read_dir("./").unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        if let Some(filename) = path.file_name() {
            let filename = filename.display().to_string();

            trace!(
                "Creating row for {} {}",
                if path.is_dir() { "directory" } else { "file" },
                filename
            );

            if path.is_dir() {
                let is_expanded = state.expanded_directories.contains(&filename);
                let row = create_directory_row(filename, button_height, is_expanded);
                buttons.push(row);
            } else if path.is_file() {
                let row = create_file_row(filename, button_height);
                buttons.push(row);
            }
        }
    }

    let file_list = Column::from_vec(buttons).width(Length::Fill);
    let scrollable_list = scrollable(file_list);
    container(scrollable_list)
        .padding(10)
        .width(Length::Fill)
        .into()
}

fn main() -> iced::Result {
    env_logger::init();
    defira::print_hello();
    iced::run("defira", update, view)
}
