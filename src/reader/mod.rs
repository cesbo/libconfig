use std::{fmt, result};
use std::io::{Read, BufRead, BufReader};

mod error;
pub use self::error::{Error, ErrorKind};

pub type Result<T> = result::Result<T, Error>;

pub enum IniEvent<'a> {
    /// Beginning of the INI section. Contain unescaped section name
    StartSection(&'a str),
    /// End of the INI section.
    EndSection,
    /// Key-Value pair.
    Key(&'a str, &'a str),
    /// End of the INI document.
    EndDocument,
}

impl<'a> fmt::Debug for IniEvent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IniEvent::Key(ref key, ref value) => write!(f, "Key({}, {})", key, value),
            IniEvent::StartSection(ref name) => write!(f, "StartSection({})", name),
            IniEvent::EndSection => write!(f, "EndSection"),
            IniEvent::EndDocument => write!(f, "EndDocument"),
        }
    }
}

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
    pub fn new(source: R) -> EventReader<R> {
        EventReader {
            reader: BufReader::new(source),
            buffer: String::new(),
            line: 0,
            parse_section: false,
            skip_read: false,
        }
    }

    #[inline]
    fn read_token(&mut self) -> Result<usize> {
        self.buffer.clear();
        let size = self.reader.read_line(&mut self.buffer)?;
        Ok(size)
    }

    #[inline]
    fn check_token(&mut self) -> bool {
        let token = self.buffer.as_str().trim_start();
        ! (token.len() == 0 || token.starts_with(';'))
    }

    fn parse_token(&mut self) -> Result<IniEvent> {
        let token = self.buffer.as_str();

        if token.starts_with('[') {
            /* Section */
            if self.parse_section {
                self.parse_section = false;
                self.skip_read = true;
                return Ok(IniEvent::EndSection);
            } else {
                self.parse_section = true;
            }

            let token = &token[1 ..]; /* skip [ */
            let token = token.trim_start();
            let token = match token.find(']') {
                Some(v) => &token[.. v],
                None => return Err(Error::from((self.line, "wrong section format"))),
            };
            let token = token.trim_end();
            return Ok(IniEvent::StartSection(token));
        }

        // TODO: continue here...
        Ok(IniEvent::Key("", token))
    }

    /// Pulls and returns next INI event from the stream
    pub fn next(&mut self) -> Result<IniEvent> {
        loop {
            if ! self.skip_read {
                let size = self.read_token()?;
                if size == 0 {
                    return Ok(IniEvent::EndDocument);
                }
                self.line += 1;

                if ! self.check_token() {
                    continue;
                }
            } else {
                self.skip_read = false;
            }

            return self.parse_token();
        }
    }
}
