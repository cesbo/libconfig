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


/// Ini-inspired configuration format
///
/// ## Properties
///
/// Basic element, contains Key-Value pair devide with `=`. Example: `name = Value`
/// White-spaces around line and around delimiter ignores.
/// Each value is a string, without quotes.
///
/// ## Sections
///
/// Section is a group of properties. The section name should be wraped into `[]` symbols.
/// Example: `[section]`. All keys after the section declaration are associated with that section.
/// Sections could be nested. The nested section name should starts with parent section name and
/// separated by `/` symbol. Example: [section/nested]`
///
/// ## Comments
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

    /// Deserialize config
    pub fn parse<R: Read>(src: R) -> Result<Config> {
        let mut line = 0;

        let mut reader = BufReader::new(src);
        let mut buffer = String::new();

        let mut root = Config::new("");
        let mut last = &mut root;

        loop {
            buffer.clear();
            if reader.read_line(&mut buffer)? == 0 {
                break
            }

            line += 1;

            let token = buffer.trim_start();
            if token.is_empty() || token.starts_with(';') {
                continue;
            }

            if token.starts_with('[') {
                /* Config */

                let token = (&token[1 ..]).trim_start(); /* skip [ */
                let end = token.find(']').ok_or_else(|| ConfigError::InvalidFormat(line))?;
                let token = (&token[.. end]).trim_end(); /* ignore ] */

                let mut skip = 0;
                last = &mut root;

                loop {
                    let next = token[skip ..].find('/').map_or(0, |v| v + skip);
                    if next == 0 { break }
                    let item = &token[skip .. next];
                    skip = next + 1;

                    last = last.nested.last_mut()
                        .ok_or_else(|| ConfigError::InvalidKey(line, token.to_owned()))?;

                    if last.name != item {
                        return Err(ConfigError::InvalidKey(line, token.to_owned()));
                    }
                }

                let section = Config {
                    line,
                    name: token[skip ..].to_owned(),
                    properties: Vec::new(),
                    nested: Vec::new(),
                };

                last.nested.push(section);
                last = last.nested.last_mut().unwrap();

                continue;
            }

            let skip = token.find('=')
                .ok_or_else(|| ConfigError::InvalidFormat(line))?;

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
        let file = File::open(path)?;
        Self::parse(file)
    }

    fn dump_section<W: Write>(&self, dst: &mut W, level: &mut String) -> io::Result<()> {
        for p in &self.properties {
            writeln!(dst, "{} = {}", &p.name, &p.value)?;
        }

        if ! self.nested.is_empty() {
            let level_skip = level.len();

            if ! self.name.is_empty() {
                level.push_str(&self.name);
                level.push('/');
            }

            for s in &self.nested {
                writeln!(dst, "\n[{}{}]", level, &s.name)?;
                s.dump_section(dst, level)?;
            }

            level.truncate(level_skip);
        }

        Ok(())
    }

    /// Serializes config
    #[inline]
    pub fn dump<W: Write>(&self, dst: &mut W) -> Result<()> {
        let mut level = String::with_capacity(256);
        self.dump_section(dst, &mut level)?;
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
