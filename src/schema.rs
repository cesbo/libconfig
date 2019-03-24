use std::boxed::Box;

use super::config::Config;
pub use crate::error::{Error, Result};


type OptionBox<T> = Option<Box<T>>;

struct Param {
    name: String,
    description: String,
    required: bool,
    //validator: OptionBox,
    validator: OptionBox<Fn(&str) -> bool>,
}


pub struct Schema {
    params: Vec<Param>
}


impl Schema {
    #[inline]
    pub fn new() -> Self {
        Schema {
            params: Vec::new()
        }
    }
        
    #[inline]
    pub fn set<S,B: 'static>(&mut self, name: S, description: S, required: bool, validator: B)
    where
        S: Into<String>,
        B: Fn(&str) -> bool,
        //B: Into<OptionBox>,  
    {
        let param = Param {
            name: name.into(),
            description: description.into(),
            required: required,
            //validator: validator,
            validator: Some(Box::new(validator)),
        };
        self.params.push(param);
    }
    
    pub fn check(&mut self, config: &Config) ->  Result<()> {
        for param in self.params.iter() {
            if param.required {
                let name = config.get_str(&param.name);
                if name == None {
                    return Err(Error::Syntax(config.get_line(), "missing required config parametr"));
                }
                if let Some(v) = &param.validator {
                    if v(name.unwrap()) == false {
                        return Err(Error::Syntax(config.get_line(), "problem whith check parametr"));
                    }           
                }
                /*if (&param.validator)(config.get_str(&param.name).unwrap()) = false {
                    return Err(Error::Syntax(config.get_line(), "problem whith check parametr"));
                }*/
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
