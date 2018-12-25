use std::fmt;
use std::io::{Read, BufRead, BufReader};
use std::iter::Iterator;

use crate::{Error, Result};

pub enum IniItem {
    /// Beginning of the INI section. Contain unescaped section name
    StartSection(String),
    /// End of the INI section
    EndSection,
    /// Key-Value pair
    Property(String, String),
}

impl fmt::Debug for IniItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IniItem::Property(ref key, ref value) => write!(f, "Property({}, {})", key, value),
            IniItem::StartSection(ref name) => write!(f, "StartSection({})", name),
            IniItem::EndSection => write!(f, "EndSection"),
        }
    }
}

pub struct IniReader<R: Read> {
    reader: BufReader<R>,
    buffer: String,
    line: usize,

    parse_section: bool,
    skip_read: bool,
}

impl<R: Read> IniReader<R> {
    /// Creates a new reader
    #[inline]
    pub fn new(src: R) -> IniReader<R> {
        IniReader {
            reader: BufReader::new(src),
            buffer: String::new(),
            line: 0,
            parse_section: false,
            skip_read: false,
        }
    }
}

impl<R: Read> Iterator for IniReader<R> {
    type Item = Result<IniItem>;

    fn next(&mut self) -> Option<Result<IniItem>> {
        let token = loop {
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
            if token.is_empty() || token.starts_with(';') {
                continue;
            }
            break token;
        };

        if token.starts_with('[') {
            /* Section */
            if self.parse_section {
                self.parse_section = false;
                self.skip_read = true;
                return Some(Ok(IniItem::EndSection));
            } else {
                self.parse_section = true;
            }

            let token = (&token[1 ..]).trim_start(); /* skip [ */
            let token = match token.find(']') {
                Some(v) => &token[.. v],
                None => return Some(Err(Error::from((self.line, "Syntax Error: expected ‘]’ after section name")))),
            };
            let token = token.trim_end().to_owned();
            return Some(Ok(IniItem::StartSection(token)));
        }

        let delim = match token.find('=') {
            Some(v) => v,
            None => return Some(Err(Error::from((self.line, "Syntax Error: expected ‘=’ after property name")))),
        };

        let key = (&token[.. delim]).trim_end().to_owned();
        let value = (&token[delim + 1 ..]).trim().to_owned();

        Some(Ok(IniItem::Property(key, value)))
    }
}
