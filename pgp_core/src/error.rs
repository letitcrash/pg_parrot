#[derive(Debug, Clone, Copy)]
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
