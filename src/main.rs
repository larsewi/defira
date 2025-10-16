use iced::widget::{button, container, text, Column};
use iced::{Element, Fill};
use log::trace;
use std::fs;

#[derive(Debug, Clone)]
enum FileAction {
    OnClick,
}

#[derive(Default)]
struct State {
    path: String,
}

fn update(state: &mut State, action: FileAction) {
    match action {
        FileAction::OnClick => state.path = String::from("../"),
    }
}

fn view(_state: &State) -> Element<'_, FileAction> {
    let mut buttons: Vec<Element<FileAction>> = Vec::new();

    let entries = fs::read_dir("./").unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        if let Some(filename) = path.file_name() {
            let filename = filename.display();
            let typ = if path.is_dir() { "directory" } else { "file" };
            trace!("Creating button for {} {}", typ, filename);

            let btn = button(text(filename.to_string()))
                .width(1000)
                .on_press(FileAction::OnClick)
                .into();
            buttons.push(btn);
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
    iced::run("A cool counter", update, view)
}
