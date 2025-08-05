## UG-Scraper-RS
[![GitHub](https://img.shields.io/badge/-GitHub-181717?style=for-the-badge&logo=GitHub&logoColor=white)](https://github.com/Lich-Corals/ug-scraper-rs)
[![Crates](https://img.shields.io/badge/-Crates.io-ffc933?style=for-the-badge&logo=cratesio&logoColor=black)](https://www.coff.ee/lichcorals)
[![Coffee](https://img.shields.io/badge/-Buy%20me%20a%20coffee-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black)](https://www.coff.ee/lichcorals)

This crate is able to fetch search results and tab data from Ultimate Guitar using web-scraping.
It aims to be easy to use and as reliable as possible.

> #### IMPORTANT NOTICE  
> This crate fetches data in a way not intended by the host (Ultimate Guitar).
> Thus, the used RegEx patterns may obsolete if the host makes major changes on their website.
>
> When this happens, I'll publish a patch as soon as possible, but there is no warranty of the promptness of publication.

### Features
The crate provides functions for getting search results for a certain query and for downloading specific kinds of tabs from Ultimate Guitar.

#### Downloading tabs
Supported tab formats are: Chords, Tabs, Bass Tabs, Ukulele Chords, Drums

When a tab is downloaded, a `Song` is returned, containing basic metadata like the artist or song name, optional metadata like the capo position or tuning and a `Vec` of the tab's lines with their associated type.