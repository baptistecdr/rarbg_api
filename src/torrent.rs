extern crate uuid;

use std::fmt;
use std::fs::File;
use std::io;
use std::io::Write;

use chrono::{DateTime, Utc};
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer, Serialize};

use crate::category::Category;
use crate::episode_info::EpisodeInfo;

use self::uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Torrent {
    title: Option<String>,
    filename: Option<String>,
    category: Category,
    download: String,
    seeders: Option<u32>,
    leechers: Option<u32>,
    size: Option<u128>,
    pubdate: Option<DateTime<Utc>>,
    episode_info: Option<EpisodeInfo>,
    #[serde(default, deserialize_with = "bool_from_int")]
    ranked: Option<bool>,
    info_page: Option<String>,
}

impl Torrent {
    /// Return the title.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    /// Return the filename
    ///
    /// Only available when `format` is set to `Format::Json`.
    pub fn filename(&self) -> Option<&String> {
        self.filename.as_ref()
    }

    /// Return the category that the torrent belongs to.
    pub fn category(&self) -> &Category {
        &self.category
    }

    /// Return a magnet link.
    pub fn download(&self) -> &str {
        self.download.as_str()
    }

    /// Return the number of seeders available.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn seeders(&self) -> Option<&u32> {
        self.seeders.as_ref()
    }

    /// Return the number of leechers.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn leechers(&self) -> Option<&u32> {
        self.leechers.as_ref()
    }

    /// Return the size in bytes.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn size(&self) -> Option<&u128> {
        self.size.as_ref()
    }

    /// Return the publication date.
    ///
    /// DateTime is always synchronize with UTC.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn pub_date(&self) -> Option<&DateTime<Utc>> {
        self.pubdate.as_ref()
    }

    /// Return the episode info.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn episode_info(&self) -> Option<&EpisodeInfo> {
        self.episode_info.as_ref()
    }

    /// Return true if it's a scene, rarbg or rartv releases, otherwise false.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn ranked(&self) -> Option<&bool> {
        self.ranked.as_ref()
    }

    /// Return an HTTP link that redirect to the torrent page.
    ///
    /// Only available when `format` is set to `Format::JsonExtended`.
    pub fn info_page(&self) -> Option<&String> {
        self.info_page.as_ref()
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
                None => Uuid::new_v4().to_string(),
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

// https://github.com/serde-rs/serde/issues/1344#issuecomment-410309140
fn bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(Some(false)),
        1 => Ok(Some(true)),
        other => Err(Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}
