use std::fmt;

pub enum IniEvent<'a> {
    /// Beginning of the INI section. Contain unescaped section name
    StartSection(&'a str),
    /// End of the INI section
    EndSection,
    /// Key-Value pair
    Property(&'a str, &'a str),
    /// Empty line or comment
    Skip,
}

impl<'a> fmt::Debug for IniEvent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IniEvent::Property(key, value) => write!(f, "Property({}, {})", key, value),
            IniEvent::StartSection(name) => write!(f, "StartSection({})", name),
            IniEvent::EndSection => write!(f, "EndSection"),
            IniEvent::Skip => write!(f, "Skip"),
        }
    }
}
