use std::io::{
    self,
    Read,
    BufRead,
    BufReader,
    Write,
    BufWriter,
};
use std::fs::File;
use std::path::Path;

use failure::{
    format_err,
    Error,
    Fail,
};


#[derive(Debug, Fail)]
#[fail(display = "Config IO Error: {}", 0)]
struct IoError(io::Error);


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
    pub fn get<F>(&self, name: &str, opt: F) -> Result<F, Error>
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
    pub fn parse<R: Read>(src: R) -> Result<Config, Error> {
        let mut line = 0;

        let mut reader = BufReader::new(src);
        let mut buffer = String::new();

        let mut root = Config::new("");
        let mut last = &mut root;

        loop {
            buffer.clear();
            if reader.read_line(&mut buffer).map_err(|e| IoError(e))? == 0 {
                break
            }

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
                    last = last.nested.last_mut().ok_or_else(||
                        format_err!("Config Error: invalid level for '{}' at line {}", token, line))?;
                }
                last.nested.push(section);
                last = last.nested.last_mut().unwrap();

                continue;
            }

            let skip = token.find('=').ok_or_else(||
                format_err!("Config Error: invalid format at line {}", line))?;

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
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path).map_err(|e| IoError(e))?;
        Self::parse(file)
    }

    fn dump_section<W: Write>(&self, dst: &mut W, level: usize) -> io::Result<()> {
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
    pub fn dump<W: Write>(&self, dst: &mut W) -> Result<(), Error> {
        self.dump_section(dst, 0).map_err(|e| IoError(e))?;
        Ok(())
    }

    /// Saves config into file
    #[inline]
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let file = File::create(path).map_err(|e| IoError(e))?;
        let mut writer = BufWriter::new(file);
        self.dump(&mut writer)
    }
}


/// A trait to abstract creating a new instance of a type from a Property
pub trait FromProperty: Sized {
    fn from_property(p: &Property) -> Result<Self, Error>;
}


#[derive(Debug, Fail)]
#[fail(display = "Config Error: invalid property '{}' at line {}", 1, 0)]
struct FromPropertyError(usize, String);


impl FromProperty for bool {
    #[inline]
    fn from_property(p: &Property) -> Result<bool, Error> {
        let value = p.value.parse::<bool>()
            .map_err(|_| FromPropertyError(p.line, p.name.to_owned()))?;
        Ok(value)
    }
}


macro_rules! impl_get_number {
    ( $( $t:tt ),* ) => {
        $( impl FromProperty for $t {
            #[inline]
            fn from_property(p: &Property) -> Result<$t, Error> {
                let (skip, radix) = if p.value.starts_with("0x") { (2, 16u32) } else { (0, 10u32) };
                let value = $t::from_str_radix(&p.value[skip ..], radix)
                    .map_err(|_| FromPropertyError(p.line, p.name.to_owned()))?;
                Ok(value)
            }
        } )*
    };
}


impl_get_number!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);
