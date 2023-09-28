mod sidebar;
use iced::widget::{self, button, column, container, row, text, Column, PaneGrid};
use iced::{Alignment, Application, Color, Command, Element, Length, Settings, Theme};

#[derive(Debug)]
pub struct Dashboard {
    config: crate::Config,
    sidebar: sidebar::Sidebar,
}

#[derive(Debug, Clone)]
pub enum Message {
    Connect(u8),
    Disconnect(u8),
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
                self.config = self.config.set_connection_active(id, true);

                Command::none()
                // Command::perform(ai_client::connect(connection), Message::Connect)
            }
            Message::Disconnect(id) => {
                self.config = self.config.set_connection_active(id, false);
                Command::none()
                // Command::perform(ai_client::disconnect(connection), Message::Disconnect)
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
