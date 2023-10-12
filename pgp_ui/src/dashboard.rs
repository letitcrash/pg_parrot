mod sidebar;
mod viewport;
use std::collections::BTreeMap;

use iced::theme::palette::Danger;
use iced::widget::{self, button, column, container, row, text, Column, PaneGrid};
use iced::{Alignment, Application, Color, Command, Element, Length, Settings, Theme};
use pgp_core::config::Config;
use pgp_core::connection::{self, Connection};
use pgp_core::error::Error;
use pgp_core::Database;
use viewport::Viewport;

#[derive(Debug)]
pub struct Dashboard {
    sidebar: sidebar::Sidebar,
    viewport: Viewport,
    connections_state: BTreeMap<u8, bool>,
    config: Config,
}

#[derive(Debug, Clone)]
pub enum Message {
    Connect(u8),
    Disconnect(u8),
    Connected(Result<Database, Error>),
    Viewppoort(viewport::Message),
}

impl Dashboard {
    pub fn new(config: crate::Config) -> Self {
        Self {
            sidebar: sidebar::Sidebar::new(),
            viewport: Viewport::default(),
            connections_state: config.default_state(),
            config,
        }
    }

    pub fn title(&self) -> &str {
        match &self.viewport {
            Viewport::Ready { db, .. } => self.config.get_connection(db.id).database.as_str(),
            Viewport::Loading { .. } => "Loading",
            _ => "Dashboard",
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Connect(id) => {
                self.connections_state = self.config.default_state();

                let connection = self.config.get_connection(id).clone();
                let name = connection.database.clone();
                self.viewport = Viewport::Loading { name, message };
                Command::perform(pgp_core::client(connection), Message::Connected)
            }
            Message::Disconnect(id) => {
                self.connections_state.insert(id, false);
                self.viewport = Viewport::default();
                Command::none()
            }
            Message::Connected(Ok(db)) => {
                self.connections_state.insert(db.id, true);
                self.viewport = Viewport::new(db);

                Command::none()
            }
            Message::Connected(Err(error)) => {
                self.viewport = Viewport::Errored { error };
                Command::none()
            }
            Message::Viewppoort(message) => self.viewport.update(message).map(Message::Viewppoort),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let height_margin = if cfg!(target_os = "macos") { 20 } else { 0 };
        let sidebar = self.sidebar.view(&self.config, &self.connections_state);
        let viewport = self.viewport.view(&self.config).map(Message::Viewppoort);
        let base = row![].push(sidebar).push(viewport);

        base.width(Length::Fill)
            .height(Length::Fill)
            .padding([height_margin, 0, 0, 0])
            .into()
    }
}
