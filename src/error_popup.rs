use iced::widget;
use iced::{Element, Length};

/// State for the error popup modal.
#[derive(Debug, Clone)]
pub struct State {
    /// The error title.
    pub title: String,
    /// The error message.
    pub message: String,
}

impl State {
    /// Create a new error popup state with the given title and message.
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
        }
    }
}

/// Messages for error popup interactions.
#[derive(Debug, Clone)]
pub enum Message {
    /// User dismissed the error popup (clicked OK or backdrop).
    Dismiss,
}

/// Creates a semi-transparent backdrop that blocks interaction with content behind the modal.
pub fn create_backdrop<'a, M>(on_click: M) -> Element<'a, M>
where
    M: Clone + 'a,
{
    widget::mouse_area(
        widget::container(widget::Space::new(Length::Fill, Length::Fill)).style(
            |_theme: &iced::Theme| widget::container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgba(
                    0.0, 0.0, 0.0, 0.5,
                ))),
                ..Default::default()
            },
        ),
    )
    .on_press(on_click)
    .into()
}

/// Renders the error popup modal.
///
/// # Arguments
/// * `state` - The current error popup state
/// * `on_message` - Function to wrap error popup messages into the parent message type
///
/// # Type Parameters
/// * `M` - The parent message type (e.g., FileAction)
pub fn view<'a, M>(state: &'a State, on_message: fn(Message) -> M) -> Element<'a, M>
where
    M: Clone + 'a,
{
    // Title
    let title = widget::text(&state.title)
        .size(20)
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center);

    // Message text
    let message = widget::text(&state.message)
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center);

    // OK button
    let ok_button = widget::button(
        widget::text("OK")
            .align_x(iced::alignment::Horizontal::Center)
            .width(Length::Fill),
    )
    .on_press(on_message(Message::Dismiss))
    .padding(10)
    .width(80);

    let button_row = widget::row![
        widget::horizontal_space(),
        ok_button,
        widget::horizontal_space()
    ]
    .align_y(iced::Alignment::Center);

    // Modal content
    let content = widget::column![title, message, button_row]
        .spacing(16)
        .padding(24)
        .width(400);

    // Modal container with styling
    let modal = widget::container(content)
        .style(|theme: &iced::Theme| widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::WHITE)),
            text_color: Some(theme.palette().text),
            border: iced::Border {
                color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 16.0,
            },
        })
        .width(Length::Shrink);

    // Center the modal on screen
    widget::container(modal)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}
