use iced::{Alignment, Application, Color, Command, Element, Length, Settings, Theme};
use iced::widget::{self, button, column, container, row, text, Column, PaneGrid, scrollable};

use crate::Message;

#[derive(Debug)]
pub struct Sidebar {
    hidden: bool,
    connection_names: Vec<String>,
}

impl Sidebar {
    pub fn new(connection_names: Vec<String>) -> Self {
        Self { hidden: false, connection_names }
    }

    pub fn toggle(&mut self) {
        self.hidden = !self.hidden;
    }

    pub fn view(&self) -> Element<Message> {
        let mut column = column![].spacing(1);

        for name in &self.connection_names {
            column = column.push(text(name));
            // column = column.push(button(name).on_press(Message::SelectConnection(name.clone())));
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
        .into()

        // if self.hidden {
        //     sidebar.width(Length::Shrink).into()
        // } else {
        //     sidebar.width(Length::Units(200)).into()
        // }
    }
}
