use iced::widget;
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum Message {
    UrlChanged(String),
    Submit,
}

pub struct State {
    pub github_url: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            github_url: String::new(),
        }
    }
}

/// Returns Some(url) when the user submits, signaling a screen transition
pub fn update(state: &mut State, message: Message) -> Option<String> {
    match message {
        Message::UrlChanged(url) => {
            state.github_url = url;
            None
        }
        Message::Submit => Some(state.github_url.clone()),
    }
}

pub fn view(state: &State) -> Element<'_, Message> {
    let title = widget::text("Welcome to Defira")
        .size(32)
        .width(Length::Fill)
        .align_x(iced::Alignment::Center);

    let label = widget::text("Enter your GitHub repository URL:")
        .size(16)
        .width(Length::Fill)
        .align_x(iced::Alignment::Center);

    let input = widget::text_input("https://github.com/...", &state.github_url)
        .on_input(Message::UrlChanged)
        .on_submit(Message::Submit)
        .padding(10)
        .width(400);

    let button = widget::button(widget::text("Continue"))
        .on_press(Message::Submit)
        .padding([10, 30]);

    let content = widget::column![title, label, input, button]
        .spacing(20)
        .align_x(iced::Alignment::Center)
        .width(Length::Fill);

    widget::container(content)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}
