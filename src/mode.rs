#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Mode {
    Search,
    List,
}

impl Mode {
    pub fn as_str(&self) -> &str {
        match self {
            Mode::Search => "search",
            Mode::List => "list",
        }
    }
}
