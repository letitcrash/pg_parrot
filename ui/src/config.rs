use super::error::Error;
use serde::{Deserialize, Deserializer};
use std::io::Read;
use std::sync::atomic::{AtomicU8, Ordering};
use std::{collections::HashMap, fs::File};
use url::Url;

const PG_DEFAULT_PORT: u16 = 5432;
static NEXT_ID: AtomicU8 = AtomicU8::new(1);

#[derive(Debug, Clone)]
pub struct Connection {
    id: u8,
    username: String,
    password: String,
    host: String,
    port: u16,
    database: String,
    sslmode: Option<String>,
    active: bool,
}

impl Connection {
    pub async fn connect(&mut self) -> Result<(), Error> {
        self.active = true;
        Ok(())
    }
}

impl<'de> Deserialize<'de> for Connection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map: HashMap<String, toml::Value> = HashMap::deserialize(deserializer)?;
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        let active = false;

        if map.contains_key("url") {
            let url = map.remove("url").unwrap().as_str().unwrap().to_string();
            let u = Url::parse(&url).map_err(|e| serde::de::Error::custom(e.to_string()))?;
            let username = u.username().to_string();
            let password = u.password().unwrap_or("").to_string();
            let host = u.host_str().unwrap().to_string();
            let port = u.port().unwrap_or(PG_DEFAULT_PORT);
            let database = u.path().trim_start_matches('/').to_string();
            let sslmode = u
                .query_pairs()
                .find(|(k, _)| k == "sslmode")
                .map(|(_, v)| v.to_string());

            Ok(Self {
                id,
                username,
                password,
                host,
                port,
                database,
                sslmode,
                active,
            })
        } else {
            let username = map
                .remove("username")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            let password = map
                .remove("password")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            let host = map.remove("host").unwrap().as_str().unwrap().to_string();
            let port = map
                .remove("port")
                .unwrap()
                .as_integer()
                .unwrap_or(PG_DEFAULT_PORT as i64) as u16;
            let database = map
                .remove("database")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            let sslmode = map
                .remove("sslmode")
                .map(|v| v.as_str().unwrap().to_string());

            Ok(Self {
                id,
                username,
                password,
                host,
                port,
                database,
                sslmode,
                active,
            })
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenAI {
    pub token: String,
}

#[derive(Debug, Deserialize, Clone)]
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

        print!("{:?}", config);

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

    pub fn set_connection_active(&self, id: u8, active: bool) -> Self {
        let mut connections = self.connections.as_ref().unwrap().clone();
        let index = connections.iter().position(|c| c.id == id).unwrap();
        connections[index].active = active;

        Self {
            connections: Some(connections),
            ..self.clone()
        }
    }
}
