use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Limit {
    TwentyFive,
    Fifty,
    OneHundred,
}

impl Default for Limit {
    fn default() -> Self {
        Limit::TwentyFive
    }
}

impl Limit {
    pub fn as_str(&self) -> &str {
        match self {
            Limit::TwentyFive => "25",
            Limit::Fifty => "50",
            Limit::OneHundred => "100",
        }
    }
}
