mod config;
mod dashboard;
mod error;

use ai_client;
use config::{Config, Connection};
use dashboard::Dashboard;
use error::Error;
use iced::executor;
use iced::widget::{self, button, column, container, row, text};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme};

pub fn main() -> iced::Result {
    PgParrot::run(Settings::default())
}

#[derive(Debug)]
enum PgParrot {
    Loading,
    Loaded {
        dashboard: Dashboard,
        config: Config,
    },
    Errored {
        e: Error,
    },
}

#[derive(Debug, Clone)]
pub enum Message {
    Config(Result<Config, Error>),
    Retry,
    Exit,
    Connect(u8),
    Disconnect(u8),
    Connected(Result<(), Error>),
}

impl Application for PgParrot {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            PgParrot::Loading,
            Command::perform(Config::new(), Message::Config),
        )
    }

    fn title(&self) -> String {
        let subtitle = match self {
            PgParrot::Loaded { .. } => "Dashboard",
            PgParrot::Errored { .. } => "Whoops!",
            _ => "Loading",
        };

        format!("{subtitle} - PgParrot")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Exit => window::close(),
            Message::Retry => Command::perform(Config::new(), Message::Config),
            Message::Config(Ok(config)) => {
                // let connections = config.connections.unwrap();
                let dashboard = Dashboard::new();
                *self = PgParrot::Loaded { dashboard, config };
                Command::none()
            }
            Message::Config(Err(e)) => {
                dbg!(e);
                *self = PgParrot::Errored { e };
                Command::none()
            }
            Message::Connect(id) => match self {
                PgParrot::Loaded { dashboard, config } => {
                    let mut connection = config.get_connection(id).clone();
                    // match connection.connect().await {
                    //     Ok(()) => {
                    //         *self = PgParrot::Loaded {
                    //             dashboard: *dashboard,
                    //             config: config.set_connection_active(id, true),
                    //         };
                    //     }
                    //     Err(e) => {
                    //         *self = PgParrot::Errored { e };
                    //     }
                    // }
                    // Command::perform(conne, Message::Connected)

                    // *self = PgParrot::Loaded {
                    //     config: config.set_connection_active(id, true),
                    //     ..*self
                    // };

                    Command::none()
                }
                _ => Command::none(),
            },
            Message::Connected(Ok(())) => {
                dbg!("connected");
                Command::none()
            }
            Message::Connected(Err(e)) => {
                *self = PgParrot::Errored { e };
                Command::none()
            }
            Message::Disconnect(id) => {
                dbg!(id);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self {
            PgParrot::Loading => column![text("Loading...").size(18),]
                .width(Length::Shrink)
                .into(),
            PgParrot::Errored { e } => e.view(),
            PgParrot::Loaded { dashboard, config } => dashboard.view(config),
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
