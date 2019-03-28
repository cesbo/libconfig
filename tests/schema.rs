extern crate config;
use config::config::Config;
use config::schema;

fn range(r: std::ops::Range<usize>) -> impl Fn(&str) -> bool {
    move |s: &str| -> bool { 
        let v = s.parse().unwrap();
        if r.start >= v || r.end <= v { return false }
        true
    }
}

#[test]
fn test_schema_ok() {
    let mut schema = schema::Schema::new("", "");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("output", "Output streem", true, None);
    schema.check(&config).unwrap();
}

#[test]
fn test_schema_whithout_parametr() {
    let mut schema = schema::Schema::new("", "");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("put", "Bad parametr", true, None);
    assert!(schema.check(&config).is_err());
}

#[test]
fn test_schema_unrequred() {
    let mut schema = schema::Schema::new("", "");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("output", "Output streem", false, None);
    schema.check(&config).unwrap();
}

#[test]
fn test_schema_unrequred_whithout_parametr() {
    let mut schema = schema::Schema::new("", "");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("put", "Bad parametr", false, None);
    schema.check(&config).unwrap();
}

#[test]
fn test_schema_range() {
    let mut schema = schema::Schema::new("", "");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", true, range(0 .. 65000));
    schema.check(&config).unwrap();
}

#[test]
fn test_schema_out_range() {
    let mut schema = schema::Schema::new("", "");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", true, range(0 .. 1));
    assert!(schema.check(&config).is_err());
}

#[test]
fn test_schema_out_range_unrequred() {
    let mut schema = schema::Schema::new("", "");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", false, range(0 .. 1));
    assert!(schema.check(&config).is_err());
}

#[test]
fn test_schema_nested_ok() {
    let mut schema = schema::Schema::new("","");
    let mut multiplex = schema::Schema::new("multiplex","simple dvb multiplex");
    let mut service = schema::Schema::new("service","");
    let config = Config::open("tests/data/t1.conf").unwrap();
    service.set("pnr", "Program name", true, None);
    multiplex.push(service);
    schema.push(multiplex);    
    schema.check(&config).unwrap();
}

#[test]
fn test_schema_nested_whithout_parametr() {
    let mut schema = schema::Schema::new("","");
    let mut multiplex = schema::Schema::new("multiplex","simple dvb multiplex");
    let mut service = schema::Schema::new("service","");
    let config = Config::open("tests/data/t1.conf").unwrap();
    service.set("pttr", "Unreal", true, None);
    multiplex.push(service);
    schema.push(multiplex);    
    assert!(schema.check(&config).is_err());
}

#[test]
fn test_schema_nested_range() {
    let mut schema = schema::Schema::new("","");
    let mut multiplex = schema::Schema::new("multiplex","simple dvb multiplex");
    let mut service = schema::Schema::new("service","");
    let config = Config::open("tests/data/t1.conf").unwrap();
    service.set("pnr", "Program name", true, range(0 .. 65000));
    multiplex.push(service);
    schema.push(multiplex);   
    schema.check(&config).unwrap();
}

#[test]
fn test_schema_nested_out_range() {
    let mut schema = schema::Schema::new("","");
    let mut multiplex = schema::Schema::new("multiplex","simple dvb multiplex");
    let mut service = schema::Schema::new("service","");
    let config = Config::open("tests/data/t1.conf").unwrap();
    service.set("pnr", "Program name", true, range(0 .. 1));
    multiplex.push(service);
    schema.push(multiplex);   
    assert!(schema.check(&config).is_err());
}

#[test]
fn test_schema_nested_out_range_unrequred() {
    let mut schema = schema::Schema::new("","");
    let mut multiplex = schema::Schema::new("multiplex","simple dvb multiplex");
    let mut service = schema::Schema::new("service","");
    let config = Config::open("tests/data/t1.conf").unwrap();
    service.set("pnr", "Program name", false, range(0 .. 1));
    multiplex.push(service);
    schema.push(multiplex);   
    assert!(schema.check(&config).is_err());
}
    