#![crate_name = "rarbg_api"]
extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::thread::sleep;
use std::time::Duration;

use reqwest::{Client, Error as ReqwestError, RequestBuilder, Response};
use serde_json::Error as SerdeJsonError;

use crate::api_parameters::ApiParameters;
use crate::error::Error;
use crate::mode::Mode;
use crate::token::Token;
use crate::torrents::Torrents;

pub mod api_parameters;
pub mod api_parameters_builder;
pub mod category;
pub mod episode_info;
pub mod error;
pub mod format;
pub mod limit;
pub mod mode;
pub mod sort_by;
pub mod token;
pub mod torrent;
pub mod torrents;

/* The API has a 1req/2s limit. We take three extra second just to be sure. */
const REQUEST_TIME_LIMIT: u64 = 5;
const USER_AGENT: &str =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:73.0) Gecko/20100101 Firefox/73.0";
const ENDPOINT: &str = "https://torrentapi.org/pubapi_v2.php";

#[derive(Debug)]
pub struct RarBgApi {
    app_id: String,
    token: Token,
}

impl RarBgApi {
    /// Return the name of your app.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rarbg_api::RarBgApi;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = RarBgApi::new("RustExample").await;
    ///     let app_id = api.app_id();
    /// }
    /// ```
    pub fn app_id(&self) -> &str {
        &self.app_id.as_str()
    }

    /// Return the token associate to your app.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rarbg_api::RarBgApi;
    /// use rarbg_api::token::Token;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = RarBgApi::new("RustExample").await;
    ///     let token: &Token = api.token();
    /// }
    /// ```
    pub fn token(&self) -> &Token {
        &self.token
    }

    /// Create a new RARBG client.
    ///
    /// # Arguments
    ///
    /// * `app_id` - A string slice that holds the name of your app.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rarbg_api::RarBgApi;
    /// use rarbg_api::token::Token;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = RarBgApi::new("RustExample").await;
    /// }
    /// ```
    pub async fn new(app_id: &str) -> Self {
        RarBgApi {
            token: Token::new(app_id).await,
            app_id: app_id.to_string(),
        }
    }

    async fn request(
        &mut self,
        search_value: Option<&[(&str, &str)]>,
        mode: Mode,
        parameters: Option<&ApiParameters>,
    ) -> Result<Torrents, Error> {
        if !self.token.is_valid() {
            self.token = Token::new(self.app_id()).await;
        }
        sleep(Duration::new(REQUEST_TIME_LIMIT, 0));

        let client: Client = Client::builder().user_agent(USER_AGENT).build().unwrap();

        let mut request: RequestBuilder = client
            .get(ENDPOINT)
            .query(&[("mode", mode.as_str())])
            .query(&[("token", self.token().value())])
            .query(&[("app_id", self.app_id())]);

        if search_value.is_some() {
            request = request.query(search_value.unwrap());
        }

        if parameters.is_some() {
            let pm = parameters.unwrap();
            request = request
                .query(&[("ranked", *pm.ranked() as isize)])
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
                let categories = pm.categories().unwrap();
                let stringified_categories: Vec<&str> =
                    categories.iter().map(|c| c.as_str()).collect();
                let joined_categories: String = stringified_categories.join(";");
                request = request.query(&[("category", joined_categories)]);
            }
        }
        let response: Result<Response, ReqwestError> = request.send().await;

        let content = match response {
            Ok(res) => res.text().await,
            Err(reason) => panic!("{}", reason),
        };

        let text = match content {
            Ok(text) => text,
            Err(reason) => panic!("{}", reason),
        };

        let torrents: Result<Torrents, SerdeJsonError> = serde_json::from_str(text.as_str());
        match torrents {
            Ok(torrents) => Ok(torrents),
            Err(reason1) => {
                let api_error: Result<Error, SerdeJsonError> = serde_json::from_str(text.as_str());
                match api_error {
                    Ok(api_error) => Err(api_error),
                    Err(reason2) => panic!("First reason: {}. Second reason: {}", reason1, reason2),
                }
            }
        }
    }

    /// List the torrents available depending on parameters given.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rarbg_api::RarBgApi;
    /// use rarbg_api::token::Token;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut api = RarBgApi::new("RustExample").await;
    ///     // It will get the 25 last ranked torrents
    ///     let result = api.list(None).await;
    /// }
    /// ```
    pub async fn list(&mut self, parameters: Option<&ApiParameters>) -> Result<Torrents, Error> {
        self.request(None, Mode::List, parameters).await
    }

    /// Search torrents by its name with some or no parameters.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rarbg_api::RarBgApi;
    /// use rarbg_api::token::Token;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut api = RarBgApi::new("RustExample").await;
    ///     let result = api.search("Rick and Morty", None).await;
    /// }
    /// ```
    pub async fn search(
        &mut self,
        value: &str,
        parameters: Option<&ApiParameters>,
    ) -> Result<Torrents, Error> {
        self.request(Some(&[("search_string", value)]), Mode::Search, parameters)
            .await
    }

    /// Search torrents by its IMDB id with some or no parameters.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rarbg_api::RarBgApi;
    /// use rarbg_api::token::Token;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut api = RarBgApi::new("RustExample").await;
    ///     // tt2861424 is Rick and Morty
    ///     let result = api.search_by_imdb("tt2861424", None).await;
    /// }
    /// ```
    pub async fn search_by_imdb(
        &mut self,
        value: &str,
        parameters: Option<&ApiParameters>,
    ) -> Result<Torrents, Error> {
        self.request(Some(&[("search_imdb", value)]), Mode::Search, parameters)
            .await
    }

    /// Search torrents by its TVDB id with some or no parameters.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rarbg_api::RarBgApi;
    /// use rarbg_api::token::Token;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut api = RarBgApi::new("RustExample").await;
    ///     // 275274 is Rick and Morty
    ///     let result = api.search_by_tvdb("275274", None).await;
    /// }
    /// ```
    pub async fn search_by_tvdb(
        &mut self,
        value: &str,
        parameters: Option<&ApiParameters>,
    ) -> Result<Torrents, Error> {
        self.request(Some(&[("search_tvdb", value)]), Mode::Search, parameters)
            .await
    }

    /// Search torrents by its TMDB id with some or no parameters.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rarbg_api::RarBgApi;
    /// use rarbg_api::token::Token;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut api = RarBgApi::new("RustExample").await;
    ///     // 60625 is Rick and Morty
    ///     let result = api.search_by_tmdb("60625", None).await;
    /// }
    /// ```
    pub async fn search_by_tmdb(
        &mut self,
        value: &str,
        parameters: Option<&ApiParameters>,
    ) -> Result<Torrents, Error> {
        self.request(Some(&[("search_tmdb", value)]), Mode::Search, parameters)
            .await
    }
}
