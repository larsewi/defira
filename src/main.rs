mod assets;
mod context_menu;
mod error_popup;
mod file_explorer;
mod password_prompt;

fn main() -> iced::Result {
    env_logger::init();
    iced::run("defira", file_explorer::update, file_explorer::view)
}
