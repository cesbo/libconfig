extern crate ini;
use ini::Section;

#[test]
fn test_reader() {
    let config = Section::open("tests/data/t1.ini").unwrap();

    assert_eq!(config.get_str("xmltv").unwrap(), "/projects/opt/discovery.xml");
    assert_eq!(config.get_str("test").unwrap_or("opt"), "opt");
    assert_eq!(config.get_bool("bool", false).unwrap(), true);
    assert_eq!(config.get_number::<u16>("u16", 0).unwrap(), 1234u16);

    for multiplex in config.sections() {
        assert_eq!(multiplex.get_name(), "multiplex");
        assert_eq!(multiplex.get_number::<u16>("tsid", 0).unwrap(), 1);

        for service in multiplex.sections() {
            match service.get_number::<u16>("pnr", 0).unwrap() {
                1 => assert_eq!(service.get_str("xmltv-id").unwrap(), "discovery-channel"),
                1185 => assert_eq!(service.get_str("xmltv-id").unwrap(), "yamal-region"),
                _ => unreachable!(),
            }
        }
    }
}
