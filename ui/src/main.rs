mod config;
mod error;
mod dashboard;

use ai_client;
use config::Config;
use error::Error;
use dashboard::Dashboard;
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
    Loaded { dashboard: Dashboard },
    Errored { e: Error },
}

#[derive(Debug, Clone)]
pub enum Message {
    Config(Result<Config, Error>),
    Retry,
    Exit,
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
            PgParrot::Loaded { dashboard } => dashboard.title(),
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
                *self = PgParrot::Loaded { dashboard: Dashboard::new(config) };
                Command::none()
            }
            Message::Config(Err(e)) => {
                dbg!(e);
                *self = PgParrot::Errored { e };
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self {
            PgParrot::Loading => column![text("Loading...").size(18),].width(Length::Shrink).into(),
            PgParrot::Errored { e } => e.view(),
            PgParrot::Loaded { dashboard } => dashboard.view()
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
