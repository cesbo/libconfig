use std::fs;

use config::Config;

#[test]
fn test_writer() {
    let mut config = Config::new("");
    config.set("xmltv", "/projects/opt/discovery.xml");
    config.set("output", "udp://127.0.0.1:10000");
    config.set("u16", 1234);
    config.set("bool", true);

    let mut m = Config::new("multiplex");
    m.set("tsid", 1);

    let mut s = Config::new("service");
    s.set("name", "🐽");
    s.set("pnr", 1);
    s.set("xmltv-id", "discovery-channel");
    m.push(s);

    let mut s = Config::new("service");
    s.set("xmltv", "/projects/opt/yamal.xml");
    s.set("pnr", 1185);
    s.set("xmltv-id", "yamal-region");
    m.push(s);

    config.push(m);

    let mut s = Vec::<u8>::new();
    config.dump(&mut s).unwrap();
    let s = unsafe { String::from_utf8_unchecked(s) };

    let t1 = fs::read_to_string("tests/data/t1.conf").unwrap();
    assert_eq!(s.as_str(), t1.as_str());
}
