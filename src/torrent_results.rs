extern crate serde_derive;

use self::serde_derive::{Deserialize, Serialize};
use torrent_result::TorrentResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct TorrentResults {
    torrent_results: Vec<TorrentResult>,
}

impl TorrentResults {
    pub fn torrents(&self) -> &Vec<TorrentResult> {
        &self.torrent_results
    }

    pub fn new(torrent_results: Vec<TorrentResult>) -> Self {
        TorrentResults {
            torrent_results
        }
    }
}
