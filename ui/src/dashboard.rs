use crate::Message;
mod sidebar;
use iced::widget::{self, button, column, container, row, text, Column, PaneGrid};
use iced::{Alignment, Application, Color, Command, Element, Length, Settings, Theme};

#[derive(Debug)]
pub struct Dashboard {
    config: super::Config,
    sidebar: sidebar::Sidebar,
}

impl Dashboard {
    pub fn new(config: super::Config) -> Self {
        let connection_names = config.connection_names();
        Self { config, sidebar: sidebar::Sidebar::new(connection_names) }
    }

    pub fn title(&self) -> &str {
        &self.config.openai.token
    }

    pub fn view(&self) -> Element<Message> {
        let height_margin = if cfg!(target_os = "macos") { 20 } else { 0 };
        let sidebar = self.sidebar.view();

        let base = row![]
            // .push_maybe(sidebar)
            .push(sidebar)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([height_margin, 0, 0, 0]);

        base.into()
    }
}
