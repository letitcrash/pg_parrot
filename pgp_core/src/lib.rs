use connection::Connection;
use error::Error;
use tokio_postgres::NoTls;

pub mod config;
pub mod connection;
pub mod error;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// #[tokio::main]
pub async fn start_client(conn: Connection) -> Result<Connection, Error> {
    let (client, connection) = tokio_postgres::connect(conn.url().as_str(), NoTls).await?;

    println!("Connected to {}", conn.url());

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(Connection {
        client: std::sync::Arc::new(std::sync::Mutex::new(Some(client))),
        active: true,
        ..conn
    })
}

pub async fn stop_client(conn: Connection) -> Result<Connection, Error> {
    Ok(Connection {
        client: std::sync::Arc::new(std::sync::Mutex::new(None)),
        active: false,
        ..conn
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
