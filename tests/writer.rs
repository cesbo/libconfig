use std::fs;

extern crate ini;
use ini::*;

#[test]
fn test_writer() {
    let mut config = Section::new("");
    config.push(Property::new("xmltv", "/projects/opt/discovery.xml"));
    config.push(Property::new("output", "udp://127.0.0.1:10000"));
    config.push(Property::new("u16", 1234));
    config.push(Property::new("bool", true));

    let mut m = Section::new("multiplex");
    m.push(Property::new("tsid", 1));

    let mut s = Section::new("service");
    s.push(Property::new("name", "🐽"));
    s.push(Property::new("pnr", 1));
    s.push(Property::new("xmltv-id", "discovery-channel"));
    m.push(s);

    let mut s = Section::new("service");
    s.push(Property::new("xmltv", "/projects/opt/yamal.xml"));
    s.push(Property::new("pnr", 1185));
    s.push(Property::new("xmltv-id", "yamal-region"));
    m.push(s);

    config.push(m);

    let mut s = Vec::<u8>::new();
    config.dump(&mut s).unwrap();
    let s = unsafe { String::from_utf8_unchecked(s) };

    let t1 = fs::read_to_string("tests/data/t1.ini").unwrap();
    assert_eq!(s.as_str(), t1.as_str());
}
