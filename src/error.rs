use std::{fmt, io, result};


#[derive(Debug)]
pub enum Error {
    Format(usize),
    Syntax(usize, &'static str),
    Io(io::Error),
}


pub type Result<T> = result::Result<T, Error>;


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Format(l) => write!(f, "Format Error: line:{}", l),
            Error::Syntax(l, ref e) => write!(f, "Syntax Error: {}. line:{}", e, l),
            Error::Io(ref e) => io::Error::fmt(e, f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
