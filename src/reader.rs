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

    pub fn next(&mut self) -> Option<Result<IniEvent>> {
        if self.skip_read {
            self.skip_read = false;
        } else {
            self.buffer.clear();
            match self.reader.read_line(&mut self.buffer) {
                Ok(v) if v > 0 => {},
                Ok(_) => return None,
                Err(e) => return Some(Err(Error::from(e))),
            };
            self.line += 1;
        }

        let token = self.buffer.trim_start();
        if token.len() == 0 || token.starts_with(';') {
            return Some(Ok(IniEvent::Skip));
        }

        if token.starts_with('[') {
            /* Section */
            if self.parse_section {
                self.parse_section = false;
                self.skip_read = true;
                return Some(Ok(IniEvent::EndSection));
            } else {
                self.parse_section = true;
            }

            let token = (&token[1 ..]).trim_start(); /* skip [ */
            let token = match token.find(']') {
                Some(v) => &token[.. v],
                None => return Some(Err(Error::from((self.line, "Syntax Error: expected ‘]’ after section name")))),
            };
            let token = token.trim_end();
            return Some(Ok(IniEvent::StartSection(token)));
        }

        let delim = match token.find('=') {
            Some(v) => v,
            None => return Some(Err(Error::from((self.line, "Syntax Error: expected ‘=’ after property name")))),
        };

        let key = (&token[.. delim]).trim_end();
        let value = (&token[delim + 1 ..]).trim();

        Some(Ok(IniEvent::Property(key, value)))
    }
}
