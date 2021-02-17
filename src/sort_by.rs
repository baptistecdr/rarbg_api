use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum SortBy {
    Seeders,
    Leechers,
    Last,
}

impl Default for SortBy {
    fn default() -> Self {
        SortBy::Last
    }
}

impl SortBy {
    pub fn as_str(&self) -> &str {
        match self {
            SortBy::Seeders => "seeders",
            SortBy::Leechers => "leechers",
            SortBy::Last => "last",
        }
    }
}
