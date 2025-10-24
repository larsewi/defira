mod assets;
mod explorer;

fn main() -> iced::Result {
    env_logger::init();
    defira::print_hello();
    iced::run("defira", explorer::update, explorer::view)
}
