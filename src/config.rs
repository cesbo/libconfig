use std::{
    fs::File,
    path::Path,
    io::{
        self,
        Read,
        BufRead,
        BufReader,
        Write,
        BufWriter,
    },
};


#[derive(Debug, Error)]
#[error_prefix = "Config"]
pub enum ConfigError {
    #[error_from]
    Io(io::Error),
    #[error_kind("invalid key '{}' at line {}", 1, 0)]
    InvalidKey(usize, String),
    #[error_kind("invalid property '{}' at line {}", 1, 0)]
    InvalidProperty(usize, String),
    #[error_kind("invalid format at line {}", 0)]
    InvalidFormat(usize),
    #[error_kind("missing required property '{}' at line {}", 1, 0)]
    MissingProperty(usize, String),
}


pub type Result<T> = std::result::Result<T, ConfigError>;


pub struct Property {
    line: usize,
    name: String,
    value: String,
}


impl Property {
    #[inline]
    pub fn get_line(&self) -> usize { self.line }

    #[inline]
    pub fn get_value(&self) -> &str { self.value.as_str() }
}


/// Configuration format
///
/// ## Properties
///
/// Basic element, contains Key-Value pair devide with `=`. Example: `name = Value`
/// White-spaces around line and around delimiter ignores.
/// Each value is a string, without quotes.
///
/// ## Sections
///
/// Section is a group of properties.
/// Example: `section { ... }`.
/// Sections could be nested.
///
/// ## Comments
///
/// Comment line should be started with `#`. Example: `# comment`
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
    pub fn push(&mut self, nested: Config) { self.nested.push(nested) }

    /// Returns section name
    #[inline]
    pub fn get_name(&self) -> &str { self.name.as_str() }

    /// Returns section line number
    #[inline]
    pub fn get_line(&self) -> usize { self.line }

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

    /// Returns property typed value (boolean or numbers)
    #[inline]
    pub fn get<'a, F>(&'a self, name: &str) -> Option<F>
    where
        F: FromProperty<'a>,
    {
        self.get_property(name)
            .and_then(|v| FromProperty::from_property(v).ok())
    }

    /// Returns nested sections iterator
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Config> { self.nested.iter() }

    fn parse_section<R: BufRead>(src: &mut R, buffer: &mut String, line: &mut usize) -> Result<Config> {
        let mut section = Config::new("");

        loop {
            buffer.clear();

            if src.read_line(buffer)? == 0 {
                break
            }

            *line += 1;

            let token = buffer.trim_start();

            if token.is_empty() || token.starts_with('#') {
                continue;
            }

            if token.starts_with('}') {
                break;
            }

            let skip = token.find(char::is_whitespace)
                .ok_or_else(|| ConfigError::InvalidFormat(*line))?;

            let name = token[.. skip].to_string();
            let token = token[skip ..].trim_start();

            if token.starts_with('=') {
                section.properties.push(Property {
                    line: *line,
                    name,
                    value: (&token[1 ..]).trim().to_string(),
                });
            }

            else if token.starts_with('{') {
                let mut s = Config::parse_section(src, buffer, line)?;
                s.name = name;
                section.nested.push(s);
            }

            else {
                return Err(ConfigError::InvalidFormat(*line));
            }
        }

        Ok(section)
    }

    /// Deserialize config
    pub fn parse<R: Read>(src: R) -> Result<Config> {
        let mut line = 0;

        let mut reader = BufReader::new(src);
        let mut buffer = String::new();

        Config::parse_section(&mut reader, &mut buffer, &mut line)
    }

    /// Opens config file
    #[inline]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        Self::parse(file)
    }

    fn dump_section<W: Write>(&self, dst: &mut W, level: usize) -> io::Result<()> {
        for p in &self.properties {
            writeln!(dst, "{:level$}{} = {}", "", &p.name, &p.value, level = level)?;
        }

        for s in &self.nested {
            writeln!(dst, "\n{:level$}{} {{", "", &s.name, level = level)?;
            s.dump_section(dst, level + 4)?;
            writeln!(dst, "{:level$}}}", "", level = level)?;
        }

        Ok(())
    }

    /// Serializes config
    #[inline]
    pub fn dump<W: Write>(&self, dst: &mut W) -> Result<()> {
        self.dump_section(dst, 0)?;
        Ok(())
    }

    /// Saves config into file
    #[inline]
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.dump(&mut writer)
    }
}


/// A trait to abstract creating a new instance of a type from a Property
pub trait FromProperty<'a>: Sized {
    fn from_property(p: &'a Property) -> Result<Self>;
}


impl<'a> FromProperty<'a> for &'a str {
    #[inline]
    fn from_property(p: &'a Property) -> Result<&'a str> {
        Ok(p.value.as_str())
    }
}


impl<'a> FromProperty<'a> for bool {
    #[inline]
    fn from_property(p: &'a Property) -> Result<bool> {
        let value = p.value.parse::<bool>()
            .map_err(|_| ConfigError::InvalidProperty(p.line, p.name.to_owned()))?;
        Ok(value)
    }
}


macro_rules! impl_get_number {
    ( $( $t:tt ),* ) => {
        $( impl<'a> FromProperty<'a> for $t {
            #[inline]
            fn from_property(p: &'a Property) -> Result<$t> {
                let (skip, radix) = if p.value.starts_with("0x") { (2, 16u32) } else { (0, 10u32) };
                let value = $t::from_str_radix(&p.value[skip ..], radix)
                    .map_err(|_| ConfigError::InvalidProperty(p.line, p.name.to_owned()))?;
                Ok(value)
            }
        } )*
    };
}


impl_get_number!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);
