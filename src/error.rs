use std::error::Error;
use std::fmt;

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct SimpleError {
    pub msg: String,
}

impl SimpleError {
    pub fn new(msg: &str) -> SimpleError {
        SimpleError {
            msg: msg.to_string(),
        }
    }
}

impl Error for SimpleError {}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug)]
pub struct BreakError;

impl Error for BreakError {}

impl fmt::Display for BreakError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "break error")
    }
}
