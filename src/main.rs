use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    defira::print_hello();

    Ok(())
}
