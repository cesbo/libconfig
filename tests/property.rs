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
"#;


const WF: &str = r#"
ok = true
wrong-format
"#;


const WL: &str = r#"
ok = true
[ok]
ok = true
[ok/level]
ok = true
[wrong/level]
"#;


#[test]
fn test_property_bool_true() {
    let config = Config::parse(T1.as_bytes()).unwrap();

    match config.get::<bool>("true", false) {
        Ok(v) => assert_eq!(v, true),
        Err(_) => unreachable!(),
    };
}


#[test]
fn test_property_bool_false() {
    let config = Config::parse(T1.as_bytes()).unwrap();

    match config.get::<bool>("false", true) {
        Ok(v) => assert_eq!(v, false),
        Err(_) => unreachable!(),
    };
}


#[test]
fn test_property_bool_err() {
    let config = Config::parse(T1.as_bytes()).unwrap();

    match config.get::<bool>("bool", false) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    };
}


#[test]
fn test_property_u8() {
    let config = Config::parse(T1.as_bytes()).unwrap();

    match config.get::<u8>("u8", 0) {
        Ok(v) => assert_eq!(v, 188),
        Err(_) => unreachable!(),
    };
}


#[test]
fn test_property_u8_err() {
    let config = Config::parse(T1.as_bytes()).unwrap();

    match config.get::<u8>("u8-max", 0) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    };
}


#[test]
fn test_property_i32_err() {
    let config = Config::parse(T1.as_bytes()).unwrap();

    match config.get::<i32>("i32-min", 0) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    };
}


#[test]
fn test_property_usize_err() {
    let config = Config::parse(T1.as_bytes()).unwrap();

    match config.get::<usize>("usize", 0) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    };
}



#[test]
fn test_property_u16_hex() {
    let config = Config::parse(T1.as_bytes()).unwrap();

    match config.get::<u16>("u16-hex", 0) {
        Ok(v) => assert_eq!(v, 0x1234),
        Err(_) => unreachable!(),
    };
}


#[test]
fn test_wrong_format() {
    match Config::parse(WF.as_bytes()) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    }
}


#[test]
fn test_wrong_level() {
    match Config::parse(WL.as_bytes()) {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    }
}
