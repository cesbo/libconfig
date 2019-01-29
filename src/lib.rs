mod error;
pub use crate::error::{Error, ErrorKind, Result};

mod reader;
pub use crate::reader::{IniReader, Section};

mod writer;
pub use crate::writer::IniWriter;
