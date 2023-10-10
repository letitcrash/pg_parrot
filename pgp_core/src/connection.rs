use serde::Deserialize;
use serde::Deserializer;
use tokio_postgres::NoTls;
use url::Url;

use crate::error::Error;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use tokio_postgres::Client;

const PG_DEFAULT_PORT: u16 = 5432;
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
    pub active: bool,
    pub client: Arc<Mutex<Option<Client>>>,
}

impl Connection {
    pub async fn client(&self) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(
            "postgres://test_user:secret_password@localhost/test_database",
            NoTls,
        )
        .await?;

        // tokio::spawn(async move {
        //     if let Err(e) = connection.await {
        //         eprintln!("connection error: {}", e);
        //     }
        // });

        Ok(Connection {
            id: self.id,
            username: self.username.clone(),
            password: self.password.clone(),
            host: self.host.clone(),
            port: self.port,
            database: self.database.clone(),
            sslmode: self.sslmode.clone(),
            active: self.active,
            client: Arc::new(Mutex::new(Some(client))),
        })
    }

    pub fn url(&self) -> String {
        let mut url = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        );

        if let Some(sslmode) = &self.sslmode {
            url.push_str(&format!("?sslmode={}", sslmode));
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
                active,
                client: Arc::new(Mutex::new(None)),
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
                client: Arc::new(Mutex::new(None)),
            })
        }
    }
}
