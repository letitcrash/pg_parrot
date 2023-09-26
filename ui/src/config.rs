use super::error::Error;
use serde::{Deserialize, Deserializer};
use url::Url;
use std::io::Read;
use std::{collections::HashMap, fs::File};

const PG_DEFAULT_PORT: u16 = 5432;

#[derive(Debug, Clone)]
pub struct Connection {
    username: String,
    password: String,
    host: String,
    port: u16,
    database: String,
    sslmode: Option<String>,
}

impl<'de> Deserialize<'de> for Connection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map: HashMap<String, toml::Value> = HashMap::deserialize(deserializer)?;


        if map.contains_key("url") {
            let url = map.remove("url").unwrap().as_str().unwrap().to_string();
            let u = Url::parse(&url).map_err(|e| serde::de::Error::custom(e.to_string()))?;
            let username = u.username().to_string();
            let password = u.password().unwrap_or("").to_string();
            let host = u.host_str().unwrap().to_string();
            let port = u.port().unwrap_or(PG_DEFAULT_PORT);
            let database = u.path().trim_start_matches('/').to_string();
            let sslmode = u.query_pairs().find(|(k, _)| k == "sslmode").map(|(_, v)| v.to_string());

            Ok(Self {
                username,
                password,
                host,
                port,
                database,
                sslmode,
            })

        } else {
            let username = map.remove("username").unwrap().as_str().unwrap().to_string();
            let password = map.remove("password").unwrap().as_str().unwrap().to_string();
            let host = map.remove("host").unwrap().as_str().unwrap().to_string();
            let port = map.remove("port").unwrap().as_integer().unwrap_or(PG_DEFAULT_PORT as i64) as u16;
            let database = map.remove("database").unwrap().as_str().unwrap().to_string();
            let sslmode = map.remove("sslmode").map(|v| v.as_str().unwrap().to_string());

            Ok(Self {
                username,
                password,
                host,
                port,
                database,
                sslmode,
            })
        }
    }
}

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

        Ok(config)
    }

    pub fn connection_names(&self) -> Vec<String> {
        self.connections.as_ref().unwrap().iter().map(|c| c.database.clone()).collect()
    }
}
