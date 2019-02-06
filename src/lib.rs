use std::io::{Read, BufRead, BufReader};
use std::fs::File;
use std::path::Path;

mod error;
pub use crate::error::{Error, Result};


#[derive(Debug, Default)]
pub struct Property {
    line: usize,
    name: String,
    value: String,
}


pub trait FromProperty: Sized {
    fn from_property(p: &Property) -> Result<Self>;
}


#[derive(Debug, Default)]
pub struct Section {
    line: usize,
    name: String,
    properties: Vec<Property>,
    sections: Vec<Section>,
}


impl Section {
    #[inline]
    fn get_property(&self, name: &str) -> Option<&Property> {
        for p in &self.properties {
            if p.name == name {
                return Some(&p);
            }
        }
        None
    }

    pub fn get_str<'a, T: Into<Option<&'a str>>>(&'a self, name: &str, opt: T) -> Result<&'a str> {
        match self.get_property(name) {
            Some(v) => Ok(v.value.as_str()),
            None => match opt.into() {
                Some(v) => Ok(v),
                None => Err(Error::from(format!("property '{}' not found in section '{}' line {}", name, &self.name, self.line))),
            },
        }
    }

    pub fn get_bool(&self, name: &str, opt: bool) -> Result<bool> {
        match self.get_property(name) {
            Some(v) => FromProperty::from_property(v),
            None => Ok(opt),
        }
    }

    pub fn get_number<T: FromProperty>(&self, name: &str, opt: T) -> Result<T> {
        match self.get_property(name) {
            Some(v) => FromProperty::from_property(v),
            None => Ok(opt),
        }
    }

    pub fn parse<R: Read>(src: R) -> Result<Section> {
        let mut line = 0;

        let mut reader = BufReader::new(src);
        let mut buffer = String::new();

        let mut root = Section::default();
        let mut last = &mut root;

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
                // TODO: check ‘.’
                let token = match token.find(']') {
                    Some(v) => (&token[.. v]).trim_end(),
                    None => return Err(Error::from("Syntax Error: expected ‘]’ after section name")),
                };

                let mut level = token.find(|c: char| c != '>').unwrap_or(0);
                let section = Section {
                    line,
                    name: (&token[level ..]).trim_start().to_owned(),
                    properties: Vec::new(),
                    sections: Vec::new(),
                };

                last = &mut root;
                while level > 0 {
                    last = match last.sections.last_mut() {
                        Some(v) => v,
                        None => return Err(Error::from("Syntax Error: wrong section level")),
                    };
                    level -= 1;
                }
                last.sections.push(section);
                last = last.sections.last_mut().unwrap();

                continue;
            }

            let skip = match token.find('=') {
                Some(v) => v,
                None => return Err(Error::from("Syntax Error: expected ‘=’ after property name")),
            };

            last.properties.push(Property {
                line,
                name: (&token[.. skip]).trim_end().to_owned(),
                value: (&token[skip + 1 ..]).trim().to_owned(),
            });
        }

        Ok(root)
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        Self::parse(file)
    }
}


impl FromProperty for bool {
    fn from_property(p: &Property) -> Result<bool> {
        match p.value.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(Error::from(format!("property '{}' line {} has wrong format. value should be ‘true’ or ‘false’",
                                         &p.name, p.line))),
        }
    }
}


impl FromProperty for u8 {
    fn from_property(p: &Property) -> Result<u8> {
        match p.value.parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::from(format!("property '{}' line {} has wrong format. value should be in range {} .. {}",
                                              &p.name, p.line, u8::min_value(), u8::max_value()))),
        }
    }
}


impl FromProperty for u16 {
    fn from_property(p: &Property) -> Result<u16> {
        match p.value.parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::from(format!("property '{}' line {} has wrong format. value should be in range {} .. {}",
                                              &p.name, p.line, u16::min_value(), u16::max_value()))),
        }
    }
}


impl FromProperty for u32 {
    fn from_property(p: &Property) -> Result<u32> {
        match p.value.parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::from(format!("property '{}' line {} has wrong format. value should be in range {} .. {}",
                                              &p.name, p.line, u32::min_value(), u32::max_value()))),
        }
    }
}


impl FromProperty for i32 {
    fn from_property(p: &Property) -> Result<i32> {
        match p.value.parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::from(format!("property '{}' line {} has wrong format. value should be in range {} .. {}",
                                              &p.name, p.line, i32::min_value(), i32::max_value()))),
        }
    }
}


/*
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
        if ! section.is_empty() {
            self.0.push((name.into(), section));
        }
    }

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
*/
