use iced::widget::{button, container, row, svg, text, Column};
use iced::{Element, Fill};
use log::debug;
use std::fs;

#[derive(Debug, Clone)]
enum FileAction {
    Edit,
    Delete,
    Clipboard,
}

#[derive(Default)]
struct State {
    path: String,
}

const CLIPBOARD_LOGO: &'static [u8] = include_bytes!("../assets/clipboard.svg");
const DELETE_LOGO: &'static [u8] = include_bytes!("../assets/delete.svg");
const EDIT_LOGO: &'static [u8] = include_bytes!("../assets/edit.svg");

fn update(state: &mut State, action: FileAction) {
    match action {
        FileAction::Edit => println!("Edit clicked"),
        FileAction::Delete => println!("Delete clicked"),
        FileAction::Clipboard => println!("Clipboard clicked"),
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
            if path.is_dir() {
                continue;
            }

            let filename = filename.display();
            debug!("Creating row for file {}", filename);

            let filename = text(filename.to_string()).width(filename_width);
            let edit = create_svg_button(EDIT_LOGO, FileAction::Edit, button_height);
            let delete = create_svg_button(DELETE_LOGO, FileAction::Delete, button_height);
            let clipboard = create_svg_button(CLIPBOARD_LOGO, FileAction::Clipboard, button_height);
            let listing = row![filename, edit, delete, clipboard].align_y(iced::Alignment::Center);

            buttons.push(listing.into());
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
