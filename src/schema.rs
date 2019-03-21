use std::collections::HashMap;
use std::boxed::Box;

use super::config::Config;


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
    pub fn check(&mut self, config: &Config) -> String {
        let mut result = String::new();
        for param in self.params.iter() {
            if config.get_str(&param.name) != None {
                let check_func = (&param.validators)(config.get_str(&param.name).unwrap());
                if check_func{
                    self.check_list.insert(param.name.to_string(), true);
                }
                else {
                    return format!("Format Error at line :{}",  &param.name)
                }
            }
            else{
                self.check_list.insert(param.name.to_string(), false);
                if param.required {
                    return format!("Syntax Error at line :{}", &param.name);
                }
            }
        }
        if result == "" {
            result = "Ok".to_string();
        }
        result
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
