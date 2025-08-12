# Changelog

## [0.2.2]
### Added
- Public function to validate a tab link
- Test in search_scraper to check if URLs from search results are existing
### Fixed
- Invalid URLs in search results if song name contains digits
- Search page amount specifier returning nothing for 0 or 1 additional search pages
### Changed
- The page amount specifier of get_search_results() to take an amount of pages instead of additional pages

## [0.2.1]
### Added
- Unescaping of \\" to "

## [0.2.0]
### Added
- `serde` `Serialize` and `Deserialize` traits to all types
- `serde` as a crates.io dependency