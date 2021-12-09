use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Error {
    error: String,
    error_code: u8,
}

impl Error {
    /// Return the description of the error given by the API.
    pub fn error(&self) -> &str {
        self.error.as_str()
    }

    /// Return the error code given by the API.
    pub fn error_code(&self) -> &u8 {
        &self.error_code
    }
}
