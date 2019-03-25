extern crate config;
use config::config::Config;
use config::schema;

fn test_true() -> impl Fn(&str) -> bool {
    move |v: &str| -> bool { 
        true
    }
}

fn range(r: std::ops::Range<usize>) -> impl Fn(&str) -> bool {
    move |s: &str| -> bool { 
        let v = s.parse().unwrap();
        if r.start >= v || r.end <= v { return false }
        true
    }
}

#[test]
fn test_schema() {
    println!("===================================");
    println!("test Schema whith ok parametr:");
    let mut schema = schema::Schema::new();
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("output", "Output streem", true, test_true());
    match schema.check(&config) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e.to_string()),
    }
    println!("\n Info: {}", schema.info());
    println!("===================================");
    println!("test Schema whith missing parametr:");
    let mut schema = schema::Schema::new();
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", true, range(0 .. 2110));
    schema.set("output", "Output streem", true, test_true());
    schema.set("test_key", "This is testparam", true, test_true());
    schema.set("test_req", "Test not Required param", false, test_true());
    match schema.check(&config) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e.to_string()),
    }
    println!("\n Info: {}", schema.info());
    println!("===================================");
    println!("test Schema whith trouble range:");
    let mut schema = schema::Schema::new();
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", true, range(0 .. 3));
    match schema.check(&config) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e.to_string()),
    }
    println!("\n Info: {}", schema.info());
    println!("===================================");
    println!("test Schema whith normal range:");
    let mut schema = schema::Schema::new();
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", true, range(0 .. 65000));
    match schema.check(&config) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e.to_string()),
    }
    println!("\n Info: {}", schema.info());
    println!("===================================");
    println!("test RECURSIVE Schema whith normal range:");
    let mut schema = schema::Schema::new();
    let mut multiplex = schema::Schema::new();
    let mut service = schema::Schema::new();
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", true, range(0 .. 65000));
    multiplex.set("tsid", "Number of transport", true, range(0 .. 65000));
    service.set("pnr", "Program name", true, range(0 .. 65000));
    multiplex.set_nested(service);
    schema.set_nested(multiplex);
    match schema.check(&config) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e.to_string()),
    }
    println!("\n Info: {}", schema.info());
    println!("===================================");
    /*println!("test Schema whithout validator:");
    let mut schema = schema::Schema::new();
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", true, test_true());
    match schema.check(&config) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e.to_string()),
    }
    println!("\n Info: {}", schema.info());
    println!("===================================");*/
}
