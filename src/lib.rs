use std::io::{Read, BufRead, BufReader, Write, BufWriter};
use std::fs::File;
use std::path::Path;

mod error;
pub use crate::error::{Error, ErrorKind, Result};


#[derive(Debug, Default)]
pub struct Section(Vec<(String, String)>);


impl Section {
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn push<S>(&mut self, key: S, value: S)
    where
        S: Into<String>,
    {
        self.0.push((key.into(), value.into()));
    }
}


impl IntoIterator for Section {
    type Item = (String, String);
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}


impl<'a> IntoIterator for &'a Section {
    type Item = &'a (String, String);
    type IntoIter = ::std::slice::Iter<'a, (String, String)>;

    #[inline]
    fn into_iter(self) -> ::std::slice::Iter<'a, (String, String)> {
        (&self.0).into_iter()
    }
}


#[derive(Debug, Default)]
pub struct Ini(Vec<(String, Section)>);


impl Ini {
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn push<S>(&mut self, name: S, section: Section)
    where
        S: Into<String>,
    {
        self.0.push((name.into(), section));
    }
}


impl IntoIterator for Ini {
    type Item = (String, Section);
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}


impl<'a> IntoIterator for &'a Ini {
    type Item = &'a (String, Section);
    type IntoIter = ::std::slice::Iter<'a, (String, Section)>;

    #[inline]
    fn into_iter(self) -> ::std::slice::Iter<'a, (String, Section)> {
        (&self.0).into_iter()
    }
}


impl Ini {
    pub fn parse<R: Read>(src: R) -> Result<Self> {
        let mut ini = Ini::default();

        let mut line = 0;
        let mut reader = BufReader::new(src);
        let mut buffer = String::new();

        let mut name = String::new();
        let mut section = Section::default();

        loop {
            buffer.clear();
            match reader.read_line(&mut buffer) {
                Ok(v) => if v == 0 { break },
                Err(e) => return Err(Error::from(e)),
            };
            line += 1;

            let token = buffer.trim_start();
            if token.is_empty() || token.starts_with(';') {
                continue;
            }

            if token.starts_with('[') {
                /* Section */
                let token = (&token[1 ..]).trim_start(); /* skip [ */
                let token = match token.find(']') {
                    Some(v) => (&token[.. v]).trim_end(),
                    None => return Err(Error::from((line, "Syntax Error: expected ‘]’ after section name"))),
                };

                ini.push(name, section);
                name = String::from(token);
                section = Section::default();
                continue;
            }

            let skip = match token.find('=') {
                Some(v) => v,
                None => return Err(Error::from((line, "Syntax Error: expected ‘=’ after property name"))),
            };

            let key = (&token[.. skip]).trim_end();
            let value = (&token[skip + 1 ..]).trim();
            section.push(key, value);
        }

        ini.push(name, section);
        Ok(ini)
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        Self::parse(file)
    }

    pub fn dump<W: Write>(&self, mut dst: W) -> Result<()> {
        for (name, section) in self {
            if ! name.is_empty() {
                writeln!(dst, "\n[{}]", name)?;
            }

            for (key, value) in section {
                writeln!(dst, "{} = {}", key, value)?;
            }
        }

        Ok(())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.dump(&mut writer)?;
        Ok(())
    }
}
