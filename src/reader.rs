use std::io::{Read, BufRead, BufReader};

use crate::{IniEvent, Error, Result};

pub struct EventReader<R: Read> {
    reader: BufReader<R>,
    buffer: String,
    line: usize,

    parse_section: bool,
    skip_read: bool,
}

impl<R: Read> EventReader<R> {
    /// Creates a new reader
    #[inline]
    pub fn new(src: R) -> EventReader<R> {
        EventReader {
            reader: BufReader::new(src),
            buffer: String::new(),
            line: 0,
            parse_section: false,
            skip_read: false,
        }
    }

    pub fn next(&mut self) -> Result<IniEvent> {
        let token = loop {
            if self.skip_read {
                self.skip_read = false;
                break self.buffer.trim_start();
            }

            self.buffer.clear();
            let size = self.reader.read_line(&mut self.buffer)?;
            if size == 0 {
                return Ok(IniEvent::EndDocument);
            }
            self.line += 1;

            let token = self.buffer.trim_start();
            if ! (token.len() == 0 || token.starts_with(';')) {
                break token;
            }
        };

        if token.starts_with('[') {
            /* Section */
            if self.parse_section {
                self.parse_section = false;
                self.skip_read = true;
                return Ok(IniEvent::EndSection);
            } else {
                self.parse_section = true;
            }

            let token = (&token[1 ..]).trim_start(); /* skip [ */
            let token = match token.find(']') {
                Some(v) => &token[.. v],
                None => return Err(Error::from((self.line, "Syntax Error: expected ‘]’ after section name"))),
            };
            let token = token.trim_end().to_owned();
            return Ok(IniEvent::StartSection(token));
        }

        let delim = match token.find('=') {
            Some(v) => v,
            None => return Err(Error::from((self.line, "Syntax Error: expected ‘=’ after property name"))),
        };

        let key = (&token[.. delim]).trim_end().to_owned();
        let value = (&token[delim + 1 ..]).trim().to_owned();

        Ok(IniEvent::Property(key, value))
    }
}
