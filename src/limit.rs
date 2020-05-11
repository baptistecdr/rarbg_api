extern crate serde_derive;

use self::serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Limit {
    TwentyFive = 25,
    Fifty = 50,
    OneHundred = 100,
}

impl Default for Limit {
    fn default() -> Self { Limit::TwentyFive }
}