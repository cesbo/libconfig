use std::io::{
    Read,
    BufRead,
    BufReader,
    Write,
    BufWriter,
};
use std::fs::File;
use std::path::Path;

use crate::error::{
    Error,
    Result,
};


pub struct Property {
    line: usize,
    name: String,
    value: String,
}


impl Property {
    #[inline]
    pub fn get_line(&self) -> usize {
        self.line
    }

    #[inline]
    pub fn get_value(&self) -> &str {
        self.value.as_str()
    }
}


/// Ini-inspired configuration format
///
/// # Properties
///
/// Basic element, contains Key-Value pair devide with `=`. Example: `name = Value`
/// White-spaces around line and around delimiter ignores.
/// Each value is a string, without quotes.
///
/// # Sections
///
/// Section is a group of properties. The section name should be started with `#`. Example: `# section`
/// All keys after the section declaration are associated with that section. Sections could be nested.
/// The nested section name should containg one more `#` in the name. Example: `## nested`
///
/// # Comments
///
/// Comment line should be started with `;`. Example: `; comment`
///
pub struct Config {
    line: usize,
    name: String,
    properties: Vec<Property>,
    nested: Vec<Config>,
}


impl Config {
    /// Creates new empty config with `name` (section name meaning)
    #[inline]
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Config {
            line: 0,
            name: name.into(),
            properties: Vec::new(),
            nested: Vec::new(),
        }
    }

    /// Appends config property
    pub fn set<S, T>(&mut self, name: S, value: T)
    where
        S: Into<String>,
        T: ToString,
    {
        let property = Property {
            line: 0,
            name: name.into(),
            value: value.to_string(),
        };
        self.properties.push(property);
    }

    /// Appends nested config
    #[inline]
    pub fn push(&mut self, nested: Config) {
        self.nested.push(nested);
    }

    /// Returns section name
    #[inline]
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns section line number
    #[inline]
    pub fn get_line(&self) -> usize {
        self.line
    }

    /// Returns property
    #[inline]
    pub fn get_property(&self, name: &str) -> Option<&Property> {
        for p in &self.properties {
            if p.name == name {
                return Some(&p);
            }
        }
        None
    }

    /// Returns property string value
    #[inline]
    pub fn get_str<'a>(&'a self, name: &str) -> Option<&'a str> {
        match self.get_property(name) {
            Some(v) => Some(v.value.as_str()),
            None => None,
        }
    }

    /// Returns property typed value (boolean or numbers)
    #[inline]
    pub fn get<F>(&self, name: &str, opt: F) -> Result<F>
    where
        F: FromProperty,
    {
        match self.get_property(name) {
            Some(v) => FromProperty::from_property(v),
            None => Ok(opt),
        }
    }

    /// Returns nested sections iterator
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Config> {
        self.nested.iter()
    }

    /// Deserialize config
    pub fn parse<R: Read>(src: R) -> Result<Config> {
        let mut line = 0;

        let mut reader = BufReader::new(src);
        let mut buffer = String::new();

        let mut root = Config::new("");
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
                    properties: Vec::new(),
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

            last.properties.push(Property {
                line,
                name: (&token[.. skip]).trim_end().to_owned(),
                value: (&token[skip + 1 ..]).trim().to_owned(),
            });
        }

        Ok(root)
    }

    /// Opens config file
    #[inline]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::parse(File::open(path)?)
    }

    fn dump_section<W: Write>(&self, dst: &mut W, level: usize) -> Result<()> {
        if level > 0 {
            writeln!(dst, "\n{0:#>1$} {2}", "", level, &self.name)?;
        }
        for p in &self.properties {
            writeln!(dst, "{} = {}", &p.name, &p.value)?;
        }
        for s in &self.nested {
            s.dump_section(dst, level + 1)?;
        }
        Ok(())
    }

    /// Serializes config
    #[inline]
    pub fn dump<W: Write>(&self, dst: W) -> Result<()> {
        let mut writer = BufWriter::new(dst);
        self.dump_section(&mut writer, 0)
    }

    /// Saves config into file
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
