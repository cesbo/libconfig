use std::collections::HashMap;
use std::boxed::Box;

use super::config::Config;
pub use crate::error::{Error, Result};


//#[derive(Debug, Default)]
struct Param {
    name: String,
    description: String,
    required: bool,
    validators: Box<Fn(&str) -> bool>,
}


//#[derive(Debug, Default)]
pub struct Schema 
{
    params: Vec<Param>,
    check_list: HashMap<String, bool>
}


impl Schema {
    #[inline]
    pub fn new() -> Self
    {
        Schema {
            params: Vec::new(),
            check_list: HashMap::new(),
        }
    }
        
    #[inline]
    pub fn set<S,B: 'static>(&mut self, name: S, description: S, required: bool, validators: B)
    where
        S: Into<String>,
        B: Fn(&str) -> bool, 
    {
        let param = Param {
            name: name.into(),
            description: description.into(),
            required: required,
            validators: Box::new(validators),
        };
        self.params.push(param);
    }
    
    #[inline]
    pub fn check(&mut self, config: &Config) ->  Result<&str> {
        for param in self.params.iter() {
            if config.get_str(&param.name) != None {
                let check_func = (&param.validators)(config.get_str(&param.name).unwrap());
                if check_func{
                    self.check_list.insert(param.name.to_string(), true);
                }
                else {
                    let line = config.get_line();
                    return Err(Error::Syntax(line, "problem whith check parametr"));
                }
            }
            else{
                self.check_list.insert(param.name.to_string(), false);
                if param.required {                
                    let line = config.get_line();
                    return Err(Error::Syntax(line, "required config parametr missing"));
                }
            }
        }
        Ok("ok")
    }
    
    #[inline]
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
