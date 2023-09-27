mod sidebar;
use iced::widget::{self, button, column, container, row, text, Column, PaneGrid};
use iced::{Alignment, Application, Color, Command, Element, Length, Settings, Theme};

#[derive(Debug)]
pub struct Dashboard {
    // config: &'static crate::Config,
    sidebar: sidebar::Sidebar,
}

// #[derive(Debug, Clone)]
// pub enum Message {
//     Connect(u8),
//     Disconnect(u8)
// }

impl Dashboard {
    pub fn new() -> Self {
        Self {
            sidebar: sidebar::Sidebar::new(),
            // config,
        }
    }

    // pub fn title(&self) -> &str {
    //     &self.config.openai.token
    // }

    pub fn view(&self, config: &crate::Config) -> Element<crate::Message> {
        let height_margin = if cfg!(target_os = "macos") { 20 } else { 0 };
        let sidebar = self.sidebar.view(config);

        let base = row![]
            // .push_maybe(sidebar)
            .push(sidebar)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([height_margin, 0, 0, 0]);

        base.into()
    }
}
