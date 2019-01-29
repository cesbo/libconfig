extern crate ini;
use ini::*;

const T1: &str = r#"
[section-A]
key.1 = 123
key.2 = foo

; comment

[section-B]
key.3 = 456
key.4 = bar
"#;

#[test]
fn test_reader() {
    let config = IniReader::parse(T1.as_bytes()).unwrap();
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
