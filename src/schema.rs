pub struct Schema {
    vereficate: bool,
}

impl Schema {
    #[inline]
    //pub fn new<P: AsRef<Path>>(schema: P) -> Self
    pub fn new() -> Self
    {
        Schema {
            vereficate: false,
        }
    }
    pub fn info(self)-> String {
        "test ok".to_string()
    }
}
