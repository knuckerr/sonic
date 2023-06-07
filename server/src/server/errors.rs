use std::fmt;

#[derive(Debug)]
pub enum ServerError {
    StoreLock(String),
    Compression(flate2::CompressError),
    IO(std::io::Error),
}

impl From<std::io::Error> for ServerError {
    fn from(value: std::io::Error) -> Self {
        ServerError::IO(value)
    }
}

impl From<flate2::CompressError> for ServerError {
    fn from(value: flate2::CompressError) -> Self {
        ServerError::Compression(value)
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::StoreLock(error) => write!(f, "{}", error),
            ServerError::Compression(error) => write!(f, "{}", error),
            ServerError::IO(error) => write!(f, "{}", error),
        }
    }
}
