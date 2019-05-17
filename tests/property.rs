use config::Config;


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


const WF: &str = r#"
ok = true
wrong-format
"#;


const WL: &str = r#"
ok = true
# ok-level
ok = true
### wrong-level
"#;


#[test]
fn test_property() {
    let config = Config::parse(T1.as_bytes()).unwrap();

    match config.get::<bool>("true", false) {
        Ok(v) => assert_eq!(v, true),
        Err(_) => unreachable!(),
    };

    match config.get::<bool>("false", true) {
        Ok(v) => assert_eq!(v, false),
        Err(_) => unreachable!(),
    };

    match config.get::<bool>("bool", false) {
        Ok(_) => unreachable!(),
        Err(e) => e.iter_chain().for_each(|e| println!("> {}", e)),
    };

    match config.get::<u8>("u8", 0) {
        Ok(v) => assert_eq!(v, 188),
        Err(_) => unreachable!(),
    };

    match config.get::<u8>("u8-max", 0) {
        Ok(_) => unreachable!(),
        Err(e) => e.iter_chain().for_each(|e| println!("> {}", e)),
    };

    match config.get::<i32>("i32-min", 0) {
        Ok(_) => unreachable!(),
        Err(e) => e.iter_chain().for_each(|e| println!("> {}", e)),
    };

    match config.get::<usize>("usize", 0) {
        Ok(_) => unreachable!(),
        Err(e) => e.iter_chain().for_each(|e| println!("> {}", e)),
    };

    match config.get::<u16>("u16", 0) {
        Ok(v) => assert_eq!(v, 0x1234),
        Err(_) => unreachable!(),
    };
}


#[test]
fn test_wrong_format() {
    match Config::parse(WF.as_bytes()) {
        Ok(_) => unreachable!(),
        Err(e) => e.iter_chain().for_each(|e| println!("> {}", e)),
    }
}


#[test]
fn test_wrong_level() {
    match Config::parse(WL.as_bytes()) {
        Ok(_) => unreachable!(),
        Err(e) => e.iter_chain().for_each(|e| println!("> {}", e)),
    }
}
