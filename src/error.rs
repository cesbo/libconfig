use std::{fmt, io, result};


#[derive(Debug)]
pub enum Error {
    Syntax(String),
    Io(io::Error),
}


pub type Result<T> = result::Result<T, Error>;


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Syntax(ref e) => write!(f, "{}", e),
            Error::Io(ref e) => io::Error::fmt(e, f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Syntax(s.to_owned())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Syntax(s)
    }
}
