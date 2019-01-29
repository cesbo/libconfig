use std::io::{Read, BufRead, BufReader};
use std::fs::File;


use crate::{Error, Result};


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
pub struct IniReader(Vec<(String, Section)>);


impl IniReader {
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}


impl IntoIterator for IniReader {
    type Item = (String, Section);
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}


impl<'a> IntoIterator for &'a IniReader {
    type Item = &'a (String, Section);
    type IntoIter = ::std::slice::Iter<'a, (String, Section)>;

    #[inline]
    fn into_iter(self) -> ::std::slice::Iter<'a, (String, Section)> {
        (&self.0).into_iter()
    }
}


impl IniReader {
    pub fn parse<R: Read>(src: R) -> Result<Self> {
        let mut ini = IniReader::default();

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

                ini.0.push((name, section));
                name = String::from(token);
                section = Section::default();
                continue;
            }

            let skip = match token.find('=') {
                Some(v) => v,
                None => return Err(Error::from((line, "Syntax Error: expected ‘=’ after property name"))),
            };

            let key = (&token[.. skip]).trim_end().to_owned();
            let value = (&token[skip + 1 ..]).trim().to_owned();
            section.0.push((key, value));
        }

        ini.0.push((name, section));
        Ok(ini)
    }

    pub fn open(path: &str) -> Result<Self> {
        let f = File::open(path)?;
        Self::parse(f)
    }
}
