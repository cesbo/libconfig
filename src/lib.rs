use std::io::{Read, BufRead, BufReader, Write, BufWriter};
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
pub struct Config {
    line: usize,
    name: String,
    items: Vec<Property>,
    nested: Vec<Config>,
}


impl Config {
    #[inline]
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Config {
            line: 0,
            name: name.into(),
            items: Vec::new(),
            nested: Vec::new(),
        }
    }

    #[inline]
    pub fn push<I>(&mut self, item: I)
    where
        I: ConfigPush,
    {
        ConfigPush::config_push(item, self);
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
        for p in &self.items {
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
    pub fn iter(&self) -> impl Iterator<Item = &Config> {
        self.nested.iter()
    }

    pub fn parse<R: Read>(src: R) -> Result<Config> {
        let mut line = 0;

        let mut reader = BufReader::new(src);
        let mut buffer = String::new();

        let mut root = Config::default();
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

            if token.starts_with('#') {
                /* Config */
                let level = token.find(|c: char| c != '#').unwrap_or(0);
                let token = (&token[level ..]).trim(); /* skip [ */

                let section = Config {
                    line,
                    name: token.to_owned(),
                    items: Vec::new(),
                    nested: Vec::new(),
                };

                last = &mut root;
                for _ in 1 .. level {
                    last = match last.nested.last_mut() {
                        Some(v) => v,
                        None => return Err(Error::Syntax(line, "wrong section level")),
                    };
                }
                last.nested.push(section);
                last = last.nested.last_mut().unwrap();

                continue;
            }

            let skip = match token.find('=') {
                Some(v) => v,
                None => return Err(Error::Syntax(line, "missing ‘=’ in property declaration")),
            };

            last.items.push(Property {
                line,
                name: (&token[.. skip]).trim_end().to_owned(),
                value: (&token[skip + 1 ..]).trim().to_owned(),
            });
        }

        Ok(root)
    }

    #[inline]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::parse(File::open(path)?)
    }

    fn dump_section<W: Write>(&self, dst: &mut W, level: usize) -> Result<()> {
        if level > 0 {
            writeln!(dst, "\n{0:#>1$} {2}", "", level, &self.name)?;
        }
        for p in &self.items {
            writeln!(dst, "{} = {}", &p.name, &p.value)?;
        }
        for s in &self.nested {
            s.dump_section(dst, level + 1)?;
        }
        Ok(())
    }

    #[inline]
    pub fn dump<W: Write>(&self, dst: W) -> Result<()> {
        let mut writer = BufWriter::new(dst);
        self.dump_section(&mut writer, 0)
    }

    #[inline]
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.dump(File::create(path)?)
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


/// A trait to abstract pushing a new Property or Config into Config
pub trait ConfigPush {
    fn config_push(self, s: &mut Config);
}


impl ConfigPush for Property {
    fn config_push(self, s: &mut Config) {
        s.items.push(self);
    }
}


impl ConfigPush for Config {
    fn config_push(self, s: &mut Config) {
        s.nested.push(self);
    }
}
