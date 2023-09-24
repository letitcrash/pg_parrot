use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use super::error::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub name: String,
    pub ssl: bool,
}

impl Config {
    pub async fn new() -> Result<Self, Error> {
        let mut file = File::open("config.yaml")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_yaml::from_str(&contents)?;

        Ok(config)
    }

    pub fn view(&self) -> iced::Element<'_, super::Message> {
        iced::widget::Text::new(&self.name).size(18).into()
    }
}