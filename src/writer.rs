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
            IniEvent::StartSection(ref name) => {
                let name = name.trim_left();
                if name.is_empty() {
                    return Err(Error::from((0, "Syntax Error: missing section name")));
                }

                write!(self.writer, "\n[{}]\n", name)?;
            },
            IniEvent::Property(ref key, ref value) => {
                let value = value.trim_left();
                if value.is_empty() {
                    return Ok(());
                }

                let key = key.trim_left();
                if key.is_empty() {
                    return Err(Error::from((0, "Syntax Error: missing property name")));
                }

                write!(self.writer, "{} = {}\n", key, value)?;
            },
            _ => {},
        };

        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}
