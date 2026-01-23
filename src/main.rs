mod assets;
mod context_menu;
mod crypto;
mod error_popup;
mod file_explorer;
mod setup;

use iced::Element;

pub enum Screen {
    Setup(setup::State),
    FileExplorer(file_explorer::State),
}

pub struct AppState {
    screen: Screen,
    github_url: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    Setup(setup::Message),
    FileExplorer(file_explorer::FileAction),
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            screen: Screen::Setup(setup::State::default()),
            github_url: None,
        }
    }
}

fn update(state: &mut AppState, message: AppMessage) {
    match message {
        AppMessage::Setup(msg) => {
            if let Screen::Setup(ref mut setup_state) = state.screen {
                if let Some(url) = setup::update(setup_state, msg) {
                    state.github_url = Some(url);
                    state.screen = Screen::FileExplorer(file_explorer::State::default());
                }
            }
        }
        AppMessage::FileExplorer(action) => {
            if let Screen::FileExplorer(ref mut explorer_state) = state.screen {
                file_explorer::update(explorer_state, action);
            }
        }
    }
}

fn view(state: &AppState) -> Element<'_, AppMessage> {
    match &state.screen {
        Screen::Setup(setup_state) => setup::view(setup_state).map(AppMessage::Setup),
        Screen::FileExplorer(explorer_state) => {
            file_explorer::view(explorer_state).map(AppMessage::FileExplorer)
        }
    }
}

fn main() -> iced::Result {
    env_logger::init();
    iced::run("defira", update, view)
}
