use std::boxed::Box;
use std::collections::HashMap;

use super::config::Config;
pub use crate::error::{Error, Result};


struct Param {
    name: String,
    description: String,
    required: bool,
    validator: Validator,
}


pub struct Schema {
    name: String,
    params: Vec<Param>,
    nested: HashMap<String, Schema>,
}


struct Validator(Option<Box<Fn(&str) -> bool>>);


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
    pub fn new<S>(name: S) -> Self 
    where
        S: Into<String>,
    {
        Schema {
            name: name.into(),
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
            required: required,
            validator: validator.into(),
        };
        self.params.push(param);
    }
    
    #[inline]
    pub fn push(&mut self, nested: Schema) {
        self.nested.insert(nested.name.clone(),nested);
    }
        
    pub fn check(&self, config: &Config) ->  Result<()> {
        for param in self.params.iter() {
            let name = config.get_str(&param.name);
            if param.required {
                if name == None {
                    return Err(Error::Syntax(config.get_line(), "missing required config parametr"));
                }
            }
            if let Some(v) = &param.validator.0 {
                if v(name.unwrap()) == false {
                    return Err(Error::Syntax(config.get_line(), "problem whith check parametr"));
                }
            }
        }
        for nested_config in config.iter() {
            if let Some(nested_schema) = self.nested.get(nested_config.get_name()){
                match nested_schema.check(nested_config) {
                    Ok(_) => {},
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(())
    }
    
    pub fn info(&mut self) -> String {
        let mut result = String::new();
        for param in self.params.iter() {
            result.push_str(&format!("- {} - {} \n", &param.name,&param.description));
        }
        result
    }
}
