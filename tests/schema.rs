extern crate config;
use config::config::Config;
use config::schema;


#[test]
fn test_schema() {
    let mut schema = schema::Schema::new();
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("name", "Name of streem", true, vec![]);
    schema.set("output", "Output streem", true, vec![]);
    schema.set("test_key", "This is testparam", true, vec![]);
    schema.set("test_req", "Test not Required param", false, vec![]);
    println!("Result check() schema is \n{}", schema.check(&config));
    println!("Result into() schema is \n{}", schema.info());
    println!("{:#?}", config); 
    println!("{:#?}", schema);
}
