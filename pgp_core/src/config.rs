use crate::connection::Connection;
use crate::errors::Error;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Deserialize)]
pub struct OpenAI {
    pub token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub connections: Option<Vec<Connection>>,
    pub openai: OpenAI,
}

impl Config {
    pub async fn new() -> Result<Self, Error> {
        let mut file = File::open("config.toml")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = toml::from_str(&contents)?;

        // print!("{:?}", config);

        Ok(config)
    }

    pub fn default_state(&self) -> BTreeMap<u8, bool> {
        let mut state = BTreeMap::new();

        if let Some(connections) = &self.connections {
            for connection in connections {
                state.insert(connection.id, false);
            }
        }

        state
    }

    pub fn get_connection(&self, id: u8) -> &Connection {
        self.connections
            .as_ref()
            .unwrap()
            .iter()
            .find(|c| c.id == id)
            .unwrap()
    }
}
