extern crate config;
use config::config::Config;
use config::schema;
use std::path::Path;

#[test]
fn test_schema() {
    let mut schema = schema::Schema::new();
    let mut config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("name", "Name of streem", true, vec![]);
    schema.set("output", "Output streem", true, vec![]);
    println!("{:#?}", config);
    println!("Result testing schema is {}", schema.check(&config));
    //println!("{}", schema.info());
}
