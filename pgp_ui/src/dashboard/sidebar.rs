use std::collections::BTreeMap;

use iced::widget::{self, button, column, container, row, scrollable, text, Column, PaneGrid};
use iced::{theme, Alignment, Application, Color, Command, Element, Length, Settings, Theme};

use super::Message;
use pgp_core::config::Config;

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

    pub fn view(&self, config: &Config, connections_state: &BTreeMap<u8, bool>) -> Element<Message> {
        let mut column = column![].spacing(1);
        let state: Vec<_> = connections_state.into_iter().collect();

        for (id, active) in state {
            let connection = config.get_connection(*id);
            let name = connection.database.clone();

            let button = if *active {
                button(text(name))
                    .on_press(Message::Disconnect(*id))
                    .style(theme::Button::Positive)
            } else {
                button(text(name))
                    .on_press(Message::Connect(*id))
                    .style(theme::Button::Secondary)
            };

            column = column.push(button.width(Length::Fill));
        }

        container(
            scrollable(column).direction(scrollable::Direction::Vertical(
                iced::widget::scrollable::Properties::default()
                    .width(0)
                    .scroller_width(0),
            )),
        )
        .padding([8, 8, 6, 6])
        .center_x()
        .max_width(150)
        .height(Length::Fill)
        .style(theme::Container::Box)
        .into()

        // if self.hidden {
        //     sidebar.width(Length::Shrink).into()
        // } else {
        //     sidebar.width(Length::Units(200)).into()
        // }
    }
}
