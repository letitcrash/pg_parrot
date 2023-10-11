mod sidebar;
mod viewport;
use iced::theme::palette::Danger;
use iced::widget::{self, button, column, container, row, text, Column, PaneGrid};
use iced::{Alignment, Application, Color, Command, Element, Length, Settings, Theme};
use pgp_core::config::Config;
use pgp_core::connection::{self, Connection};
use pgp_core::error::Error;
use viewport::Viewport;

#[derive(Debug)]
pub struct Dashboard {
    config: Config,
    sidebar: sidebar::Sidebar,
    viewport: Viewport,
}

#[derive(Debug, Clone)]
pub enum Message {
    Connect(u8),
    Disconnect(u8),
    Connected(Result<Connection, Error>),
    Disconnected(Result<Connection, Error>),
    Query
}

impl Dashboard {
    pub fn new(config: crate::Config) -> Self {
        Self {
            sidebar: sidebar::Sidebar::new(),
            viewport: Viewport::default(),
            config,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Connect(id) => {
                let connection = self.config.get_connection(id).clone();
                let name = connection.database.clone();
                self.viewport = Viewport::Loading { name, message };
                println!("Connecting to {:?}", connection.database);
                Command::perform(pgp_core::start_client(connection), Message::Connected)
            }
            Message::Disconnect(id) => {
                let connection = self.config.get_connection(id).clone();
                let name = connection.database.clone();
                self.viewport = Viewport::Loading { name, message };
                Command::perform(pgp_core::stop_client(connection), Message::Disconnected)
            }
            Message::Connected(Ok(connention)) => {
                let mut connections = self.config.connections.as_ref().unwrap().clone();
                let index = connections
                    .iter()
                    .position(|c| c.id == connention.id)
                    .unwrap();
                connections[index] = connention;

                self.config.connections = Some(connections);
                self.viewport = Viewport::new();

                Command::none()
            }
            Message::Connected(Err(error)) => {
                self.viewport = Viewport::Errored(error);
                Command::none()
            }
            Message::Disconnected(Ok(connention)) => {
                let mut connections = self.config.connections.as_ref().unwrap().clone();
                let index = connections
                    .iter()
                    .position(|c| c.id == connention.id)
                    .unwrap();
                connections[index] = connention;

                self.config.connections = Some(connections);
                self.viewport = Viewport::default();
                Command::none()
            }
            Message::Disconnected(Err(error)) => {
                // dbg!(error);
                self.viewport = Viewport::Errored(error);
                Command::none()
            }
            Message::Query => {
                let connection = self.config.get_connection(0).clone();
                let name = connection.database.clone();
                self.viewport = Viewport::Loading { name, message };
                // Command::perform(pgp_core::query(connection), Message::Disconnected)
                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let height_margin = if cfg!(target_os = "macos") { 20 } else { 0 };
        let sidebar = self.sidebar.view(&self.config);
        let viewport = self.viewport.view(&self.config);
        let base = row![].push(sidebar).push(viewport);

        // let base = match &self.notification {
        //     Some(notification) => {
        //         let notification = notification.view();
        //         row![].push(sidebar).push(notification)
        //     }
        //     None => row![].push(sidebar),
        // };

        // row![].push(sidebar);

        // if self.notification.is_some() {
        //     let notification = self.notification.as_ref().unwrap().view(&self.config);
        //     base.push(notification);
        // }

        // base = match &self.notification {
        //     Some(notification) => base.push(notification.view(&self.config)),
        //     None => base,
        // };

        base.width(Length::Fill)
            .height(Length::Fill)
            .padding([height_margin, 0, 0, 0])
            .into()
    }
}
