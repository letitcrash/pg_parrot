mod sidebar;
use iced::theme::palette::Danger;
use iced::widget::{self, button, column, container, row, text, Column, PaneGrid};
use iced::{Alignment, Application, Color, Command, Element, Length, Settings, Theme};
use pgp_core::config::Config;
use pgp_core::connection::{self, Connection};
use pgp_core::error::Error;

#[derive(Debug)]
pub struct Dashboard {
    config: Config,
    sidebar: sidebar::Sidebar,
}

#[derive(Debug, Clone)]
pub enum Message {
    Connect(u8),
    Disconnect(u8),
    Connected(Result<Connection, Error>),
    Disconnected(Result<Connection, Error>),
}

impl Dashboard {
    pub fn new(config: crate::Config) -> Self {
        Self {
            sidebar: sidebar::Sidebar::new(),
            config,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Connect(id) => {
                let connection = self.config.get_connection(id).clone();

                println!("Connecting to {:?}", connection.database);
                Command::perform(pgp_core::start_client(connection), Message::Connected)
            }
            Message::Disconnect(id) => {
                let connection = self.config.get_connection(id).clone();

                // println!("Disconnecting from {:?}", id);
                Command::perform(pgp_core::stop_client(connection), Message::Disconnected)
            }
            Message::Connected(Ok(connention)) => {
                println!("Connected");
                let mut connections = self.config.connections.as_ref().unwrap().clone();
                let index = connections
                    .iter()
                    .position(|c| c.id == connention.id)
                    .unwrap();
                connections[index] = connention;

                self.config.connections = Some(connections);

                Command::none()
            }
            Message::Connected(Err(error)) => {
                dbg!(error);
                Command::none()
            }
            Message::Disconnected(Ok(connention)) => {
                println!("Disconnected");
                let mut connections = self.config.connections.as_ref().unwrap().clone();
                let index = connections
                    .iter()
                    .position(|c| c.id == connention.id)
                    .unwrap();
                connections[index] = connention;

                self.config.connections = Some(connections);

                Command::none()
            }
            Message::Disconnected(Err(error)) => {
                dbg!(error);
                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let height_margin = if cfg!(target_os = "macos") { 20 } else { 0 };
        let sidebar = self.sidebar.view(&self.config);

        let base = row![]
            // .push_maybe(sidebar)
            .push(sidebar)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([height_margin, 0, 0, 0]);

        base.into()
    }
}
