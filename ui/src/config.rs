use super::error::Error;
use serde::{Deserialize, Deserializer};
use std::io::Read;
use std::{collections::HashMap, fs::File};

const PG_DEFAULT_PORT: u16 = 5432;

#[derive(Debug, Clone, Deserialize)]
pub struct Connection {
    username: String,
    password: String,
    host: String,
    port: u16,
    database: String,
    sslmode: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConnectionUrl {
    url: String,
}

#[derive(Debug, Clone)]
pub enum DatabaseConfig {
    Connection(Connection),
    ConnectionUrl(ConnectionUrl),
}

impl<'de> Deserialize<'de> for DatabaseConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map: HashMap<String, toml::Value> = HashMap::deserialize(deserializer)?;


        if map.contains_key("url") {
            let url = map.remove("url").unwrap().as_str().unwrap().to_string();
            Ok(DatabaseConfig::ConnectionUrl(ConnectionUrl { url }))
        } else {
            let username = map.remove("username").unwrap().as_str().unwrap().to_string();
            let password = map.remove("password").unwrap().as_str().unwrap().to_string();
            let host = map.remove("host").unwrap().as_str().unwrap().to_string();
            let port = map.remove("port").unwrap().as_integer().unwrap_or(PG_DEFAULT_PORT as i64) as u16;
            let database = map.remove("database").unwrap().as_str().unwrap().to_string();
            let sslmode = map.remove("sslmode").map(|v| v.as_str().unwrap().to_string());

            Ok(DatabaseConfig::Connection(Connection {
                username,
                password,
                host,
                port,
                database,
                sslmode,
            }))
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenAI {
    pub token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub connections: Option<Vec<DatabaseConfig>>,
    pub openai: OpenAI,
}

impl Config {
    pub async fn new() -> Result<Self, Error> {
        let mut file = File::open("config.toml")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }
}
