extern crate serde_derive;

use self::serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Format {
    Json,
    JsonExtended,
}

impl Format {
    pub fn as_string(&self) -> &str {
        match self {
            Format::Json => "json",
            Format::JsonExtended => "json_extended"
        }
    }
}

impl Default for Format {
    fn default() -> Self { Format::Json }
}
