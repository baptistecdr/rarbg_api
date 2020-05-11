extern crate reqwest;
extern crate serde_derive;

use std::collections::HashMap;
use std::time::{SystemTime};
use reqwest::blocking::{Client, Response};
use self::serde_derive::{Deserialize, Serialize};
use reqwest::Error;

use USER_AGENT;
use ENDPOINT;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Token {
    value: String,
    created_at: SystemTime,
}

impl Token {
    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn created_at(&self) -> &SystemTime {
        &self.created_at
    }

    pub fn new(app_id: &str) -> Self {
        let response = Token::get(app_id);
        let content = Token::parse(response);
        let value = content.get("token");
        match value {
            Some(token) => Token {
                value: token.clone(),
                created_at: SystemTime::now(),
            },
            None => panic!("Failed to retrieve a token from RARBG API.")
        }
    }

    fn get(app_id: &str) -> Response {
        let client: Client = Client::builder()
            .user_agent(USER_AGENT)
            .build().unwrap();
        let response: Result<Response, Error> = client.get(ENDPOINT)
            .query(&[("get_token", "get_token")])
            .query(&[("app_id", app_id)])
            .send();
        match response {
            Ok(response) => response,
            Err(reason) => panic!("{}", reason)
        }
    }

    fn parse(response: Response) -> HashMap<String, String> {
        match response.json() {
            Ok(json) => json,
            Err(reason) => panic!("{}", reason)
        }
    }

    pub fn is_valid(&self) -> bool {
        let sys_time = SystemTime::now();
        let difference = sys_time.duration_since(self.created_at);
        match difference {
            Ok(duration) => duration.as_secs() as f64 + f64::from(duration.subsec_nanos()) * 1e-9 < 600.0, // < 10 min
            Err(_) => false
        }
    }

    pub fn as_string(&self) -> &String {
        self.value()
    }
}
