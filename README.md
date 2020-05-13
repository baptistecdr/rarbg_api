<h3 align="center">RARBG TorrentAPI</h3>
<p align="center">
    A Rust wrapper for RARBG TorrentAPI
    <br>
    <a href="https://github.com/baptistecdr/rarbg_api/issues/new">Report bug</a>
    ·
    <a href="https://github.com/baptistecdr/rarbg_api/issues/new">Request feature</a>
</p>

## Table of contents
- [Quick start](#quick-start)
- [Bugs and feature requests](#bugs-and-feature-requests)
- [Contributing](#contributing)
- [Contibutors](#contributors)

## Description
This Rust module allows easy interaction with RARBG TorrentAPI.

In particular, it allows to list or search torrents and to export them to a magnet file.

## Quick start

Add this to your `Cargo.toml`:
```toml
[dependencies]
rarbg_api = "0.1.0"
```
### Example
```rust
extern crate rarbg_api;

use rarbg_api::RarBgApi;

fn main() {
    let mut api = RarBgApi::new("my_app_id");
    let result = api.list(None);
    match result {
        Ok(torrent_results) => println!("{:?}", torrent_results.torrents()),
        Err(reason) => println!("{:?}", reason)
    }
}
```

## Bugs and feature requests
Have a bug or a feature request? Please first search for existing and closed issues. If your problem or idea is not addressed yet, [please open a new issue](https://github.com/baptistecdr/rarbg_api/issues).

## Contributing
Contributions are welcome!

## Contributors

## Disclaimer

Downloading content that you (don't) have might be illegal in your country.

This is an unofficial rust package.
