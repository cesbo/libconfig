extern crate ini;
use ini::*;

const T1: &str = r#"
[section-A]
key.1 = 123
key.2 = foo

[section-B]
key.3 = 456
key.4 = bar
"#;

#[test]
fn test_writer() {
    let buffer = Vec::<u8>::new();
    let mut writer = EventWriter::new(buffer);

    writer.write(IniEvent::StartSection("section-A")).unwrap();
    writer.write(IniEvent::Property("key.1", "123")).unwrap();
    writer.write(IniEvent::Property("key.2", "foo")).unwrap();
    writer.write(IniEvent::StartSection("section-B")).unwrap();
    writer.write(IniEvent::Property("key.3", "456")).unwrap();
    writer.write(IniEvent::Property("key.4", "bar")).unwrap();

    let s = String::from_utf8(writer.into_inner()).unwrap();
    println!("{}", s);
    assert_eq!(s, T1);
}
