mod assets;
mod context_menu;
mod file_explorer;

fn main() -> iced::Result {
    env_logger::init();
    defira::print_hello();
    iced::run("defira", file_explorer::update, file_explorer::view)
}
