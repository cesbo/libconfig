use std::collections::HashMap;

use crate::config::Config;
use crate::error::{
    Error,
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
    nested: HashMap<String, Schema>,
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
            nested: HashMap::new(),
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
    pub fn push(&mut self, nested: Schema) {
        self.nested.insert(nested.name.clone(), nested);
    }

    /// Validates config with schema
    pub fn check(&self, config: &Config) ->  Result<()> {
        for item in &self.properties {
            if let Some(property) = config.get_property(&item.name) {
                if let Some(validator) = &item.validator.0 {
                    if ! validator(&property.get_value()) {
                        return Err(Error::Syntax(property.get_line(), "problem whith check parametr"));
                    }
                }
            } else if item.required {
                return Err(Error::Syntax(config.get_line(), "missing required config parametr"));
            }
        }
        for config in config.iter() {
            if let Some(schema) = self.nested.get(config.get_name()) {
                schema.check(config)?;
            }
        }
        Ok(())
    }

    /// Returns information about schema parameters and nested schemas
    pub fn info(&mut self) -> String {
        let mut result = String::new();
        self.info_section(&mut result, 0);
        result
    }

    fn info_section(&self, result: &mut String, level: usize) {
        if level > 0 {
            result.push_str(&format!("\n{0:#>1$} {2}\n", "", level, self.name));
            if ! self.description.is_empty() {
                result.push_str(&format!("; {}\n", self.description));
            }
        }
        for item in &self.properties {
            result.push_str(&format!("{} - {}\n", &item.name, &item.description));
        }
        for schema in self.nested.values() {
            schema.info_section(result, level + 1);
        }
    }
}
