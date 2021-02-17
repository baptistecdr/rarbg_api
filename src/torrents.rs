use crate::torrent::Torrent;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Torrents {
    torrent_results: Vec<Torrent>,
}

impl Torrents {
    /// Return a list of torrents.
    pub fn torrents(&self) -> &Vec<Torrent> {
        &self.torrent_results
    }
}
