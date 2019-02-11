extern crate config;
use config::Config;

#[test]
fn test_reader() {
    let config = Config::open("tests/data/t1.conf").unwrap();

    assert_eq!(config.get_str("xmltv").unwrap(), "/projects/opt/discovery.xml");
    assert_eq!(config.get_str("test").unwrap_or("opt"), "opt");
    assert_eq!(config.get::<bool>("bool", false).unwrap(), true);
    assert_eq!(config.get::<u16>("u16", 0).unwrap(), 1234u16);

    for multiplex in config.iter() {
        assert_eq!(multiplex.get_name(), "multiplex");
        assert_eq!(multiplex.get::<u16>("tsid", 0).unwrap(), 1);

        for service in multiplex.iter() {
            match service.get::<u16>("pnr", 0).unwrap() {
                1 => assert_eq!(service.get_str("xmltv-id").unwrap(), "discovery-channel"),
                1185 => assert_eq!(service.get_str("xmltv-id").unwrap(), "yamal-region"),
                _ => unreachable!(),
            }
        }
    }
}
