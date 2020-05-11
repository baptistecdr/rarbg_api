extern crate serde_derive;
extern crate uuid;

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use self::serde_derive::{Deserialize, Serialize};
use self::uuid::Uuid;
use std::io;

use category::Category;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TorrentResult {
    title: Option<String>,
    filename: Option<String>,
    category: Category,
    download: String,
    seeders: Option<i64>,
    leechers: Option<i64>,
    size: Option<i64>,
    pubdate: Option<String>,
    episode_info: Option<HashMap<String, Option<String>>>,
    ranked: Option<i64>,
    info_page: Option<String>,
}

impl TorrentResult {
    pub fn title(&self) -> &Option<String> { &self.title }

    pub fn filename(&self) -> &Option<String> { &self.filename }

    pub fn download(&self) -> &String { &self.download }

    pub fn seeders(&self) -> &Option<i64> {
        &self.seeders
    }

    pub fn leechers(&self) -> &Option<i64> {
        &self.leechers
    }

    pub fn size(&self) -> &Option<i64> {
        &self.size
    }

    pub fn ranked(&self) -> &Option<i64> {
        &self.ranked
    }

    pub fn info_page(&self) -> &Option<String> {
        &self.info_page
    }

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
            Ok(_) => Ok(format!("{} exported to '{}'.", filename, filepath)),
            Err(reason) => Err(reason),
        }
    }
}

