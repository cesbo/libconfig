use config::Config;


const T1: &str = r#"
true = true
false = false
bool = test
u8 = 188
u8-max = 1000
i32-min = -2200000000
usize = 1q
u16-hex = 0x1234
str = Hello, world!
"#;


const WF: &str = r#"
ok = true
wrong-format
"#;


#[test]
fn test_property_bool_true() {
    let config = Config::parse(T1.as_bytes()).unwrap();
    assert_eq!(config.get("true"), Some(true));
}


#[test]
fn test_property_bool_false() {
    let config = Config::parse(T1.as_bytes()).unwrap();
    assert_eq!(config.get("false"), Some(false));
}


#[test]
fn test_property_bool_invalid() {
    let config = Config::parse(T1.as_bytes()).unwrap();
    assert_eq!(config.get::<bool>("bool"), None);
}


#[test]
fn test_property_bool_unavail() {
    let config = Config::parse(T1.as_bytes()).unwrap();
    assert_eq!(config.get::<bool>("unavail"), None);
}


#[test]
fn test_property_u8() {
    let config = Config::parse(T1.as_bytes()).unwrap();
    assert_eq!(config.get("u8"), Some(188u8));
}


#[test]
fn test_property_u8_err() {
    let config = Config::parse(T1.as_bytes()).unwrap();
    assert_eq!(config.get::<u8>("u8-max"), None);
}


#[test]
fn test_property_i32_err() {
    let config = Config::parse(T1.as_bytes()).unwrap();
    assert_eq!(config.get::<i32>("i32-min"), None);
}


#[test]
fn test_property_usize_err() {
    let config = Config::parse(T1.as_bytes()).unwrap();
    assert_eq!(config.get::<usize>("usize"), None);
}



#[test]
fn test_property_u16_hex() {
    let config = Config::parse(T1.as_bytes()).unwrap();
    assert_eq!(config.get("u16-hex"), Some(0x1234u16));
}


#[test]
fn test_property_str() {
    let config = Config::parse(T1.as_bytes()).unwrap();
    assert_eq!(config.get("str"), Some("Hello, world!"));
}


#[test]
fn test_wrong_format() {
    match Config::parse(WF.as_bytes()) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    }
}
