use core::fmt;

use iced::widget::{self, column, container, row, text, Column, button};
use iced::{
    Alignment, Application, Color, Command, Element, Length, Settings, Theme,
};
use crate::Message;


#[derive(Debug, Clone, Copy)]
pub enum Error {
    NotFound,
    ParseError,
    ConnectionError,
    QueryError
}

impl Error {
    pub fn view(&self) -> Column<Message> {
        match self {
            Error::NotFound =>  {
                column![
                    text("Config not found").size(18),
                    button("Retry").on_press(Message::Retry)
                ]
                .max_width(500)
                .spacing(20)
                .align_items(Alignment::Center)
            }
            Error::ParseError => {
                column![
                    text("Wrong config").size(18),
                    button("Exit!").on_press(Message::Retry)
                ]
                .max_width(500)
                .spacing(20)
                .align_items(Alignment::Center)
            }
            Error::ConnectionError => {
                column![text("Connection error").size(18)]
                    .width(Length::Shrink)
            }
            Error::QueryError => {
                column![text("Query error").size(18)]
                    .width(Length::Shrink)
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        dbg!(error);

        Error::NotFound 
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Error {
        dbg!(error);

        Error::ParseError
    }
}