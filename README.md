## UG-Scraper-RS
[![Codeberg](https://img.shields.io/badge/-Codeberg-2185D0?style=for-the-badge&logo=Codeberg&logoColor=white)](https://codeberg.org/Lich-Corals/ug-scraper-rs)
[![Docs](https://img.shields.io/badge/docs.rs-000000?style=for-the-badge&logo=docs.rs)](https://docs.rs/ug-scraper/)
[![Crates](https://img.shields.io/badge/-Crates.io-ffc933?style=for-the-badge&logo=rust&logoColor=black)](https://crates.io/crates/ug-scraper)
[![Coffee](https://img.shields.io/badge/-Buy%20me%20a%20coffee-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black)](https://www.coff.ee/lichcorals)

> [!CAUTION]  
> For _reasons_, this project has moved to Codeberg.
>
> The GitHub page is not archived to enable regular tests for quick responses to changes on Ultimate Guitar.
> If you still want to support this project, consider starring it on Codeberg.
> 
> [![Codeberg](https://img.shields.io/badge/-view_on_codeberg-2185D0?style=for-the-badge&logo=Codeberg&logoColor=white)](https://codeberg.org/Lich-Corals/ug-scraper-rs)

This crate is able to fetch search results and tab data from Ultimate Guitar using web-scraping.
It aims to be easy to use and as reliable as possible.

> [!IMPORTANT]  
> This crate fetches data in a way not intended by the host (Ultimate Guitar).
> Thus, the used RegEx patterns may obsolete if the host makes major changes on their website.
>
> When this happens, I'll publish a patch as soon as possible, but there is no warranty of the promptness of publication.

### Features
The crate provides functions for getting search results for a certain query and for downloading specific kinds of tabs from Ultimate Guitar.

All types implement the `serde` traits `Serialize` and `Deserialize`.

#### Downloading tabs
Supported tab formats are: Chords, Tabs, Bass Tabs, Ukulele Chords, Drums

When a tab is downloaded, a `Song` is returned, containing basic metadata like the artist or song name, optional metadata like the capo position or tuning and a `Vec` of the tab's lines with their associated type.

There is an option to automatically detect and replace the German names of chords with their English equivalents.

#### Searching for tabs
There is module for searching for a certain query on UG is available.
It returns a `Vec` of `SearchResult` objects containing basic metadata about the tab and the associated URL which may be downloaded like described above.

The search results can be limited to a specific amount of result pages on UG to accelerate the search process by getting fewer results.

**Note**: The search API also returns types of tabs which are not downloadable with this tool. Results should be filtered before downloading them automatically.

### Code examples
Downloading a specific tab:
```rust
use ug_scraper::tab_scraper::get_song_data;

// Downloads and evaluates the tab behind the given URL. The second argument is set to true to automatically replace German chord names. (although this tab doesn't have any)
get_song_data("https://tabs.ultimate-guitar.com/tab/rick-astley/never-gonna-give-you-up-chords-521741", true);
```

Searching for a specific song or artist:
```rust
use ug_scraper::search_scraper::get_search_results;

// Searches for a given query on only the first page of results
get_search_results("Never gonna give you up", 1);
```

### Documentation
#### Code documentation
For further code examples and a detailed documentation, head over to [docs.rs](https://docs.rs/ug-scraper/).
