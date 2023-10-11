use iced::widget::{
    self, button, column, container, row, scrollable, text, Column, Container, PaneGrid, Text,
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
    Ready,
}

impl Viewport {
    pub fn default() -> Self {
        Self::Default("Select a connection".to_string())
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

            Viewport::Ready => Container::new(
                Column::new()
                    .push(Text::new("Ready").size(18))
                    .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .style(theme::Container::Transparent)
            .into(),
        }
    }
}
