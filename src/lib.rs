extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::thread::sleep;
use std::time::Duration;
use serde_json::{Error as SerdeJsonError};
use reqwest::blocking::{Client, Response, RequestBuilder};
use reqwest::Error as ReqwestError;

pub mod category;
pub mod token;
pub mod torrent_result;
pub mod torrent_results;
pub mod format;
pub mod limit;
pub mod api_parameters;
pub mod mode;
pub mod sort_by;
pub mod error;

use api_parameters::ApiParameters;
use mode::Mode;
use token::Token;
use torrent_results::TorrentResults;
use error::Error;

/* The API has a 1req/2s limit. We take one extra second just to be sure. */
const REQUEST_TIME_LIMIT: u64 = 3;
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:73.0) Gecko/20100101 Firefox/73.0";

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

    pub fn list(&mut self, parameters: Option<&ApiParameters>) -> Result<TorrentResults, Error> {
        self.request(None, Mode::List, parameters)
    }

    fn request(&mut self, search_value: Option<&[(&str, &str)]>, mode: Mode, parameters: Option<&ApiParameters>) -> Result<TorrentResults, Error> {
        if !self.token.is_valid() {
            self.token = Token::new(self.app_id());
        }
        sleep(Duration::new(REQUEST_TIME_LIMIT, 0));

        let client: Client = Client::builder()
            .user_agent(USER_AGENT)
            .build().unwrap();

        let mut request: RequestBuilder = client.get("https://torrentapi.org/pubapi_v2.php")
            .query(&[("mode", mode.as_string())])
            .query(&[("token", self.token().as_string())])
            .query(&[("app_id", self.app_id())]);

        if search_value.is_some() {
            request = request.query(search_value.unwrap());
        }

        if parameters.is_some() {
            let pm = parameters.unwrap();
            request = request.query(&[("ranked", pm.ranked())]);
            request = request.query(&[("sort", pm.sort_by().as_string())]);
            request = request.query(&[("limit", pm.limit())]);
            request = request.query(&[("format", pm.format().as_string())]);

            if pm.minimum_seeders().is_some() {
                request = request.query(&[("minimum_seeders", pm.minimum_seeders().unwrap())]);
            }

            if pm.minimum_leechers().is_some() {
                request = request.query(&[("minimum_leechers", pm.minimum_leechers().unwrap())]);
            }

            if pm.categories().is_some() {
                let categories = pm.categories().as_ref().unwrap();
                let mut cat_to_str = String::new();
                for i in 0..categories.len() {
                    let id = categories[i] as usize;
                    cat_to_str.push_str(&format!("{},", id));
                }
                cat_to_str.pop(); // Remove last comma
                request = request.query(&[("category", cat_to_str)]);
            }
        }

        let response: Result<Response, ReqwestError> = request.send();
        sleep(Duration::new(REQUEST_TIME_LIMIT, 0));

        let content = match response {
            Ok(res) => res.text(),
            Err(reason) => panic!("{}", reason)
        };

        let text = match content {
            Ok(text) => text,
            Err(reason) => panic!("{}", reason)
        };

        let torrents: Result<TorrentResults, SerdeJsonError> = serde_json::from_str(text.clone().as_str());
        match torrents {
            Ok(torrents) => Ok(torrents),
            Err(_) => {
                let api_error: Result<Error, SerdeJsonError> = serde_json::from_str(text.clone().as_str());
                match api_error {
                    Ok(api_error) => Err(api_error),
                    Err(reason) => panic!("{}", reason)
                }
            }
        }
    }

    pub fn search(&mut self, value: &str, parameters: Option<&ApiParameters>) -> Result<TorrentResults, Error> {
        self.request(Some(&[("search_string", value)]), Mode::Search, parameters)
    }

    pub fn search_by_imdb(&mut self, value: &str, parameters: Option<&ApiParameters>) -> Result<TorrentResults, Error> {
        self.request(Some(&[("search_imdb", value)]), Mode::Search, parameters)
    }

    pub fn search_by_tvdb(&mut self, value: &str, parameters: Option<&ApiParameters>) -> Result<TorrentResults, Error> {
        self.request(Some(&[("search_tvdb", value)]), Mode::Search, parameters)
    }

    pub fn search_by_tmdb(&mut self, value: &str, parameters: Option<&ApiParameters>) -> Result<TorrentResults, Error> {
        self.request(Some(&[("search_tmdb", value)]), Mode::Search, parameters)
    }
}
