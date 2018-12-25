mod error;
pub use crate::error::{Error, ErrorKind, Result};

mod event;
pub use crate::event::IniEvent;

mod reader;
pub use crate::reader::EventReader;

mod writer;
pub use crate::writer::IniWriter;
