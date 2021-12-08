extern crate reqwest;

use std::collections::HashMap;
use std::time::SystemTime;

use crate::ENDPOINT;
use crate::USER_AGENT;

use serde::{Deserialize, Serialize};
use reqwest::{Client, Response};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Token {
    value: String,
    created_at: SystemTime,
}

impl Token {
    /// Return the value of the token obtained from the API.
    ///
    /// # Example
    ///
    /// ```
    /// use rarbg_api::token::Token;
    /// #[tokio::main]
    /// async fn main() {
    ///     let token = Token::new("RustExample").await;
    ///     let value = token.value();
    ///     assert_ne!(value, "")
    /// }
    /// ```
    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    /// Returns the time when the token was created.
    ///
    /// # Example
    ///
    /// ```
    /// use rarbg_api::token::Token;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let token = Token::new("RustExample").await;
    ///     let time_of_creation = token.created_at();
    /// }
    /// ```
    pub fn created_at(&self) -> &SystemTime {
        &self.created_at
    }

    /// Create a Token with the value obtained from the API.
    /// This token can be use to make requests to the API.
    ///
    /// # Panics
    ///
    /// Panics if a token cannot be retrieve from the API.
    ///
    /// # Example
    ///
    /// ```
    /// use rarbg_api::token::Token;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let token = Token::new("RustExample").await;
    /// }
    /// ```
    pub async fn new(app_id: &str) -> Self {
        let response = Token::get(app_id).await;
        let content = Token::parse(response).await;
        let value = content.get("token");
        match value {
            Some(token) => Token {
                value: token.clone(),
                created_at: SystemTime::now(),
            },
            None => panic!("Failed to retrieve a token from RARBG API."),
        }
    }

    async fn get(app_id: &str) -> Response {
        let client: Client = Client::builder().user_agent(USER_AGENT).build().unwrap();
        let response = client
            .get(ENDPOINT)
            .query(&[("get_token", "get_token")])
            .query(&[("app_id", app_id)])
            .send().await;
        match response {
            Ok(response) => response,
            Err(reason) => panic!("{}", reason),
        }
    }

    async fn parse(response: Response) -> HashMap<String, String> {
        match response.json().await {
            Ok(json) => json,
            Err(reason) => panic!("{}", reason),
        }
    }

    /// Verifies that the token is still valid to use it with the API.
    /// Officially, a token is valid for 15 minutes but we keep this token valid for 10 minutes.
    ///
    /// # Example
    ///
    /// ```
    /// use rarbg_api::token::Token;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let token = Token::new("RustExample").await;
    ///     assert!(token.is_valid(), "Token should be valid !");
    /// }
    /// ```
    pub fn is_valid(&self) -> bool {
        let sys_time = SystemTime::now();
        let difference = sys_time.duration_since(self.created_at);
        match difference {
            Ok(duration) => {
                duration.as_secs() as f64 + f64::from(duration.subsec_nanos()) * 1e-9 < 600.0
            } // < 10 min
            Err(_) => false,
        }
    }
}
