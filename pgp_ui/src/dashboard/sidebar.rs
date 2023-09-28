use iced::widget::{self, button, column, container, row, scrollable, text, Column, PaneGrid};
use iced::{theme, Alignment, Application, Color, Command, Element, Length, Settings, Theme};

use pgp_core::config::{Config, Connection};

#[derive(Debug)]
pub struct Sidebar {
    hidden: bool,
}

impl Sidebar {
    pub fn new() -> Self {
        Self { hidden: false }
    }

    pub fn toggle(&mut self) {
        self.hidden = !self.hidden;
    }

    pub fn view(&self, config: &Config) -> Element<super::Message> {
        let mut column = column![].spacing(1);

        for (name, id, active) in config.connection_names() {
            let (title, action, style) = if active {
                (
                    format!("{} (connected)", name),
                    super::Message::Disconnect(id),
                    theme::Button::Primary,
                )
            } else {
                (
                    name,
                    super::Message::Connect(id),
                    theme::Button::Secondary,
                )
            };

            let button = button(text(title)).on_press(action).style(style);

            column = column.push(button);
        }

        container(
            scrollable(column).direction(scrollable::Direction::Vertical(
                iced::widget::scrollable::Properties::default()
                    .width(0)
                    .scroller_width(0),
            )),
        )
        .padding([8, 0, 6, 6])
        .center_x()
        .max_width(120)
        .style(theme::Container::Box)
        .into()

        // if self.hidden {
        //     sidebar.width(Length::Shrink).into()
        // } else {
        //     sidebar.width(Length::Units(200)).into()
        // }
    }
}
