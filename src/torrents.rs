extern crate serde_derive;

use torrent::Torrent;

use self::serde_derive::{Deserialize, Serialize};

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
