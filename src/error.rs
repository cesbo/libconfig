use std::{error, fmt, io, result};
use std::borrow::Cow;

#[inline]
fn std_error_description(e: &error::Error) -> &str {
    e.description()
}

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
        write!(f, "line:{} {}", self.line, self.msg())
    }
}

impl Error {
    /// Returns a reference to a message which is contained inside this error.
    #[inline]
    pub fn msg(&self) -> &str {
        match self.kind {
            ErrorKind::Syntax(ref msg) => msg.as_ref(),
            ErrorKind::Io(ref e) => std_error_description(e),
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
