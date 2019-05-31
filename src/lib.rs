#[macro_use]
extern crate error_rules;

mod config;
pub use crate::config::{
    Config,
    ConfigError,
};

mod schema;
pub use crate::schema::Schema;
