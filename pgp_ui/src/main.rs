mod dashboard;
mod error;

use dashboard::Dashboard;
use error::ErrorExt;
use iced::executor;
use iced::widget::{self, button, column, container, row, text};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme};

use pgp_core::config::Config;
use pgp_core::error::Error;

#[tokio::main]
async fn main() -> iced::Result {
    PgParrot::run(Settings::default())
}

#[derive(Debug)]
enum PgParrot {
    Loading,
    Ready {
        dashboard: Dashboard,
        // notifications: Vec<Notification>,
    },
    Errored {
        error: Error,
    },
}

#[derive(Debug, Clone)]
pub enum Message {
    BuildConfig(Result<Config, Error>),
    Retry,
    Dashboard(dashboard::Message),
    // Error(error::Message),
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
            Command::perform(Config::new(), Message::BuildConfig),
        )
    }

    fn title(&self) -> String {
        let subtitle = match self {
            PgParrot::Ready { dashboard, .. } => dashboard.title(),
            PgParrot::Errored { .. } => "Whoops!",
            _ => "Loading",
        };

        format!("{subtitle} - PgParrot")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Exit => window::close(),
            Message::Retry => Command::perform(Config::new(), Message::BuildConfig),
            Message::BuildConfig(Ok(config)) => {
                let dashboard = Dashboard::new(config);
                *self = PgParrot::Ready { dashboard };
                Command::none()
            }
            Message::BuildConfig(Err(error)) => {
                // dbg!(error);
                *self = PgParrot::Errored { error };
                Command::none()
            }
            Message::Dashboard(message) => match self {
                PgParrot::Ready { dashboard } => {
                    dashboard.update(message).map(Message::Dashboard)
                },
                _ => Command::none(),
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self {
            PgParrot::Loading => column![text("Loading...").size(18),]
                .width(Length::Shrink)
                .into(),
            PgParrot::Errored { error } => error.view(),
            PgParrot::Ready { dashboard } => dashboard.view().map(Message::Dashboard),
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
