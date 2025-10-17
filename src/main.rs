use iced::widget::{button, container, row, svg, text, Column};
use iced::{Element, Fill};
use log::trace;
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

fn view(_state: &State) -> Element<'_, FileAction> {
    let mut buttons: Vec<Element<FileAction>> = Vec::new();

    let entries = fs::read_dir("./").unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        if let Some(filename) = path.file_name() {
            if path.is_dir() {
                continue;
            }

            let filename = filename.display();
            trace!("Creating row for file {}", filename);

            let filename = text(filename.to_string()).width(100);

            let size = 40;

            let edit = svg(svg::Handle::from_memory(EDIT_LOGO));
            let edit = button(edit)
                .on_press(FileAction::Edit)
                .height(size)
                .width(size);

            let delete = svg(svg::Handle::from_memory(DELETE_LOGO));
            let delete = button(delete)
                .on_press(FileAction::Delete)
                .height(size)
                .width(size);

            let clipboard = svg(svg::Handle::from_memory(CLIPBOARD_LOGO));
            let clipboard = button(clipboard)
                .on_press(FileAction::Clipboard)
                .height(size)
                .width(size);

            let listing = row![filename, edit, delete, clipboard];

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
