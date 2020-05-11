extern crate serde_derive;

use self::serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    error: String,
    error_code: u8,
}


impl Error {
    pub fn new(error: String, error_code: u8) -> Self {
        Error {
            error,
            error_code,
        }
    }

    pub fn error(&self) -> &String {
        return &self.error;
    }

    pub fn error_code(&self) -> &u8 {
        return &self.error_code;
    }
}
