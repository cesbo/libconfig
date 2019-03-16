use std::collections::HashMap;

use super::config::Config;


#[derive(Debug, Default)]
struct Param {
    name: String,
    description: String,
    required: bool,
    validators: Vec<String>,
}


#[derive(Debug, Default)]
pub struct Schema {
    params: Vec<Param>,
    check_global: bool,
    check_list: HashMap<String, bool>
}


impl Schema {
    #[inline]
    pub fn new() -> Self
    {
        Schema {
            params: Vec::new(),
            check_global: true,
            check_list: HashMap::new(),
        }
    }
        
    #[inline]
    pub fn set<S>(&mut self, name: S, description: S, required: bool, validators: Vec<String>)
    where
        S: Into<String>,    
    {
        let param = Param {
            name: name.into(),
            description: description.into(),
            required: required,
            validators: validators,
        };
        self.params.push(param);
    }
    
    #[inline]
    pub fn check(&mut self, config: &Config)-> String {
        let mut result = String::new();
        for param in self.params.iter() {
            if config.get_str(&param.name) != None {
                self.check_list.insert(param.name.to_string(), true);
            }
            else{
                self.check_list.insert(param.name.to_string(), false);
                if param.required {
                    self.check_global = false;
                    result.push_str("Error: config whithout parametr: ");
                    result.push_str(&param.name);
                    result.push_str("\n");
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
