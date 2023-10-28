use iced::widget::{self, button, column, container, row, text, Column};
use iced::{Alignment, Application, Color, Command, Element, Length, Settings, Theme};
use pgp_core::errors::Error;
use crate::Message;

pub trait ErrorExt {
    fn view(&self) -> Element<Message>;
}

// #[derive(Debug, Clone)]
// pub enum Message {
//     Retry,
// }

impl ErrorExt for Error {
    fn view(&self) -> Element<Message> {
        match self {
            Error::NotFound => column![
                text("Config not found").size(18),
                button("Retry").on_press(Message::Retry)
            ]
            .max_width(500)
            .spacing(20)
            .align_items(Alignment::Center)
            .into(),
            Error::ParseError => column![
                text("Wrong config").size(18),
                button("Exit!").on_press(Message::Retry)
            ]
            .max_width(500)
            .spacing(20)
            .align_items(Alignment::Center)
            .into(),
            Error::ConnectionError => column![text("Connection error").size(18)]
                .width(Length::Shrink)
                .into(),
            Error::QueryError => column![text("Query error").size(18)]
                .width(Length::Shrink)
                .into(),
        }
    }
}
