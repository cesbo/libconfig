extern crate config;
use config::config::Config;
use config::schema;
use std::path::Path;

#[test]
fn test_schema() {
    let schema = schema::Schema::new();
    let config = Config::open("tests/data/t1.conf").unwrap();
    //schema.validate(&config)
}
