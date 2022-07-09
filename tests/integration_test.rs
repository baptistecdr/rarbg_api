extern crate rarbg_api;

use std::path::Path;
use rarbg_api::api_parameters_builder::ApiParametersBuilder;
use rarbg_api::category::Category;
use rarbg_api::format::Format;
use rarbg_api::limit::Limit;
use rarbg_api::RarBgApi;
use rarbg_api::sort_by::SortBy;

#[tokio::test]
async fn search_torrents_and_export_first_torrent() {
    let mut api = RarBgApi::new("integration-test").await;

    assert_eq!(api.app_id(), "integration-test");

    let parameters = ApiParametersBuilder::new()
        .limit(Limit::TwentyFive)
        .categories(vec![Category::TvUhdEpisodes, Category::TvHdEpisodes, Category::TvEpisodes])
        .sort_by(SortBy::Seeders)
        .build();
    // Specified options
    assert_eq!(*parameters.limit(), Limit::TwentyFive);
    assert_eq!(*parameters.categories().unwrap(), vec![Category::TvUhdEpisodes, Category::TvHdEpisodes, Category::TvEpisodes]);
    assert_eq!(*parameters.sort_by(), SortBy::Seeders);
    // Default options
    assert!(*parameters.ranked());
    assert_eq!(parameters.minimum_seeders(), None);
    assert_eq!(parameters.minimum_leechers(), None);
    assert_eq!(*parameters.format(), Format::Json);

    let result = (api.list(Some(&parameters)).await).unwrap();
    let torrents = result.torrents();
    let torrent = torrents.first().unwrap();
    let filepath = torrent.export(".").unwrap();
    assert!(Path::new(filepath.as_str()).exists());
}
