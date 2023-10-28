use serde::Deserialize;
use serde::Deserializer;
use url::Url;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};

const PG_DEFAULT_PORT: u16 = 5432;
const DEFAULT_CONNECT_TIMEOUT: u16 = 5;
static NEXT_ID: AtomicU8 = AtomicU8::new(1);

#[derive(Debug, Clone)]
pub struct Connection {
    pub id: u8,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub sslmode: Option<String>,
    pub cert: Option<String>,
    pub timeout: u16,
    // pub client: Arc<Mutex<Option<Client>>>,
}

impl Connection {
    pub fn url(&self) -> String {
        let mut url = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        );

        url.push_str(&format!("?connect_timeout={}", self.timeout));

        if let Some(sslmode) = &self.sslmode {
            url.push_str(&format!("&sslmode={}", sslmode));
        }

        url
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
                cert: None,
                timeout: DEFAULT_CONNECT_TIMEOUT,
                // client: Arc::new(Mutex::new(None)),
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

            let timeout = map
                .remove("timeout")
                .map(|v| v.as_integer().unwrap() as u16)
                .unwrap_or(DEFAULT_CONNECT_TIMEOUT);

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
                cert: None,
                timeout,
                // client: Arc::new(Mutex::new(None)),
            })
        }
    }
}
