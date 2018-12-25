use std::io::{Write};

use crate::{Error, Result};

pub struct IniWriter<W: Write> {
    writer: W,
}

impl<W: Write> IniWriter<W> {
    /// Creates a new reader
    #[inline]
    pub fn new(dst: W) -> IniWriter<W> {
        IniWriter {
            writer: dst,
        }
    }

    pub fn write_section(&mut self, name: &str) -> Result<()> {
        let name = name.trim();
        if name.is_empty() {
            return Err(Error::from((0, "Syntax Error: missing section name")));
        }

        writeln!(self.writer, "\n[{}]", name)?;
        Ok(())
    }

    pub fn write_property(&mut self, key: &str, value: &str) -> Result<()> {
        if value.is_empty() {
            return Ok(());
        }

        let key = key.trim_left();
        if key.is_empty() {
            return Err(Error::from((0, "Syntax Error: missing property name")));
        }

        writeln!(self.writer, "{} = {}", key, value)?;
        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}
