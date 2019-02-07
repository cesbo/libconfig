extern crate ini;
use ini::Section;

const T1: &str = r#"
true = true
false = false
bool = test
u8 = 188
u8-max = 1000
i32-min = -2200000000
usize = 1q
u16 = 0x1234
"#;

#[test]
fn test_property() {
    let config = Section::parse(T1.as_bytes()).unwrap();

    match config.get_bool("true", false) {
        Ok(v) => assert_eq!(v, true),
        Err(_) => unreachable!(),
    };

    match config.get_bool("false", true) {
        Ok(v) => assert_eq!(v, false),
        Err(_) => unreachable!(),
    };

    match config.get_bool("bool", false) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    };

    match config.get_number::<u8>("u8", 0) {
        Ok(v) => assert_eq!(v, 188),
        Err(_) => unreachable!(),
    };

    match config.get_number::<u8>("u8-max", 0) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    };

    match config.get_number::<i32>("i32-min", 0) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    };

    match config.get_number::<usize>("usize", 0) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    };

    match config.get_number::<u16>("u16", 0) {
        Ok(v) => assert_eq!(v, 0x1234),
        Err(_) => unreachable!(),
    };
}
