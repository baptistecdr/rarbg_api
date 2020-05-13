extern crate serde_derive;
extern crate std;

use self::serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Category {
    #[serde(rename = "XXX (18+)")]
    Xxx = 4,
    #[serde(rename = "Movies/XVID")]
    MoviesXvid = 14,
    #[serde(rename = "Movies/XVID/720")]
    MoviesXvid720 = 48,
    #[serde(rename = "Movies/x264")]
    MoviesX264 = 17,
    #[serde(rename = "Movies/x264/1080")]
    MoviesX2641080 = 44,
    #[serde(rename = "Movies/x264/720")]
    MoviesX264720 = 45,
    #[serde(rename = "Movies/x264/3D")]
    MoviesX2643d = 47,
    #[serde(rename = "Movies/x264/4k")]
    MoviesX2644k = 50,
    #[serde(rename = "Movies/x265/4k")]
    MoviesX2654k = 51,
    #[serde(rename = "Movies/x264/4k/HDR")]
    MoviesX2654kHdr = 52,
    #[serde(rename = "Movies/Full BD")]
    MoviesFullBd = 42,
    #[serde(rename = "Movies/BD Remux")]
    MoviesBdRemux = 46,
    #[serde(rename = "TV Episodes")]
    TvEpisodes = 18,
    #[serde(rename = "TV HD Episodes")]
    TvHdEpisodes = 41,
    #[serde(rename = "TV UHD Episodes")]
    TvUhdEpisodes = 49,
    #[serde(rename = "Music/MP3")]
    MusicMp3 = 23,
    #[serde(rename = "Music/FLAC")]
    MusicFlac = 25,
    #[serde(rename = "Games/PC ISO")]
    GamesPcIso = 27,
    #[serde(rename = "Games/PC RIP")]
    GamesPcRip = 28,
    #[serde(rename = "Games/PS3")]
    GamesPs3 = 40,
    #[serde(rename = "Games/XBOX-360")]
    GamesXbox360 = 32,
    #[serde(rename = "Software/PC ISO")]
    SoftwarePcIso = 33,
    #[serde(rename = "Games/PS4")]
    GamesPs4 = 53,
}
