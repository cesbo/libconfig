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
    let config = Section::parse(T1.as_bytes()).unwrap();
    let mut s = Vec::<u8>::new();
    config.dump(&mut s).unwrap();
    assert_eq!(s.as_slice(), T1.as_bytes());
}
