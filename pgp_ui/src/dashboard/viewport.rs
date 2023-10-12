use std::vec;

use iced::gradient::Linear;
use iced::widget::{
    self, button, column, container, horizontal_rule, row, scrollable, text, text_input,
    vertical_space, Column, Container, PaneGrid, Text,
};
use iced::{theme, Alignment, Application, Color, Command, Element, Length, Settings, Theme};

use super::Error;
// use super::Message;
use pgp_core::config::{self, Config};
use pgp_core::Database;

#[derive(Debug)]
pub enum Viewport {
    Default(String),
    Loading {
        name: String,
        message: super::Message,
    },
    Errored {
        error: Error,
    },
    Ready {
        input: String,
        output: Vec<String>,
        db: Database,
    },
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Query,
    QueryComplete(Result<String, Error>),
}

impl Viewport {
    pub fn default() -> Self {
        Self::Default("Select a connection".to_string())
    }

    pub fn new(db: Database) -> Self {
        Self::Ready {
            input: String::new(),
            output: vec![],
            db,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(input) => {
                if let Viewport::Ready { input: i, .. } = self {
                    *i = input;
                }
                Command::none()
            }
            Message::Query => {
                if let Viewport::Ready { input, db, .. } = self {
                    return Command::perform(
                        pgp_core::exec(input.to_string(), db.clone()),
                        Message::QueryComplete,
                    );
                }
                Command::none()
            }
            Message::QueryComplete(Ok(result)) => {
                println!("Result: {:?}", result);

                Command::none()
            }
            Message::QueryComplete(Err(error)) => {
                *self = Viewport::Errored { error };
                Command::none()
            }
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

            Viewport::Errored { error } => Container::new(
                Column::new()
                    .push(
                        row![
                            text(error.to_string()).size(18),
                            // button("Retry").on_press(message)
                        ]
                        .align_items(iced::Alignment::Center)
                        .spacing(20.0),
                    )
                    .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .style(theme::Container::Transparent)
            .into(),

            Viewport::Loading { name, message } => {
                let label = match message {
                    super::Message::Connect(_) => format!("Connecting to {}", name),
                    super::Message::Disconnect(_) => format!("Disconnecting from {}", name),
                    _ => "Loading..".to_string(),
                };

                Container::new(
                    Column::new()
                        .push(
                            row![
                                text(label).size(18),
                                // Linear::new()
                                //     .easing(easing)
                                //     .cycle_duration(Duration::from_secs_f32(self.cycle_duration)),
                                // Circular::new()
                                //     .easing(easing)
                                //     .cycle_duration(Duration::from_secs_f32(self.cycle_duration))
                            ]
                            .align_items(iced::Alignment::Center)
                            .spacing(20.0),
                        )
                        .align_items(Alignment::Center),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center)
                .style(theme::Container::Transparent)
                .into()
            }

            Viewport::Ready { input, .. } => {
                // let mut column = column![].spacing(1);

                let text_input = text_input("Type something...", &input)
                    .on_input(Message::InputChanged)
                    .padding(10)
                    .size(18)
                    .width(Length::Fill);

                let button = button("Submit").padding(10).on_press(Message::Query);

                let chat = column![].width(Length::Fill).spacing(1);

                // chat.push("Hello");
                // chat.push("World");

                let scrollable = scrollable(chat)
                    .direction(scrollable::Direction::Vertical(
                        iced::widget::scrollable::Properties::default()
                            .width(0)
                            .alignment(iced::widget::scrollable::Alignment::End)
                            .scroller_width(0),
                    ))
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
