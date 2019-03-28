use std::{
    io,
    result,
};
use std::fmt::{
    self,
    Display,
    Formatter,
};
use std::num::ParseIntError;
use std::str::ParseBoolError;


#[derive(Debug)]
pub enum Error {
    Syntax(usize, String),
    ParseBoolError(usize, ParseBoolError),
    ParseIntError(usize, ParseIntError),
    Io(io::Error),
}


pub type Result<T> = result::Result<T, Error>;


impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Error::Syntax(line, ref e) => write!(f, "Syntax Error at line {}: {}", line, e),
            Error::ParseBoolError(line, ref e) => write!(f, "Format Error at line {}: {}", line, e),
            Error::ParseIntError(line, ref e) => write!(f, "Format Error at line {}: {}", line, e),
            Error::Io(ref e) => io::Error::fmt(e, f),
        }
    }
}


impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
