use std::collections::HashMap;
use std::boxed::Box;

use super::config::Config;
pub use crate::error::{Error, Result};


struct Param {
    name: String,
    description: String,
    required: bool,
    validator: Box<Fn(&str) -> bool>,
}


pub struct Schema {
    params: Vec<Param>,
    check_list: HashMap<String, bool>
}


impl Schema {
    #[inline]
    pub fn new() -> Self {
        Schema {
            params: Vec::new(),
            check_list: HashMap::new(),
        }
    }
        
    #[inline]
    pub fn set<S,B: 'static>(&mut self, name: S, description: S, required: bool, validator: B)
    where
        S: Into<String>,
        B: Fn(&str) -> bool, 
    {
        let param = Param {
            name: name.into(),
            description: description.into(),
            required: required,
            validator: Box::new(validator),
        };
        self.params.push(param);
    }
    
    pub fn check(&mut self, config: &Config) ->  Result<()> {
        for param in self.params.iter() {
            if let Some(value) = config.get_str(&param.name) {
                if (&param.validator)(value){
                    self.check_list.insert(param.name.to_string(), true);
                } else {
                    return Err(Error::Syntax(config.get_line(), "problem whith check parametr"));
                }
            } else {
                self.check_list.insert(param.name.to_string(), false);
                if param.required {                
                    return Err(Error::Syntax(config.get_line(), "required config parametr missing"));
                }
            }
        }
        Ok(())
    }
    
    pub fn info(&mut self) -> String {
        let mut result = String::new();
        for param in self.params.iter() {
            result.push_str(&param.name);
            result.push_str(" - ");
            result.push_str(&param.description);
            result.push_str("\n");
        }
        result
    }
}
