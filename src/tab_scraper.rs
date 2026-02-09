// UG-Scraper - A basic rust API for getting data from Ultimate Guitar
// Copyright (C) 2025  Linus Tibert
//
// This program was originally published under the MIT licence as seen
// here: https://github.com/Lich-Corals/ug-tab-scraper-rs/blob/mistress/LICENCE

use crate::types::*;
use crate::network::*;
use crate::error::UGError;
use regex::Regex;
use std::str::FromStr;

const END_OF_CHORDS_DELIM: &str = "&quot;,&quot;revision_id&quot;:";
const START_OF_CHORDS_DELIM: &str = "&quot;:{&quot;wiki_tab&quot;:{&quot;content&quot;:&quot;";
const HTML_BLACKLIST: [&str; 1] = ["&quot;type&quot;:&quot;Video&quot;"];
const VALID_LINK_REGEX: &str = r"http[s]*:\/\/[www.]*[tabs.]*ultimate-guitar.com\/tab\/[\S]+";
const METADATA_REGEX: &str = r"&quot;adsupp_binary_blocked&quot;:null,&quot;meta&quot;:\{[&quot;capo&quot;:]*(\d*)[,]*&quot;[tonality&quot;:&quot;]*(\w*)[&quot;,&quot;]*tuning&quot;:\{&quot;name&quot;:&quot;([^:]*)&quot;,&quot;value&quot;:&quot;([^:]*)&quot;,";
const BASIC_DATA_REGEX: &str = r"tab&quot;:\{&quot;id&quot;:(\d+),&quot;song_id&quot;:(\d+),&quot;song_name&quot;:&quot;([^:]+)&quot;,&quot;artist_id&quot;:\d+,&quot;artist_name&quot;:&quot;([^:]+)&quot;,&quot;type&quot;:&quot;([\w\s]+)&quot;,&quot;part&quot;:";

/// Gets as much data about a tab as possible.
/// 
/// ## Arguments
/// * `url`: The URL to the tab
/// * `replace_german_names`: Wether to replace german chord names like `H` with `B`
///     * View [`crate::types::Line::replace_german_names`] for more information.
/// 
/// ## Example: 
/// ```
/// use ug_scraper::tab_scraper::get_song_data;
/// 
/// // Returns a wrapped Song object with associated data and replaced german chords
/// get_song_data("https://tabs.ultimate-guitar.com/tab/rick-astley/never-gonna-give-you-up-chords-521741", true);
/// ```
/// 
/// ## Possible errors
/// * `ureq::Error::*`
/// * [`crate::error::UGError`]
///     * `InvalidHTMLError`
///     * `InvalidURLError`
///     * `NoBasicDataMatchError`
///     * `UnexpectedWebResultError`
pub fn get_song_data(url: &str, replace_german_names: bool) -> Result<Song, Box<dyn std::error::Error>> {
        let raw_html: String;
        match get_raw_html(url) {
                Ok(s) => raw_html = s,
                Err(e) => return Err(e.into()),
        }
        let song_lines: Vec<Line> = get_tab_lines(&raw_html, replace_german_names)?;
        let song_metadata: Option<SongMetaData>;
        let basic_song_data: BasicSongData;
        match get_basic_metadata(&raw_html, url) {
                Ok(d) => {
                        song_metadata = extract_metadata(&raw_html);
                        basic_song_data = d;
                }
                Err(e) => return Err(e.into())
        }
        let song: Song = Song { lines: song_lines, metadata: song_metadata, basic_data: basic_song_data };
        Ok(song)
}

