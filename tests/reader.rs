extern crate ini;
use ini::Section;

const T1: &str = r#"
; comment

xmltv = /projects/opt/discovery.xml
output = udp://127.0.0.1:10000
u16 = 1234
bool = true

[multiplex]
tsid = 1

[.service]
pnr = 1
xmltv-id = discovery-channel

[..test]
value = test for discovery-channel

[.service]
xmltv = /projects/opt/yamal.xml
pnr = 1185
xmltv-id = yamal-region
"#;

#[test]
fn test_reader() {
    let config = Section::parse(T1.as_bytes()).unwrap();

    assert_eq!(config.get_str("xmltv", None).unwrap(), "/projects/opt/discovery.xml");
    assert_eq!(config.get_str("test", "opt").unwrap(), "opt");
    assert_eq!(config.get_bool("bool", false).unwrap(), true);
    assert_eq!(config.get_number::<u16>("u16", 0).unwrap(), 1234u16);
}

/*
#[test]
fn test_reader() {
    let config = Ini::parse(T1.as_bytes()).unwrap();
    assert_eq!(config.len(), 3);

    for (ref name, ref section) in config {
        match name.as_str() {
            "section-A" => {
                assert_eq!(section.len(), 2);
                for (ref key, ref value) in section {
                    match key.as_str() {
                        "key.1" => assert_eq!(value.as_str(), "123"),
                        "key.2" => assert_eq!(value.as_str(), "foo"),
                        _ => unreachable!(),
                    };
                }
            },
            "section-B" => {
                assert_eq!(section.len(), 2);
                for (ref key, ref value) in section {
                    match key.as_str() {
                        "key.3" => assert_eq!(value.as_str(), "456"),
                        "key.4" => assert_eq!(value.as_str(), "bar"),
                        _ => unreachable!(),
                    };
                }
            },
            "" => {
                assert_eq!(section.len(), 0);
            },
            _ => unreachable!(),
        }
    }
}
*/
