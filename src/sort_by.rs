extern crate serde_derive;

use self::serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum SortBy {
    Seeders,
    Leechers,
    Last,
}

impl Default for SortBy {
    fn default() -> Self { SortBy::Last }
}

impl SortBy {
    pub fn as_string(&self) -> &str {
        match self {
            SortBy::Seeders => "seeders",
            SortBy::Leechers => "leechers",
            SortBy::Last => "last"
        }
    }
}