/// Get the basic metadata about a tab from valid HTML
/// 
/// ## Arguments
/// * `raw_html`: the raw HTML of a supported UG tab page
/// * `tab_link`: the link to the page
/// 
/// ## Example:
/// ```
/// use ug_scraper::tab_scraper::get_basic_metadata;
/// use ug_scraper::network::get_raw_html;
/// 
/// let url: &str = "https://tabs.ultimate-guitar.com/tab/rick-astley/never-gonna-give-you-up-chords-521741";
/// let raw_html: &str = &get_raw_html(url).unwrap_or_default();
/// let basic_data = get_basic_metadata(raw_html, url);
/// // Returns:
/// // BasicSongData { title: "Never Gonna Give You Up",
/// //                 artist: "Rick Astley",
/// //                 tab_link: "https://tabs.ultimate-guitar.com/tab/rick-astley/never-gonna-give-you-up-chords-521741",
/// //                 song_id: 196324,
/// //                 tab_id: 521741,
/// //                 data_type: Chords }
/// ```
/// 
/// ## Possible errors
/// * [`crate::error::UGError`]
///     * `InvalidHTMLError`
///     * `InvalidURLError`
///     * `NoBasicDataMatchError`
///     * `UnexpectedWebResultError`
pub fn get_basic_metadata(raw_html: &str, tab_link: &str) -> Result<BasicSongData, UGError> {
        validate_html(raw_html)?;
        validate_link(tab_link)?;

        let regex = Regex::new(BASIC_DATA_REGEX).unwrap();
        let captures = regex.captures(raw_html);
        if let Some(cap) = captures {
                let song_type: DataSetType = get_data_type(&cap[5]).unwrap_or_default();
                let tab_id = match u32::from_str(&cap[1]) {
                        Ok(i) => i,
                        Err(_e) => return Err(UGError::UnexpectedWebResultError),
                };
                let song_id = match u32::from_str(&cap[2]) {
                        Ok(i) => i,
                        Err(_e) => return Err(UGError::UnexpectedWebResultError),
                };
                let title = unescape_string(&cap[3]).to_string();
                let artist = unescape_string(&cap[4]).to_string();
                let song_basic_meta: BasicSongData = BasicSongData { title,
                        artist,
                        tab_link: tab_link.to_string(),
                        song_id,
                        tab_id,
                        data_type: song_type };
                Ok(song_basic_meta)
        } else {
                Err(UGError::NoBasicDataMatchError)
        }
}

/// Get a `Vec` with the lines of a tab
/// 
/// ## Arguments
/// * `raw_html`: the raw HTML of a supported UG tab page
/// * `replace_german_names`: Wether to replace german chord names like `H` with `B`
///     * View [`crate::types::Line::replace_german_names`] for more information.
/// 
/// ## Example:
/// ```
/// use ug_scraper::tab_scraper::get_tab_lines;
/// use ug_scraper::network::get_raw_html;
/// use ug_scraper::types::Line;
/// 
/// let url: &str = "https://tabs.ultimate-guitar.com/tab/rick-astley/never-gonna-give-you-up-chords-521741";
/// let raw_html: &str = &get_raw_html(url).unwrap_or_default();
/// 
/// // Ruturns lines of the tab with german chord names replaced
/// let lines_vec = get_tab_lines(raw_html, true).unwrap_or_default();
/// ```
/// 
/// ## Possible errors
/// * [`crate::error::UGError::InvalidHTMLError`]
pub fn get_tab_lines(raw_html: &str, replace_german_names: bool) -> Result<Vec<Line>, UGError> {
        validate_html(raw_html)?;
        let string_parts: Vec<&str> = raw_html.split(END_OF_CHORDS_DELIM).collect();
        let raw_data: &str = string_parts[0].split(START_OF_CHORDS_DELIM).collect::<Vec<&str>>()[1];
        let formatted_string_lines = unescape_string(raw_data);
        let lines: Vec<Line> = clean_and_evaluate(formatted_string_lines.lines(), replace_german_names);
        Ok(lines)
}

/// Checks if a given URL is leading to a tab on Ultimate Guitar
/// 
/// Returns Ok(()) if valid and Err(UGError::InvalidURLError) if invalid.
pub fn validate_link(url: &str) -> Result<(), UGError> {
        let regex = Regex::new(VALID_LINK_REGEX).unwrap();
        let captures = regex.captures(url);
        match captures {
                Some(_d) => Ok(()),
                None => Err(UGError::InvalidURLError),
        }
}

fn validate_html(raw_html: &str) -> Result<(), UGError> {
        for item in HTML_BLACKLIST {
                if raw_html.contains(item) {
                        return Err(UGError::InvalidHTMLError)
                }
        }
        if !raw_html.contains(START_OF_CHORDS_DELIM) || !raw_html.contains(END_OF_CHORDS_DELIM) {
                return Err(UGError::InvalidHTMLError)
        }
        Ok(())
}

fn extract_metadata(raw_html: &str) -> Option<SongMetaData> {
        let regex = Regex::new(METADATA_REGEX).unwrap();
        let captures = regex.captures(raw_html);
        let mut song_metadata: SongMetaData = SongMetaData::default();
        if let Some(cap) = captures {
                let mut capture_options: [Option<String>; 4] = [Some(cap[1].to_string()), 
                        Some(cap[2].to_string()), 
                        Some(cap[3].to_string()), 
                        Some(cap[4].to_string())];
                for i in 0..4 {
                        if capture_options[i].clone().unwrap().is_empty() {
                                capture_options[i] = None;
                        }
                        match i {
                                0 => song_metadata.capo = capture_options[i].clone(),
                                1 => song_metadata.tonality = capture_options[i].clone(),
                                2 => song_metadata.tuning_name = capture_options[i].clone(),
                                3 => song_metadata.tuning = capture_options[i].clone(),
                                _ => (),
                        }
                }                
        } else {
                return None
        }
        Some(song_metadata)
}

