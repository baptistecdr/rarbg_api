use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum Format {
    Json,
    JsonExtended,
}

impl Default for Format {
    fn default() -> Self {
        Format::Json
    }
}

impl Format {
    pub fn as_str(&self) -> &str {
        match self {
            Format::Json => "json",
            Format::JsonExtended => "json_extended",
        }
    }
}
