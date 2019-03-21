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
    let mut schema = schema::Schema::new();
    let config = Config::open("tests/data/t1.conf").unwrap();
    schema.set("u16", "Test u16", true, range(0 .. 2110));
    schema.set("output", "Output streem", true, test_true());
    schema.set("test_key", "This is testparam", true, test_true());
    schema.set("test_req", "Test not Required param", false, test_true());
    schema.check(&config);
    println!("\n Info: {}", schema.info());
    //println!("{}",range(v));
    //println!("{:#?}", config); 
    //println!("{:#?}", schema);
}