fn clean_and_evaluate(lines: std::str::Lines<'_>, replace_german_names: bool) -> Vec<Line> {
        let mut clean_lines: Vec<Line> = Vec::new();
        for line in lines {
                let mut line_type: DataType = DataType::Lyric;
                if line.contains("[ch]") {
                        line_type = DataType::Chord;
                }
                let mut clean_line: String = String::from(line);
                for key in ["[ch]", "[/ch]", "[tab]", "[/tab]"] {
                        clean_line = clean_line.replace(key, "")
                }
                if clean_line.contains("[") && clean_line.contains("]") && line_type != DataType::Chord {
                        line_type = DataType::SectionTitle;
                }
                let mut line = Line {line_type, text_data: clean_line};
                if replace_german_names {
                        line = line.replace_german_names();
                }
                clean_lines.push(line);
        }
        clean_lines
}

#[cfg(test)]
mod tests {
        use core::panic;
        use std::{thread::sleep, time::Duration};

        use super::*;

        #[test]
        fn get_lines_of_tab() {
                let tabs_to_get = ["https://tabs.ultimate-guitar.com/tab/367279", 
                        "https://tabs.ultimate-guitar.com/tab/queen/dont-stop-me-now-chords-519549"];
                for tab in tabs_to_get {
                        println!("Getting tab: {}", tab);
                        sleep(Duration::from_secs(1));
                        if let Ok(html) = &get_raw_html(tab) {
                                assert!(!matches!(get_tab_lines(html, true), Err(UGError::InvalidHTMLError)));
                        }
                }
        }

        #[test]
        fn tab_link_validation() {
                assert_eq!(validate_link("https://tabs.ultimate-guitar.com/tab/rick-astley/never-gonna-give-you-up-chords-521741"), Ok(()));
                assert_ne!(validate_link("tabs.ultimate-guitar.com/tab/refused/rather-be-dead-power-595658"), Ok(()));
        }

        #[test]
        fn type_detection() {
                let type_detection_checks: Vec<(DataSetType, &str)> = vec![(DataSetType::Chords, "https://tabs.ultimate-guitar.com/tab/queen/dont-stop-me-now-chords-519549"),
                        (DataSetType::Chords, "https://tabs.ultimate-guitar.com/tab/rick-astley/never-gonna-give-you-up-chords-521741"),
                        (DataSetType::Bass, "https://tabs.ultimate-guitar.com/tab/bloc-party/this-modern-love-bass-180218"),
                        (DataSetType::Tab, "https://tabs.ultimate-guitar.com/tab/led-zeppelin/stairway-to-heaven-tabs-9488"),
                        (DataSetType::Ukulele, "https://tabs.ultimate-guitar.com/tab/olli-schulz/wenn-es-gut-ist-ukulele-1381967"),
                        (DataSetType::Drums, "https://tabs.ultimate-guitar.com/tab/phil-collins/in-the-air-tonight-drums-880599"),
                        (DataSetType::Bass, "https://tabs.ultimate-guitar.com/tab/pink-floyd/empty-spaces-bass-147995")];
                for check in type_detection_checks {
                        println!("Testing valid url: {}", check.1);
                        sleep(Duration::from_secs(1));
                        if let Ok(html) = &get_raw_html(check.1) {
                                assert_eq!(get_basic_metadata(html, check.1).unwrap().data_type, check.0);
                        }
                }
        }

        #[test]
        fn validate_page_contents() {
                let valid_page_urls = vec!["https://tabs.ultimate-guitar.com/tab/queen/dont-stop-me-now-chords-519549",
                        "https://tabs.ultimate-guitar.com/tab/rick-astley/never-gonna-give-you-up-chords-521741",
                        "https://tabs.ultimate-guitar.com/tab/led-zeppelin/stairway-to-heaven-tabs-9488",
                        "https://tabs.ultimate-guitar.com/tab/olli-schulz/wenn-es-gut-ist-ukulele-1381967",
                        "https://tabs.ultimate-guitar.com/tab/phil-collins/in-the-air-tonight-drums-880599",
                        "https://tabs.ultimate-guitar.com/tab/blink-182/feeling-this-bass-104175",
                        "https://tabs.ultimate-guitar.com/tab/pink-floyd/empty-spaces-bass-147995",
                        "https://tabs.ultimate-guitar.com/tab/367279"];
                for valid_page_url in valid_page_urls {
                        println!("Testing valid url: {}", valid_page_url);
                        sleep(Duration::from_secs(1));
                        if let Ok(html) = &get_raw_html(valid_page_url) {
                                assert!(!matches!(validate_html(html), Err(UGError::InvalidHTMLError)));
                        }
                }

                let invalid_page_urls = vec!["https://tabs.ultimate-guitar.com/tab/refused/i-wanna-watch-the-world-burn-guitar-pro-5868920", 
                        "https://tabs.ultimate-guitar.com/tab/refused/rather-be-dead-power-595658", 
                        "https://tabs.ultimate-guitar.com/tab/the-beatles/let-it-be-video-781202",
                        "https://www.youtube.com/watch?v=dQw4w9WgXcQ&list=RDdQw4w9WgXcQ"];
                for invalid_page_url in invalid_page_urls {
                        println!("Testing invalid url: {}", invalid_page_url);
                        sleep(Duration::from_secs(1));
                        if let Ok(html) = &get_raw_html(invalid_page_url) {
                                assert!(matches!(validate_html(html), Err(UGError::InvalidHTMLError)));
                        }
                }
        }

