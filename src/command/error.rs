use std::fmt;

pub type Result<T> = std::result::Result<T, Box<Error>>;

#[derive(Debug)]
pub enum Error {
    InvalidCommand,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error")
    }
}
