use chrono::NaiveDate;

use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EpisodeInfo {
    imdb: Option<String>,
    tvrage: Option<String>,
    tvdb: Option<String>,
    themoviedb: Option<String>,
    airdate: Option<String>,
    epnum: Option<String>,
    seasonnum: Option<String>,
    title: Option<String>,
}

impl EpisodeInfo {
    /// Return the TMDB id.
    pub fn imdb_id(&self) -> Option<&String> {
        self.imdb.as_ref()
    }

    /// Return the TVRage id.
    pub fn tvrage_id(&self) -> Option<&String> {
        self.tvrage.as_ref()
    }

    /// Return the TVDB id.
    pub fn tvdb_id(&self) -> Option<&String> {
        self.tvdb.as_ref()
    }

    /// Return the TMDB id.
    pub fn tmdb_id(&self) -> Option<&String> {
        self.themoviedb.as_ref()
    }

    /// Return the airing date.
    pub fn air_date(&self) -> Option<NaiveDate> {
        if self.airdate.is_some() {
            let date = self.airdate.as_ref().unwrap();
            if date == "0000-00-00" {
                return None;
            }
            return Some(NaiveDate::from_str(date.as_str()).unwrap());
        }
        return None;
    }

    /// Return the episode number.
    pub fn episode_number(&self) -> Option<&String> {
        self.epnum.as_ref()
    }

    /// Return the season number.
    pub fn season_number(&self) -> Option<&String> {
        self.seasonnum.as_ref()
    }

    /// Return the title.
    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }
}
