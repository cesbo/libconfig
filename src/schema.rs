use std::{
    fmt::{
        self,
        Write,
    },
    ops::Range,
};

use crate::config::{
    Config,
    ConfigError,
    Result,
};


pub struct Validator(Option<Box<Fn(&str) -> bool>>);


struct Property {
    name: String,
    description: String,
    required: bool,
    validator: Validator,
}


/// Scheme for validating the configuration file.
pub struct Schema {
    name: String,
    description: String,
    properties: Vec<Property>,
    nested: Vec<Schema>,
}


impl From<Option<Box<Fn(&str) -> bool>>> for Validator {
    #[inline]
    fn from(f: Option<Box<Fn(&str) -> bool>>) -> Validator {
        Validator(f)
    }
}


impl<F: 'static> From<F> for Validator
where
    F: Fn(&str) -> bool,
{
    #[inline]
    fn from(f: F) -> Validator {
        Validator(Some(Box::new(f)))
    }
}


impl Schema {
    /// Creates new schema
    ///
    /// - `name` - section name
    /// - `description` - section description
    pub fn new<S>(name: S, description: S) -> Self
    where
        S: Into<String>,
    {
        Schema {
            name: name.into(),
            description: description.into(),
            properties: Vec::new(),
            nested: Vec::new(),
        }
    }

    /// Appends information about schema parameter
    ///
    /// - `name` - config parameter name
    /// - `description` - parameter description
    /// - `required` - is parameter required
    /// - `validator` - validator function or `None`
    pub fn set<S, B>(&mut self, name: S, description: S, required: bool, validator: B)
    where
        S: Into<String>,
        B: Into<Validator>,
    {
        let property = Property {
            name: name.into(),
            description: description.into(),
            required,
            validator: validator.into(),
        };
        self.properties.push(property);
    }

    /// Appends nested schema
    #[inline]
    pub fn push(&mut self, nested: Schema) { self.nested.push(nested) }

    fn get_nested(&self, name: &str) -> Option<&Schema> {
        for schema in &self.nested {
            if schema.name == name {
                return Some(schema)
            }
        }
        None
    }

    /// Validates config with schema
    pub fn check(&self, config: &Config) ->  Result<()> {
        for item in &self.properties {
            if let Some(property) = config.get_property(&item.name) {
                if let Some(validator) = &item.validator.0 {
                    if ! validator(&property.get_value()) {
                        return Err(ConfigError::InvalidProperty(property.get_line(), item.name.to_owned()));
                    }
                }
            } else if item.required {
                return Err(ConfigError::MissingProperty(config.get_line(), item.name.to_owned()));
            }
        }

        for config in config.iter() {
            if let Some(schema) = self.get_nested(config.get_name()) {
                schema.check(config)?;
            }
        }

        Ok(())
    }

    fn info_section(&self, result: &mut String, level: &mut String) -> fmt::Result {
        if ! self.description.is_empty() {
            writeln!(result, "; {}", self.description)?;
        }

        for item in &self.properties {
            writeln!(result, "{} = {}", &item.name, &item.description)?;
        }

        if ! self.nested.is_empty() {
            let level_skip = level.len();

            if ! self.name.is_empty() {
                level.push_str(&self.name);
                level.push('/');
            }

            for s in &self.nested {
                writeln!(result, "\n[{}{}]", level, &s.name)?;
                s.info_section(result, level)?;
            }

            level.truncate(level_skip);
        }

        Ok(())
    }

    /// Returns information about schema parameters and nested schemas
    pub fn info(&mut self) -> String {
        let mut level = String::with_capacity(256);
        let mut result = String::new();
        self.info_section(&mut result, &mut level).unwrap();
        result
    }

    /// Range validator
    pub fn range(r: Range<usize>) -> impl Fn(&str) -> bool {
        move |s: &str| -> bool {
            let (skip, radix) = if s.starts_with("0x") { (2, 16u32) } else { (0, 10u32) };
            match usize::from_str_radix(&s[skip ..], radix) {
                Ok(v) => (v >= r.start) && (v <= r.end),
                _ => false,
            }
        }
    }
}