        #[test]
        fn get_basic_data() {
                let test_sets: Vec<(&str, &str, &str, u32, u32)> = vec![("https://tabs.ultimate-guitar.com/tab/queen/dont-stop-me-now-chords-519549",
                                "Dont Stop Me Now", "Queen", 15591, 519549),
                        ("https://tabs.ultimate-guitar.com/tab/rick-astley/never-gonna-give-you-up-chords-521741",
                                "Never Gonna Give You Up", "Rick Astley", 196324, 521741),
                        ("https://tabs.ultimate-guitar.com/tab/led-zeppelin/stairway-to-heaven-tabs-9488",
                                "Stairway To Heaven", "Led Zeppelin", 31683, 9488),
                        ("https://tabs.ultimate-guitar.com/tab/olli-schulz/wenn-es-gut-ist-ukulele-1381967",
                                "Wenn Es Gut Ist", "Olli Schulz", 317511, 1381967),
                        ("https://tabs.ultimate-guitar.com/tab/phil-collins/in-the-air-tonight-drums-880599",
                                "In The Air Tonight", "Phil Collins", 138587, 880599),
                        ("https://tabs.ultimate-guitar.com/tab/blink-182/feeling-this-bass-104175",
                                "Feeling This", "Blink-182", 54209, 104175), // The title is actually wrong it the UG metadata. This is not a bug!
                        ("https://tabs.ultimate-guitar.com/tab/pink-floyd/empty-spaces-bass-147995",
                                "Empty Spaces", "Pink Floyd", 17357, 147995),
                        ("https://tabs.ultimate-guitar.com/tab/367279",
                                "Zu Spät", "Die Ärzte", 1577513, 367279)];

                for set in test_sets {
                        sleep(Duration::from_secs(1));
                        if let Ok(html) = &get_raw_html(set.0) {
                                let result = get_basic_metadata(html, set.0).unwrap();
                                assert_eq!(result.title, set.1);
                                assert_eq!(result.artist, set.2);
                                assert_eq!(result.song_id, set.3);
                                assert_eq!(result.tab_id, set.4)
                        }
                }
        }

        #[test]
        fn get_metadata() {
                let url_metadata_sets: Vec<(Option<SongMetaData>, &str)> = vec![(Some(SongMetaData { 
                                capo: Some(String::from("3")), 
                                tonality: None, 
                                tuning_name: Some(String::from("G C E A")), 
                                tuning: Some(String::from("G C E A")) }), "https://tabs.ultimate-guitar.com/tab/olli-schulz/wenn-es-gut-ist-ukulele-1381967"),
                        (None, "https://tabs.ultimate-guitar.com/tab/pink-floyd/empty-spaces-bass-147995"),
                        (None, "https://tabs.ultimate-guitar.com/tab/phil-collins/in-the-air-tonight-drums-880599"),
                        (Some(SongMetaData { capo: Some(String::from("1")), 
                                tonality: None, 
                                tuning_name: Some(String::from("Standard")), 
                                tuning: Some(String::from("E A D G B E")) }), "https://tabs.ultimate-guitar.com/tab/rick-astley/never-gonna-give-you-up-chords-521741"),
                        (Some(SongMetaData { capo: None, 
                                tonality: Some(String::from("F")), 
                                tuning_name: Some(String::from("Standard")), 
                                tuning: Some(String::from("E A D G B E")) }), "https://tabs.ultimate-guitar.com/tab/queen/dont-stop-me-now-chords-519549"),];
                for url_metadata_set in url_metadata_sets {
                        sleep(Duration::from_secs(1));
                        if let Ok(html) = &get_raw_html(url_metadata_set.1) {
                                match extract_metadata(html) {
                                        Some(d) => assert_eq!(d, url_metadata_set.0.unwrap()),
                                        None => {
                                                if url_metadata_set.0.is_some() {
                                                        panic!("Found metadata for song without known metadata.")
                                                }
                                        },
                                }

                        }
                }
        }
}
