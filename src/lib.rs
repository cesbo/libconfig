mod error;
pub use crate::error::{Error, ErrorKind, Result};

mod reader;
pub use crate::reader::{IniItem, IniReader};

mod writer;
pub use crate::writer::IniWriter;
