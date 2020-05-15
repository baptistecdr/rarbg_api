extern crate serde_derive;

use self::serde_derive::{Deserialize, Serialize};
use torrent::Torrent;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Torrents {
    torrent_results: Vec<Torrent>,
}

impl Torrents {
    pub fn torrents(&self) -> &Vec<Torrent> {
        &self.torrent_results
    }

    pub fn new(torrent_results: Vec<Torrent>) -> Self {
        Torrents {
            torrent_results
        }
    }
}
