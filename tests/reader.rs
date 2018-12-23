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
    let mut reader = EventReader::new(T1.as_bytes());
    let mut section_count = 0;
    let mut end_section_count = 0;
    let mut key_count = 0;

    while let Some(item) = reader.next() {
        match item.unwrap() {
            IniEvent::StartSection(name) => {
                section_count += 1;
                match section_count {
                    1 => assert_eq!(name, "section-A"),
                    2 => assert_eq!(name, "section-B"),
                    _ => unreachable!(),
                };
            },
            IniEvent::EndSection => {
                end_section_count += 1;
            },
            IniEvent::Property(key, value) => {
                key_count += 1;
                match key {
                    "key.1" => assert_eq!(value, "123"),
                    "key.2" => assert_eq!(value, "foo"),
                    "key.3" => assert_eq!(value, "456"),
                    "key.4" => assert_eq!(value, "bar"),
                    _ => unreachable!(),
                };
            },
            IniEvent::Skip => {},
        }
    }

    assert_eq!(key_count, 4);
    assert_eq!(section_count, 2);
    assert_eq!(end_section_count, 1);
}
