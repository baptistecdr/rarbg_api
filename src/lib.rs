#![crate_name = "rarbg_api"]
extern crate reqwest;
extern crate serde_json;

use std::thread::sleep;
use std::time::Duration;
use serde_json::{Error as SerdeJsonError};
use reqwest::blocking::{Client, Response, RequestBuilder};
use reqwest::Error as ReqwestError;

pub mod category;
pub mod token;
pub mod torrent;
pub mod torrents;
pub mod format;
pub mod limit;
pub mod api_parameters;
pub mod mode;
pub mod sort_by;
pub mod error;
pub mod api_parameters_builder;

use api_parameters::ApiParameters;
use mode::Mode;
use token::Token;
use torrents::Torrents;
use error::Error;

/* The API has a 1req/2s limit. We take one extra second just to be sure. */
const REQUEST_TIME_LIMIT: u64 = 3;
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:73.0) Gecko/20100101 Firefox/73.0";
const ENDPOINT: &str = "https://torrentapi.org/pubapi_v2.php";

#[derive(Debug)]
pub struct RarBgApi {
    app_id: String,
    token: Token,
}

impl RarBgApi {
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn new(app_id: &str) -> Self {
        RarBgApi {
            token: Token::new(app_id),
            app_id: app_id.to_string(),
        }
    }

    pub fn list(&mut self, parameters: Option<&ApiParameters>) -> Result<Torrents, Error> {
        self.request(None, Mode::List, parameters)
    }

    fn request(&mut self, search_value: Option<&[(&str, &str)]>, mode: Mode, parameters: Option<&ApiParameters>) -> Result<Torrents, Error> {
        if !self.token.is_valid() {
            self.token = Token::new(self.app_id());
        }
        sleep(Duration::new(REQUEST_TIME_LIMIT, 0));

        let client: Client = Client::builder()
            .user_agent(USER_AGENT)
            .build().unwrap();

        let mut request: RequestBuilder = client.get(ENDPOINT)
            .query(&[("mode", mode.as_str())])
            .query(&[("token", self.token().as_str())])
            .query(&[("app_id", self.app_id())]);

        if search_value.is_some() {
            request = request.query(search_value.unwrap());
        }

        if parameters.is_some() {
            let pm = parameters.unwrap();
            request = request.query(&[("ranked", pm.ranked().clone() as isize)])
                .query(&[("sort", &pm.sort_by().as_str())])
                .query(&[("limit", pm.limit().as_str())])
                .query(&[("format", pm.format().as_str())]);

            if pm.minimum_seeders().is_some() {
                request = request.query(&[("min_seeders", pm.minimum_seeders().unwrap())]);
            }

            if pm.minimum_leechers().is_some() {
                request = request.query(&[("min_leechers", pm.minimum_leechers().unwrap())]);
            }

            if pm.categories().is_some() {
                let categories = pm.categories().as_ref().unwrap();
                let stringified_categories: Vec<&str> = categories.iter().map(|c| c.as_str()).collect();
                let joined_categories: String = stringified_categories.join(";");
                request = request.query(&[("category", joined_categories)]);
            }
        }
        let response: Result<Response, ReqwestError> = request.send();

        let content = match response {
            Ok(res) => res.text(),
            Err(reason) => panic!("{}", reason)
        };

        let text = match content {
            Ok(text) => text,
            Err(reason) => panic!("{}", reason)
        };

        let torrents: Result<Torrents, SerdeJsonError> = serde_json::from_str(text.clone().as_str());
        match torrents {
            Ok(torrents) => Ok(torrents),
            Err(reason1) => {
                let api_error: Result<Error, SerdeJsonError> = serde_json::from_str(text.clone().as_str());
                match api_error {
                    Ok(api_error) => Err(api_error),
                    Err(reason2) => panic!("First reason: {}. Second reason: {}", reason1, reason2)
                }
            }
        }
    }

    pub fn search(&mut self, value: &str, parameters: Option<&ApiParameters>) -> Result<Torrents, Error> {
        self.request(Some(&[("search_string", value)]), Mode::Search, parameters)
    }

    pub fn search_by_imdb(&mut self, value: &str, parameters: Option<&ApiParameters>) -> Result<Torrents, Error> {
        self.request(Some(&[("search_imdb", value)]), Mode::Search, parameters)
    }

    pub fn search_by_tvdb(&mut self, value: &str, parameters: Option<&ApiParameters>) -> Result<Torrents, Error> {
        self.request(Some(&[("search_tvdb", value)]), Mode::Search, parameters)
    }

    pub fn search_by_tmdb(&mut self, value: &str, parameters: Option<&ApiParameters>) -> Result<Torrents, Error> {
        self.request(Some(&[("search_tmdb", value)]), Mode::Search, parameters)
    }
}
