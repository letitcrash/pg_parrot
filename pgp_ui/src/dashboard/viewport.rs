use iced::widget::{
    self, button, column, container, horizontal_rule, row, scrollable, text, text_input,
    vertical_space, Column, Container, PaneGrid, Text,
};
use iced::{theme, Alignment, Application, Color, Command, Element, Length, Settings, Theme};

use super::Error;
use super::Message;
use pgp_core::config::{self, Config};
use pgp_core::connection::Connection;

#[derive(Debug)]
pub enum Viewport {
    Default(String),
    Loading {
        name: String,
        message: super::Message,
    },
    Errored(Error),
    Ready {
        query: String,
        input: text_input::State,
    },
}

impl Viewport {
    pub fn default() -> Self {
        Self::Default("Select a connection".to_string())
    }

    pub fn new() -> Self {
        Self::Ready {
            query: String::new(),
            input: text_input::State::new(),
        }
    }

    pub fn view(&self, config: &Config) -> Element<Message> {
        match self {
            Viewport::Default(text) => Container::new(
                Column::new()
                    .push(Text::new(text).size(18))
                    .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .style(theme::Container::Transparent)
            .into(),

            Viewport::Errored(error) => Container::new(
                Column::new()
                    .push(Text::new(error.to_string()).size(18))
                    .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .style(theme::Container::Transparent)
            .into(),

            Viewport::Loading { name, message } => {
                let text = match message {
                    Message::Connect(_) => format!("Connecting to {}", name),
                    Message::Disconnect(_) => format!("Disconnecting from {}", name),
                    _ => "Loading..".to_string(),
                };

                Container::new(
                    Column::new()
                        .push(Text::new(text).size(18))
                        .align_items(Alignment::Center),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center)
                .style(theme::Container::Transparent)
                .into()
            }

            Viewport::Ready { query, input } => {
                // let mut column = column![].spacing(1);

                let text_input = text_input(&query, "Query")
                    .on_submit(Message::Query)
                    .padding(5)
                    .size(18)
                    .width(Length::Fill);

                let button = button("Submit").padding(10).on_press(Message::Query);

                // let input = text_input(&mut self.input, "Query")
                //     .on_submit(Message::Query)
                //     .padding(5)
                //     .size(18)
                //     .width(Length::Fill);

                // let button = button(text("Send"))
                //     .on_press(Message::Query)
                //     .style(theme::Button::Primary)
                //     .width(Length::Fill);

                let scrollable = scrollable(
                    column!["Scroll me!", vertical_space(800), "You did it!"].width(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill);

                let content = column![
                    scrollable,
                    // horizontal_rule(38),
                    row![text_input, button]
                        .spacing(10)
                        .align_items(Alignment::Center),
                ]
                .spacing(20)
                .padding(20)
                .width(Length::Fill);
                // .max_width(600);

                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
        }
    }
}
