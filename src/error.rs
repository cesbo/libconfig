use std::{fmt, io, result};
use std::borrow::Cow;

#[derive(Debug)]
pub enum ErrorKind {
    Syntax(Cow<'static, str>),
    Io(io::Error),
}

#[derive(Debug)]
pub struct Error {
    line: usize,
    kind: ErrorKind,
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Syntax(ref msg) => write!(f, "line:{} {}", self.line, msg),
            ErrorKind::Io(ref e) => io::Error::fmt(e, f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            line: 0,
            kind: ErrorKind::Io(e),
        }
    }
}

impl<M> From<(usize, M)> for Error
    where M: Into<Cow<'static, str>>
{
    fn from(orig: (usize, M)) -> Self {
        Error{
            line: orig.0,
            kind: ErrorKind::Syntax(orig.1.into())
        }
    }
}
