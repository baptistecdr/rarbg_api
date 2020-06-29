extern crate serde_derive;
extern crate uuid;

use std::fmt;
use std::fs::File;
use std::io;
use std::io::Write;

use chrono::{DateTime, Utc};

use category::Category;

use self::serde_derive::{Deserialize, Serialize};
use self::uuid::Uuid;
use episode_info::EpisodeInfo;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Torrent {
    title: Option<String>,
    filename: Option<String>,
    category: Category,
    download: String,
    seeders: Option<i64>,
    leechers: Option<i64>,
    size: Option<i64>,
    pubdate: Option<DateTime<Utc>>,
    episode_info: Option<EpisodeInfo>,
    ranked: Option<i64>,
    info_page: Option<String>,
}

impl Torrent {
    /// Return the title.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn title(&self) -> &Option<String> { &self.title }

    /// Return the filename
    ///
    /// Only available when `format` is set to `Format::Json`.
    pub fn filename(&self) -> &Option<String> { &self.filename }

    /// Return the category that the torrent belongs to.
    pub fn category(&self) -> &Category {
        &self.category
    }

    /// Return a magnet link.
    pub fn download(&self) -> &String { &self.download }

    /// Return the number of seeders available.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn seeders(&self) -> &Option<i64> {
        &self.seeders
    }

    /// Return the number of leechers.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn leechers(&self) -> &Option<i64> {
        &self.leechers
    }

    /// Return the size in bytes.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn size(&self) -> &Option<i64> {
        &self.size
    }

    /// Return the publication date.
    ///
    /// DateTime is always synchronize with UTC.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn pub_date(&self) -> &Option<DateTime<Utc>> {
        &self.pubdate
    }

    /// Return the episode info.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn episode_info(&self) -> &Option<EpisodeInfo> {
        &self.episode_info
    }

    /// Return true if it's a scene, rarbg or rartv releases, otherwise false.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn ranked(&self) -> &Option<i64> {
        &self.ranked
    }

    /// Return an HTTP link that redirect to the torrent page.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn info_page(&self) -> &Option<String> {
        &self.info_page
    }

    /// Export the torrent to a magnet file using its title, filename or UUID as filename.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds a path to a **folder**
    ///
    pub fn export(&self, path: &str) -> Result<String, io::Error> {
        let filename = match self.title() {
            Some(title) => title.clone(),
            None => match self.filename() {
                Some(filename) => filename.clone(),
                None => Uuid::new_v4().to_string()
            },
        };
        let filepath = format!("{}/{}.magnet", path, filename);
        let file = File::create(&filepath);
        if file.is_err() {
            return Err(file.unwrap_err());
        }
        match file.unwrap().write_all(self.download.as_bytes()) {
            Ok(_) => Ok(filepath),
            Err(reason) => Err(reason),
        }
    }
}

impl fmt::Display for Torrent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
