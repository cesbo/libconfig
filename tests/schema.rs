extern crate config;
use config::Schema;
use std::path::Path;

#[test]
fn test_schema() {
    //struct Schema_struct {
    //    file: Path,
    //}
    //let test_schema = Schema_struct {
    //    file: "tests/data/t1.conf",
    //}
    let mut schema = Schema::new("tests/data/t1.conf");
    schema.handbook(); 
}