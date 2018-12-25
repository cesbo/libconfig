extern crate ini;
use ini::*;

const T1: &str = r#"
[section-A]
key.1 = 🐽
key.2 = foo

[section-B]
key.3 = 456
key.4 = bar
"#;

#[test]
fn test_writer() {
    let buffer = Vec::<u8>::new();
    let mut writer = IniWriter::new(buffer);

    writer.write_section("section-A").unwrap();
    writer.write_property("key.1", "🐽").unwrap();
    writer.write_property("key.2", "foo").unwrap();
    writer.write_section("section-B").unwrap();
    writer.write_property("key.3", "456").unwrap();
    writer.write_property("key.4", "bar").unwrap();

    let s = String::from_utf8(writer.into_inner()).unwrap();
    assert_eq!(s, T1);
}
