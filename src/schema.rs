use std::collections::HashMap;

use crate::config::Config;
use crate::error::{
    Error,
    Result,
};


pub struct Validator(Option<Box<Fn(&str) -> bool>>);


struct Param {
    name: String,
    description: String,
    required: bool,
    validator: Validator,
}


pub struct Schema {
    name: String,
    description: String,
    params: Vec<Param>,
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
    #[inline]
    pub fn new<S>(name: S, description: S) -> Self 
    where
        S: Into<String>,
    {
        Schema {
            name: name.into(),
            description: description.into(),
            params: Vec::new(),
            nested: HashMap::new(),
        }
    }
        
    #[inline]
    pub fn set<S, B>(&mut self, name: S, description: S, required: bool, validator: B)
    where
        S: Into<String>,
        B: Into<Validator>, 
    {
        let param = Param {
            name: name.into(),
            description: description.into(),
            required,
            validator: validator.into(),
        };
        self.params.push(param);
    }
    
    #[inline]
    pub fn push(&mut self, nested: Schema) {
        self.nested.insert(nested.name.clone(), nested);
    }
        
    pub fn check(&self, config: &Config) ->  Result<()> {
        for param in self.params.iter() {
            if let Some(value) = config.get_str(&param.name) {
                if let Some(v) = &param.validator.0 {
                    if !v(value) {
                        return Err(Error::Syntax(config.get_line(), "problem whith check parametr"));
                    }
                }
            } else if param.required {
                return Err(Error::Syntax(config.get_line(), "missing required config parametr"));
            }
        }
        for nested_config in config.iter() {
            if let Some(nested_schema) = self.nested.get(nested_config.get_name()) {
                nested_schema.check(nested_config)?;
            }
        }
        Ok(())
    }
    
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
        for param in self.params.iter() {
            result.push_str(&format!("{} - {}\n", &param.name, &param.description));
        }
        for nested in self.nested.values() {
            nested.info_section(result, level + 1);
        }
    }
}
