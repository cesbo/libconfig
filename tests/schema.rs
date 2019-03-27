extern crate config;
use config::config::Config;
use config::schema;

fn test_true() -> impl Fn(&str) -> bool {
    move |_v: &str| -> bool { 
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
    let mut schema = schema::Schema::new("","");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("output", "Output streem", true, test_true());
    match schema.check(&config) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e.to_string()),
    }
    println!("\n Info: {}", schema.info());
    println!("===================================");
    println!("test Schema whith missing parametr:");
    let mut schema = schema::Schema::new("","");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", true, range(0 .. 2110));
    schema.set("output", "Output streem", true, test_true());
    schema.set("test_key", "This is testparam", true, test_true());
    schema.set("test_req", "Test not Required param", false, test_true());
    match schema.check(&config) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e.to_string()),
    }
    println!("\n Info: \n{}", schema.info());
    println!("===================================");
    println!("test Schema whith trouble range:");
    let mut schema = schema::Schema::new("","");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", true, range(0 .. 3));
    match schema.check(&config) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e.to_string()),
    }
    println!("\n Info: \n{}", schema.info());
    println!("===================================");
    println!("test Schema whith normal range:");
    let mut schema = schema::Schema::new("","");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", true, range(0 .. 65000));
    match schema.check(&config) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e.to_string()),
    }
    println!("\n Info: \n{}", schema.info());
    println!("===================================");
    println!("test Schema whithout validator:");
    let mut schema = schema::Schema::new("","");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", true, None);
    match schema.check(&config) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e.to_string()),
    }
    println!("\n Info: \n{}", schema.info());
    println!("===================================");
    println!("test RECURSIVE Schema whith normal range:");
    let mut schema = schema::Schema::new("","");
    let mut multiplex = schema::Schema::new("multiplex","simple dvb multiplex");
    let mut service = schema::Schema::new("service","");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", true, range(0 .. 65000));
    multiplex.set("tsid", "Number of transport", true, range(0 .. 65000));
    multiplex.set("test_name", "Not required test parametr", false, None);
    service.set("pnr", "Program name", true, range(0 .. 65000));
    service.set("xmltv-id", "text name of streem", true, None);
    multiplex.push(service);
    schema.push(multiplex);    
    match schema.check(&config) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e.to_string()),
    }
    println!("\n Info: \n{}", schema.info());
    println!("===================================");
}
