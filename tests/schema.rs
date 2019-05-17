use config::Config;
use config::Schema;

#[test]
fn test_schema_range_validator() {
    let f = Schema::range(100 .. 200);
    assert!(f("150"));
    assert!(f("100"));
    assert!(f("200"));
    assert!(f("0x80"));
    assert!(! f("50"));
    assert!(! f("250"));
    assert!(! f("test"));
}

#[test]
fn test_schema_ok() {
    let mut schema = Schema::new("", "");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("output", "Output streem", true, None);
    schema.check(&config).unwrap();
}

#[test]
fn test_schema_whithout_parametr() {
    let mut schema = Schema::new("", "");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("put", "Bad parametr", true, None);
    assert!(schema.check(&config).is_err());
}

#[test]
fn test_schema_unrequred() {
    let mut schema = Schema::new("", "");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("output", "Output streem", false, None);
    schema.check(&config).unwrap();
}

#[test]
fn test_schema_unrequred_whithout_parametr() {
    let mut schema = Schema::new("", "");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("put", "Bad parametr", false, None);
    schema.check(&config).unwrap();
}

#[test]
fn test_schema_range() {
    let mut schema = Schema::new("", "");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", true, Schema::range(0 .. 65000));
    schema.check(&config).unwrap();
}

#[test]
fn test_schema_out_range() {
    let mut schema = Schema::new("", "");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", true, Schema::range(0 .. 1));
    match schema.check(&config) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    }
}

#[test]
fn test_schema_out_range_unrequred() {
    let mut schema = Schema::new("", "");
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", false, Schema::range(0 .. 1));
    match schema.check(&config) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    }
}

#[test]
fn test_schema_nested_ok() {
    let mut schema = Schema::new("","");
    let mut multiplex = Schema::new("multiplex","simple dvb multiplex");
    let mut service = Schema::new("service","");
    let config = Config::open("tests/data/t1.conf").unwrap();
    service.set("pnr", "Program name", true, None);
    multiplex.push(service);
    schema.push(multiplex);
    schema.check(&config).unwrap();
}

#[test]
fn test_schema_nested_whithout_parametr() {
    let mut schema = Schema::new("", "");
    let mut multiplex = Schema::new("multiplex", "simple dvb multiplex");
    let mut service = Schema::new("service", "");
    service.set("pttr", "Unreal", true, None);
    multiplex.push(service);
    schema.push(multiplex);

    let config = Config::open("tests/data/t1.conf").unwrap();
    match schema.check(&config) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    }
}

#[test]
fn test_schema_nested_range() {
    let mut schema = Schema::new("","");
    let mut multiplex = Schema::new("multiplex","simple dvb multiplex");
    let mut service = Schema::new("service","");
    let config = Config::open("tests/data/t1.conf").unwrap();
    service.set("pnr", "Program name", true, Schema::range(0 .. 65000));
    multiplex.push(service);
    schema.push(multiplex);
    schema.check(&config).unwrap();
}

#[test]
fn test_schema_nested_out_range() {
    let mut schema = Schema::new("","");
    let mut multiplex = Schema::new("multiplex","simple dvb multiplex");
    let mut service = Schema::new("service","");
    let config = Config::open("tests/data/t1.conf").unwrap();
    service.set("pnr", "Program name", true, Schema::range(0 .. 1));
    multiplex.push(service);
    schema.push(multiplex);
    match schema.check(&config) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    }
}

#[test]
fn test_schema_nested_out_range_unrequred() {
    let mut schema = Schema::new("","");
    let mut multiplex = Schema::new("multiplex","simple dvb multiplex");
    let mut service = Schema::new("service","");
    let config = Config::open("tests/data/t1.conf").unwrap();
    service.set("pnr", "Program name", false, Schema::range(0 .. 1));
    multiplex.push(service);
    schema.push(multiplex);
    match schema.check(&config) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    }
}
