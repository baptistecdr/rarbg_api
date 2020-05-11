#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Search,
    List,
}

impl Mode {
    pub fn as_string(&self) -> &str {
        match self {
            Mode::Search => "search",
            Mode::List => "list"
        }
    }
}