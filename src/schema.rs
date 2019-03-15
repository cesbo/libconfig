use super::config::Config;
use std::collections::HashMap;
//use super::config::*;

struct Params {
    name: String,
    description: String,
    required: bool,
    validators: Vec<String>,
}

pub struct Schema {
    vereficate: bool,
    params: Vec<Params>,
    check_list: HashMap<String, bool>
}

impl Schema {
    #[inline]
    pub fn new() -> Self
    {
        Schema {
            vereficate: false,
            params: Vec::new(),
            check_list: HashMap::new(),
        }
    }
    
    pub fn info(self)-> String {
        "test ok".to_string()
    }
    
    pub fn set<S>(&mut self, name: S, description: S, required: bool, validators: Vec<String>)
    where
        S: Into<String>,    
    {
        let params = Params {
            name: name.into(),
            description: description.into(),
            required: required,
            validators: validators,
        };
        self.params.push(params);
    }
    
    pub fn check(&mut self, config: &Config)-> String {
        let mut is_set = false;
        for params in self.params.iter() {
            if config.get_str(&params.name) != None {
                println!("{} - est",params.name);
            }
            else{
                println!("{} - none",params.name);
            }
            /*for items in config.iter_items(){
                if params.name == items.name{
                   println!("est");
                }
                else{
                   println!("no");
                }
                //дописать
            }*/
        }
        /*
        println!("Str  is {}",config.get_str("xmltv").unwrap());
        for multiplex in config.iter() {
            println!("Multiplex");
            self.check_items(multiplex);
            for service in multiplex.iter() {
                println!("Service");
                self.check_items(service);
            }
        }*/
        "ok".to_string()
    }
    
    fn check_items(&self, config: &Config) {
        for items in config.iter_items(){
            println!("Iter Items");
        }        
    }
}
