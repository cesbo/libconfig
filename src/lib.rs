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
    pub fn get_line(&self) -> usize {
        self.line
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

    #[inline]
    pub fn get_str<'a>(&'a self, name: &str) -> Option<&'a str> {
        match self.get_property(name) {
            Some(v) => Some(v.value.as_str()),
            None => None,
        }
    }

    #[inline]
    pub fn get_bool(&self, name: &str, opt: bool) -> Result<bool> {
        match self.get_property(name) {
            Some(v) => FromProperty::from_property(v),
            None => Ok(opt),
        }
    }

    #[inline]
    pub fn get_number<F>(&self, name: &str, opt: F) -> Result<F>
    where
        F: FromProperty,
    {
        match self.get_property(name) {
            Some(v) => FromProperty::from_property(v),
            None => Ok(opt),
        }
    }

    #[inline]
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
                let token = match token.find(']') {
                    Some(v) => (&token[.. v]).trim_end(),
                    None => return Err(Error::Syntax(line, "missing ‘]’ after section name")),
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
                        None => return Err(Error::Syntax(line, "wrong section level")),
                    };
                    level -= 1;
                }
                last.sections.push(section);
                last = last.sections.last_mut().unwrap();

                continue;
            }

            let skip = match token.find('=') {
                Some(v) => v,
                None => return Err(Error::Syntax(line, "missing ‘=’ in property declaration")),
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

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.dump(&mut writer)?;
        Ok(())
    }
}


/// A trait to abstract creating a new instance of a type from a Property
pub trait FromProperty: Sized {
    fn from_property(p: &Property) -> Result<Self>;
}


impl FromProperty for bool {
    #[inline]
    fn from_property(p: &Property) -> Result<bool> {
        match p.value.parse() {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::ParseBoolError(p.line, e)),
        }
    }
}


macro_rules! impl_get_number {
    ( $( $t:tt ),* ) => {
        $( impl FromProperty for $t {
            #[inline]
            fn from_property(p: &Property) -> Result<$t> {
                let (skip, radix) = if p.value.starts_with("0x") { (2, 16u32) } else { (0, 10u32) };
                match $t::from_str_radix(&p.value[skip ..], radix) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::ParseIntError(p.line, e)),
                }
            }
        } )*
    };
}


impl_get_number!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);


/// A trait to abstract pushing a new Property or Section into Section
pub trait SectionPush {
    fn section_push(self, s: &mut Section);
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

/// An iterator over the Sections of a Section.
pub struct SectionIter<'a> {
    inner: Iter<'a, Section>,
}


impl<'a> Iterator for SectionIter<'a> {
    type Item = &'a Section;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
