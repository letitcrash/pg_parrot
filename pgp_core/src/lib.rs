use error::Error;
use native_tls::TlsConnector;
use postgres_native_tls::{MakeTlsConnector, TlsStream};
use std::sync::{Arc, Mutex};
use tokio_postgres::{
    tls::{NoTlsFuture, NoTlsStream},
    Client, NoTls, Socket,
};

pub mod config;
pub mod connection;
pub mod error;

#[derive(Debug, Clone)]
pub struct Database {
    pub id: u8,
    pub client: Arc<Mutex<Option<Client>>>,
}

pub async fn client(conn: connection::Connection) -> Result<Database, Error> {
    let url = conn.url().clone();
    let id = conn.id;

    match conn.sslmode {
        Some(sslmode) => match sslmode.as_str() {
            "require" => connect_ssl(url, id).await,
            _ => connect(url, id).await,
        },
        None => connect(url, id).await,
    }
}

async fn connect(url: String, id: u8) -> Result<Database, Error> {
    let (client, connection) = tokio_postgres::connect(url.as_str(), NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(Database {
        id,
        client: Arc::new(Mutex::new(Some(client))),
    })
}

async fn connect_ssl(url: String, id: u8) -> Result<Database, Error> {
    let connector = MakeTlsConnector::new(TlsConnector::new().unwrap());
    let (client, connection) = tokio_postgres::connect(url.as_str(), connector).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(Database {
        id,
        client: Arc::new(Mutex::new(Some(client))),
    })
}

pub async fn exec(input: String, db: Database) -> Result<String, Error> {
    println!("exec: {:?}", input);
    let client = db.client.lock().unwrap().take().unwrap();
    let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;
    let value: String = rows[0].get(0);

    Ok(value)
}
