extern crate core;
extern crate serde_derive;
extern crate std;

use self::serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Category {
    #[serde(rename = "XXX (18+)")]
    Xxx,
    #[serde(rename = "Movies/XVID")]
    MoviesXvid,
    #[serde(rename = "Movies/XVID/720")]
    MoviesXvid720,
    #[serde(rename = "Movies/x264")]
    MoviesX264,
    #[serde(rename = "Movies/x264/1080")]
    MoviesX2641080,
    #[serde(rename = "Movies/x264/720")]
    MoviesX264720,
    #[serde(rename = "Movies/x264/3D")]
    MoviesX2643d,
    #[serde(rename = "Movies/x264/4k")]
    MoviesX2644k,
    #[serde(rename = "Movies/x265/4k")]
    MoviesX2654k,
    #[serde(rename = "Movies/x264/4k/HDR")]
    MoviesX2654kHdr,
    #[serde(rename = "Movies/Full BD")]
    MoviesFullBd,
    #[serde(rename = "Movies/BD Remux")]
    MoviesBdRemux,
    #[serde(rename = "TV Episodes")]
    TvEpisodes,
    #[serde(rename = "TV HD Episodes")]
    TvHdEpisodes,
    #[serde(rename = "TV UHD Episodes")]
    TvUhdEpisodes,
    #[serde(rename = "Music/MP3")]
    MusicMp3,
    #[serde(rename = "Music/FLAC")]
    MusicFlac,
    #[serde(rename = "Games/PC ISO")]
    GamesPcIso,
    #[serde(rename = "Games/PC RIP")]
    GamesPcRip,
    #[serde(rename = "Games/PS3")]
    GamesPs3,
    #[serde(rename = "Games/XBOX-360")]
    GamesXbox360,
    #[serde(rename = "Software/PC ISO")]
    SoftwarePcIso,
    #[serde(rename = "Games/PS4")]
    GamesPs4,
}

impl Category {
    pub fn as_str(&self) -> &str {
        match self {
            Category::Xxx => "4",
            Category::MoviesXvid => "14",
            Category::MoviesXvid720 => "48",
            Category::MoviesX264 => "17",
            Category::MoviesX2641080 => "44",
            Category::MoviesX264720 => "45",
            Category::MoviesX2643d => "47",
            Category::MoviesX2644k => "50",
            Category::MoviesX2654k => "51",
            Category::MoviesX2654kHdr => "52",
            Category::MoviesFullBd => "42",
            Category::MoviesBdRemux => "46",
            Category::TvEpisodes => "18",
            Category::TvHdEpisodes => "41",
            Category::TvUhdEpisodes => "49",
            Category::MusicMp3 => "23",
            Category::MusicFlac => "25",
            Category::GamesPcIso => "27",
            Category::GamesPcRip => "28",
            Category::GamesPs3 => "40",
            Category::GamesXbox360 => "32",
            Category::SoftwarePcIso => "33",
            Category::GamesPs4 => "53",
        }
    }
}
