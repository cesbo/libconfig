use config::Config;

#[test]
fn test_reader() {
    let config = Config::open("tests/data/t1.conf").unwrap();

    assert_eq!(config.get("xmltv"), Some("/projects/opt/discovery.xml"));
    assert_eq!(config.get::<&str>("test"), None);
    assert_eq!(config.get("bool"), Some(true));
    assert_eq!(config.get("u16"), Some(1234u16));

    for multiplex in config.iter() {
        assert_eq!(multiplex.get_name(), "multiplex");
        assert_eq!(multiplex.get("tsid"), Some(1u16));

        for service in multiplex.iter() {
            match service.get("pnr").unwrap_or(0u16) {
                1 => assert_eq!(service.get("xmltv-id"), Some("discovery-channel")),
                1185 => assert_eq!(service.get("xmltv-id"), Some("yamal-region")),
                _ => unreachable!(),
            }
        }
    }
}

#[test]
fn test_not_found() {
    match Config::open("tests/data/not-found.conf") {
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e),
    }
}
