# Changelog

## [0.2.4]
### Fixed
- `Esus` and `Asus` being replaced with `Ebus` and `Abus` if German chord name replacement was enabled
### Updated dependencies
- percent-encoding to 2.3.2
- regex to 1.11.3
- ureq to 3.1.2
- serde to 1.0.228

## [0.2.3]
### Fixed
- Chord lines getting converted to section headers if square brackets were present in them
### Other changes
- Code clean-up

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
