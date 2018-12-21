use std::io::{Write};

use crate::{IniEvent, Error, Result};

pub struct EventWriter<W: Write> {
    writer: W,
}

impl<W: Write> EventWriter<W> {
    /// Creates a new reader
    #[inline]
    pub fn new(dst: W) -> EventWriter<W> {
        EventWriter {
            writer: dst,
        }
    }

    pub fn write(&mut self, event: IniEvent) -> Result<()> {
        match event {
            IniEvent::StartSection(ref name) => write!(self.writer, "\n[{}]\n", name)?,
            IniEvent::Property(ref key, ref value) => write!(self.writer, "{} = {}\n", key, value)?,
            _ => {},
        };
        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}
