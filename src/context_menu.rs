use iced::widget;
use iced::{Element, Length};

/// A menu item in a context menu.
/// Generic over the Message type to allow any module to use it.
pub struct MenuItem<Message> {
    pub label: String,
    pub icon: Option<&'static [u8]>,
    pub action: Message,
}

impl<Message> MenuItem<Message> {
    /// Create a new menu item with a label and action.
    pub fn new(label: impl Into<String>, action: Message) -> Self {
        Self {
            label: label.into(),
            icon: None,
            action,
        }
    }

    /// Add an icon to the menu item.
    /// Icon data must be 'static (e.g., from include_bytes! macro for embedded assets).
    /// This is required by iced's SVG widget.
    pub fn with_icon(mut self, icon: &'static [u8]) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Renders a context menu at the specified position with the given menu items.
///
/// This is a pure UI component that knows nothing about what the menu is for.
/// It only handles rendering and styling.
///
/// # Arguments
/// * `position` - Where to render the menu (typically mouse cursor position)
/// * `items` - The menu items to display
///
/// # Type Parameters
/// * `Message` - The message type for this context (FileAction, EditorAction, etc.)
pub fn view_context_menu<'a, Message>(
    position: &'a iced::Point,
    items: Vec<MenuItem<Message>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut menu_items = widget::column![].spacing(0);

    for item in items {
        let mut button_content = widget::row![].align_y(iced::Alignment::Center).padding(8);

        // Add icon if present
        if let Some(icon_data) = item.icon {
            let icon = widget::svg(widget::svg::Handle::from_memory(icon_data)).width(16);
            button_content = button_content.push(icon).push(widget::Space::with_width(8));
        }

        // Add label (clone to avoid borrowing issues)
        let text = widget::text(item.label.clone()).size(14);
        button_content = button_content.push(text);

        let button = widget::button(button_content)
            .on_press(item.action)
            .style(|theme: &iced::Theme, status| {
                let base = widget::button::Style {
                    background: Some(iced::Background::Color(iced::Color::WHITE)),
                    text_color: theme.palette().text,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                };
                match status {
                    widget::button::Status::Hovered => widget::button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgba(
                            0.3, 0.5, 0.8, 0.3,
                        ))),
                        ..base
                    },
                    _ => base,
                }
            })
            .width(Length::Fill);

        menu_items = menu_items.push(button);
    }

    // Create the menu container with styling
    let menu = widget::container(menu_items)
        .padding(4)
        .style(|theme: &iced::Theme| widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::WHITE)),
            text_color: Some(theme.palette().text),
            border: iced::Border {
                color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                offset: iced::Vector::new(2.0, 2.0),
                blur_radius: 8.0,
            },
        })
        .width(150);

    // Position the menu using Space widgets so it doesn't block clicks
    widget::column![
        widget::Space::with_height(position.y),
        widget::row![widget::Space::with_width(position.x), menu,],
    ]
    .into()
}

/// Creates a click-outside-to-dismiss layer for context menus.
///
/// This layer covers the entire screen and triggers the provided message
/// when clicked, allowing users to dismiss the context menu by clicking elsewhere.
///
/// # Arguments
/// * `on_dismiss` - The message to send when the layer is clicked
pub fn create_dismiss_layer<Message>(on_dismiss: Message) -> Element<'static, Message>
where
    Message: Clone + 'static,
{
    widget::mouse_area(widget::container(widget::Space::new(
        Length::Fill,
        Length::Fill,
    )))
    .on_press(on_dismiss)
    .into()
}
