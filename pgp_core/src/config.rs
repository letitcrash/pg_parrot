use crate::connection::Connection;
use crate::error::Error;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

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

    pub fn connection_names(&self) -> Vec<(String, u8, bool)> {
        self.connections
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| (c.database.clone(), c.id, c.active))
            .collect()
    }

    pub fn get_connection(&self, id: u8) -> &Connection {
        self.connections
            .as_ref()
            .unwrap()
            .iter()
            .find(|c| c.id == id)
            .unwrap()
    }

    // pub async fn start_connection(&self, id: u8) -> Result<Config, Error> {
    //     let connection = self.get_connection(id);
    //     let client = connection.client().await?;
    //     let mut connections = self.connections.as_ref().unwrap().clone();
    //     let index = connections.iter().position(|c| c.id == id).unwrap();
    //     connections[index].client = Arc::new(Mutex::new(Some(client)));

    //     Ok(Self {
    //         connections: Some(connections),
    //         ..self.clone()
    //     })
    // }

    // pub fn set_connection_active(&self, active: bool) -> Self {
    //     let mut connections = self.connections.as_ref().unwrap().clone();
    //     let index = connections.iter().position(|c| c.id == id).unwrap();
    //     connections[index].active = active;

    //     Self {
    //         connections: Some(connections),
    //         ..self.clone()
    //     }
    // }
}
