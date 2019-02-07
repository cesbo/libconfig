extern crate ini;
use ini::*;

const T1: &str = r#"xmltv = /projects/opt/discovery.xml
output = udp://127.0.0.1:10000
u16 = 1234
bool = true

[multiplex]
tsid = 1

[.service]
name = 🐽
pnr = 1
xmltv-id = discovery-channel

[..test]
value = test for discovery-channel

[.service]
pnr = 1185
xmltv-id = yamal-region
"#;

#[test]
fn test_writer() {
    let mut config = Section::new("");
    config.push(Property::new("xmltv", "/projects/opt/discovery.xml"));
    config.push(Property::new("output", "udp://127.0.0.1:10000"));
    config.push(Property::new("u16", "1234"));
    config.push(Property::new("bool", "true"));

    let mut m = Section::new("multiplex");
    m.push(Property::new("tsid", "1"));

    let mut t = Section::new("test");
    t.push(Property::new("value", "test for discovery-channel"));
    let mut s = Section::new("service");
    s.push(Property::new("name", "🐽"));
    s.push(Property::new("pnr", "1"));
    s.push(Property::new("xmltv-id", "discovery-channel"));
    s.push(t);
    m.push(s);

    let mut s = Section::new("service");
    s.push(Property::new("pnr", "1185"));
    s.push(Property::new("xmltv-id", "yamal-region"));
    m.push(s);

    config.push(m);

    let mut s = Vec::<u8>::new();
    config.dump(&mut s).unwrap();
    let s = unsafe { String::from_utf8_unchecked(s) };

    assert_eq!(s.as_str(), T1);
}
