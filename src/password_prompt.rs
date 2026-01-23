use iced::widget;
use iced::{Element, Length};
use std::path::PathBuf;

/// State for the password prompt modal.
#[derive(Debug, Clone)]
pub struct State {
    /// The current password input value.
    pub password: String,
    /// Whether to show the password in plain text.
    pub show_password: bool,
    /// The file path being decrypted (for display purposes).
    pub target_path: PathBuf,
}

impl State {
    /// Create a new password prompt state for the given file path.
    pub fn new(target_path: PathBuf) -> Self {
        Self {
            password: String::new(),
            show_password: false,
            target_path,
        }
    }
}

/// Messages for password prompt interactions.
#[derive(Debug, Clone)]
pub enum Message {
    /// Password text changed.
    PasswordChanged(String),
    /// User submitted the password (clicked OK or pressed Enter).
    Submit,
    /// User cancelled the prompt.
    Cancel,
    /// Toggle password visibility.
    ToggleVisibility,
}

/// Creates a semi-transparent backdrop that blocks interaction with content behind the modal.
pub fn create_backdrop<M>(on_click: M) -> Element<'static, M>
where
    M: Clone + 'static,
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

/// Renders the password prompt modal.
///
/// # Arguments
/// * `state` - The current password prompt state
/// * `on_message` - Function to wrap password prompt messages into the parent message type
///
/// # Type Parameters
/// * `M` - The parent message type (e.g., FileAction)
pub fn view<'a, M>(state: &'a State, on_message: fn(Message) -> M) -> Element<'a, M>
where
    M: Clone + 'a,
{
    // Title
    let title = widget::text("Password")
        .size(20)
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center);

    // Password input - use fn pointer which is Copy
    let password_input = widget::text_input("Enter password...", &state.password)
        .on_input(move |s| on_message(Message::PasswordChanged(s)))
        .on_submit(on_message(Message::Submit))
        .secure(!state.show_password)
        .padding(10)
        .width(Length::Fill);

    // Visibility toggle button
    let visibility_icon = if state.show_password { "Hide" } else { "Show" };
    let visibility_button = widget::button(widget::text(visibility_icon).size(12))
        .on_press(on_message(Message::ToggleVisibility))
        .padding(10);

    let password_row = widget::row![password_input, visibility_button]
        .spacing(8)
        .align_y(iced::Alignment::Center);

    // Cancel and OK buttons
    let cancel_button = widget::button(
        widget::text("Cancel")
            .align_x(iced::alignment::Horizontal::Center)
            .width(Length::Fill),
    )
    .on_press(on_message(Message::Cancel))
    .padding(10)
    .width(80);

    let ok_button = widget::button(
        widget::text("OK")
            .align_x(iced::alignment::Horizontal::Center)
            .width(Length::Fill),
    )
    .on_press(on_message(Message::Submit))
    .padding(10)
    .width(80);

    let button_row = widget::row![widget::horizontal_space(), cancel_button, ok_button,]
        .spacing(12)
        .align_y(iced::Alignment::Center);

    // Modal content
    let content = widget::column![title, password_row, button_row,]
        .spacing(16)
        .padding(24)
        .width(350);

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
