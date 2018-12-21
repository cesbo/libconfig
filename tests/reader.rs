extern crate ini;
use ini::*;

const T1: &str = r#"
[section-A]
key.1 = 123
key.2 = foo

; comment

[section-B] ; comment
key.3 = 456
key.4 = bar
"#;

#[test]
fn test_reader() {
    let mut reader = EventReader::new(T1.as_bytes());
    let mut step = 0;
    let mut start_section_count = 0;
    let mut end_section_count = 0;
    let mut end_document_count = 0;
    let mut key_count = 0;

    loop {
        assert!(step <= 7);
        step += 1;

        match reader.next().unwrap() {
            IniEvent::StartSection(name) => {
                start_section_count += 1;
                match step {
                    1 => assert_eq!(name, "section-A"),
                    5 => assert_eq!(name, "section-B"),
                    _ => unreachable!(),
                };
            },
            IniEvent::EndSection => {
                end_section_count += 1;
                assert!(step == 4);
            },
            IniEvent::Property(key, value) => {
                key_count += 1;
                println!("Property({:?}, {:?})", key, value);
            },
            IniEvent::EndDocument => {
                end_document_count += 1;
                assert!(step == 8);
                break;
            }
        }
    }

    assert_eq!(key_count, 4);
    assert_eq!(start_section_count, 2);
    assert_eq!(end_section_count, 1);
    assert_eq!(end_document_count, 1);
}
