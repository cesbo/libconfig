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
    let mut ini = Ini::default();

    let mut sa = Section::default();
    sa.push("key.1", "🐽");
    sa.push("key.2", "foo");
    ini.push("section-A", sa);

    let mut sb = Section::default();
    sb.push("key.3", "456");
    sb.push("key.4", "bar");
    ini.push("section-B", sb);

    let mut s = Vec::<u8>::new();
    ini.dump(&mut s).unwrap();
    assert_eq!(s.as_slice(), T1.as_bytes());
}
