use std::error::Error;

pub struct Application {}

impl Application {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        let app = Self {};
        Ok(app)
    }

    pub fn events(&self) {

    }

    pub fn update(&self) {

    }

    pub fn render(&self) {

    }

    pub fn exit(&self) {

    }
}