pub mod application;

use std::error::Error;
use crate::application::Application;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    defira::print_hello();

    let app = Application::init()?;
    app.events();
    app.update();
    app.render();
    app.exit();

    Ok(())
}
