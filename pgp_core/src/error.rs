use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    NotFound,
    ParseError,
    ConnectionError,
    QueryError,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        dbg!(error);

        Error::NotFound
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Error {
        dbg!(error);

        Error::ParseError
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(error: tokio_postgres::Error) -> Error {
        dbg!(error);

        Error::ConnectionError
    }
}

impl From<native_tls::Error> for Error {
    fn from(error: native_tls::Error) -> Error {
        dbg!(error);

        Error::ConnectionError
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        dbg!(self);

        match self {
            Error::NotFound => write!(f, "Config not found"),
            Error::ParseError => write!(f, "Wrong config"),
            Error::ConnectionError => write!(f, "Connection error"),
            Error::QueryError => write!(f, "Query error"),
        }
    }
}
