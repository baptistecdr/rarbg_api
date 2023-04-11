<h3 align="center">RARBG TorrentAPI</h3>
<p align="center">
    A Rust wrapper for RARBG TorrentAPI
    <br>
    <a href="https://github.com/baptistecdr/rarbg_api/issues/new">Report bug</a>
    ·
    <a href="https://github.com/baptistecdr/rarbg_api/issues/new">Request feature</a>
</p>

<div align="center">

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/baptistecdr/rarbg_api/rust.yml?branch=main) ![Crates.io](https://img.shields.io/crates/v/rarbg_api) ![Crates.io](https://img.shields.io/crates/d/rarbg_api)  ![GitHub](https://img.shields.io/github/license/baptistecdr/rarbg_api)

</div>

## Description

This Rust module allows easy interaction with RARBG TorrentAPI.

In particular, it allows you to list or search torrents and to export them to a magnet file.

## Quick start

Add this to your `Cargo.toml`:

```toml
[dependencies]
rarbg_api = "1.0.41-rc.1"
tokio = { version = "1.27.0", features = ["full"] }
```

### Examples

```rust
extern crate rarbg_api;

use rarbg_api::api_parameters_builder::ApiParametersBuilder;
use rarbg_api::category::Category;
use rarbg_api::limit::Limit;
use rarbg_api::RarBgApi;
use rarbg_api::sort_by::SortBy;

#[tokio::main]
pub async fn main() {
    let mut api = RarBgApi::new("my_app_id").await;
    let parameters = ApiParametersBuilder::new()
        .limit(Limit::TwentyFive)
        .categories(vec![Category::TvUhdEpisodes, Category::TvHdEpisodes, Category::TvEpisodes])
        .sort_by(SortBy::Seeders)
        .build();
    let result = api.list(Some(&parameters)).await;
    match result {
        // Export all torrents found in the current directory.
        // Each file contains a magnet link that can be add in your Bittorrent client.
        Ok(result) => result.torrents().iter().for_each(|t| println!("Torrent exported to '{}'.", t.export(".").unwrap())),
        Err(reason) => println!("{}", reason.error())
    }
}
```

```rust
extern crate rarbg_api;

use rarbg_api::api_parameters_builder::ApiParametersBuilder;
use rarbg_api::category::Category;
use rarbg_api::limit::Limit;
use rarbg_api::RarBgApi;
use rarbg_api::sort_by::SortBy;

#[tokio::main]
pub async fn main() {
    let mut api = RarBgApi::new("my_app_id").await;
    let parameters = ApiParametersBuilder::new()
        .limit(Limit::TwentyFive)
        .categories(vec![Category::TvUhdEpisodes, Category::TvHdEpisodes, Category::TvEpisodes])
        .sort_by(SortBy::Seeders)
        .build();
    let result = api.search("Rick and Morty", Some(&parameters)).await;
    match result {
        // Export first torrent found in the current directory.
        // The file contains a magnet link that can be add in your Bittorrent client.
        Ok(result) => result.torrents().iter().take(1).for_each(|t| println!("Torrent exported to '{}'.", t.export(".").unwrap())),
        Err(reason) => println!("{}", reason.error())
    }
}
```

## Documentation

Documentation is available [here](https://docs.rs/rarbg_api).

## Bugs and feature requests

Have a bug or a feature request? Please first search for existing and closed issues. If your problem or idea is not
addressed yet, [please open a new issue](https://github.com/baptistecdr/rarbg_api/issues).

## Contributing

Contributions are welcome!

## Contributors

- [Orestis](https://github.com/omalaspinas) for his feedback

## Disclaimer

Downloading content that you (don't) have might be illegal in your country.

This is an unofficial rust package.
