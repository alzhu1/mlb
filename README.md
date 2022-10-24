# MLB Client in Rust

MLB client written in Rust. Uses [reqwest](https://crates.io/crates/reqwest) and [serde](https://crates.io/crates/serde).

Currently supports (on a per-season basis):
* Searching for a specific player and printing their statline
* Finding aggregate team stats
* Finding league and team leaders in various stat categories

### Shorter-term ideas for expansion:
* Expand unit testing and integ testing modules (lots of printing to stdout that should be captured/examined for testing)
* Better error handling (too many `unwrap` calls)
* Add support for more APIs (i.e. game day box score)

### Longer-term ideas for expansion:
* Convert CLI app into a desktop app with UI, using something like [Tauri](https://github.com/tauri-apps/tauri)
* Write stats to local files for caching/optimization (refresh every day or so)
* Look into converting API calls to async calls using [tokio](https://crates.io/crates/tokio)