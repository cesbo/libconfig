use std::io::{Read, BufRead, BufReader, Write, BufWriter};
use std::fs::File;
use std::slice::Iter;
use std::path::Path;

mod error;
pub use crate::error::{Error, Result};


#[derive(Debug, Default)]
pub struct Property {
    line: usize,
    name: String,
    value: String,
}


impl Property {
    pub fn new<S,T>(name: S, value: T) -> Self
    where
        S: Into<String>,
        T: ToString,
    {
        Property {
            line: 0,
            name: name.into(),
            value: value.to_string(),
        }
    }
}


pub trait FromProperty: Sized {
    fn from_property(p: &Property) -> Result<Self>;
}


pub trait SectionPush {
    fn section_push(self, s: &mut Section);
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
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Section {
            line: 0,
            name: name.into(),
            properties: Vec::new(),
            sections: Vec::new(),
        }
    }

    #[inline]
    pub fn push<I>(&mut self, item: I)
    where
        I: SectionPush,
    {
        SectionPush::section_push(item, self);
    }

    #[inline]
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    #[inline]
    fn get_property(&self, name: &str) -> Option<&Property> {
        for p in &self.properties {
            if p.name == name {
                return Some(&p);
            }
        }
        None
    }

    pub fn get_str<'a, T>(&'a self, name: &str, opt: T) -> Result<&'a str>
    where
        T: Into<Option<&'a str>>,
    {
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

    pub fn get_number<F>(&self, name: &str, opt: F) -> Result<F>
    where
        F: FromProperty,
    {
        match self.get_property(name) {
            Some(v) => FromProperty::from_property(v),
            None => Ok(opt),
        }
    }

    pub fn sections<'a>(&'a self) -> SectionIter<'a> {
        SectionIter {
            inner: self.sections.iter()
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

                let mut level = token.find(|c: char| c != '.').unwrap_or(0);
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

    #[inline]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        Self::parse(file)
    }

    fn dump_section<W: Write>(&self, dst: &mut W, level: usize) -> Result<()> {
        if ! self.name.is_empty() {
            writeln!(dst, "\n[{:.>1$}]", &self.name, self.name.len() + level - 1)?;
        }
        for p in &self.properties {
            writeln!(dst, "{} = {}", &p.name, &p.value)?;
        }
        for s in &self.sections {
            s.dump_section(dst, level + 1)?;
        }
        Ok(())
    }

    #[inline]
    pub fn dump<W: Write>(&self, dst: &mut W) -> Result<()> {
        self.dump_section(dst, 0)?;
        Ok(())
    }

    #[inline]
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.dump(&mut writer)?;
        Ok(())
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


impl FromProperty for usize {
    fn from_property(p: &Property) -> Result<usize> {
        match p.value.parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::from(format!("property '{}' line {} has wrong format. value should be in range {} .. {}",
                                              &p.name, p.line, usize::min_value(), usize::max_value()))),
        }
    }
}


impl SectionPush for Property {
    fn section_push(self, s: &mut Section) {
        s.properties.push(self);
    }
}


impl SectionPush for Section {
    fn section_push(self, s: &mut Section) {
        s.sections.push(self);
    }
}


pub struct SectionIter<'a> {
    inner: Iter<'a, Section>,
}


impl<'a> Iterator for SectionIter<'a> {
    type Item = &'a Section;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
