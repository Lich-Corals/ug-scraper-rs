// UG-Tab-Scraper - A rust API for downloading UG tabs
// Copyright (C) 2025  Linus Tibert
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public Licence as published
// by the Free Software Foundation, either version 3 of the Licence, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public Licence for more details.
//
// You should have received a copy of the GNU Affero General Public Licence
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::types_and_constants::*;
use ureq::{get, Error as ReqError};
use html_escape::{decode_html_entities};
use regex::Regex;

pub fn get_song_data_from_url(url: &str) -> Result<Song, Error> {
        let raw_html: String;
        match get_raw_html(url) {
                Ok(s) => raw_html = s,
                Err(e) => match try_to_fix_url(e, url) {
                        Ok(s) => raw_html = s,
                        Err(e) => return Err(Error::RequestError(e.to_string())),
                },
        }
        let song_lines: Vec<Line> = get_song_lines(&raw_html)?;
        let song_meta_data: Option<SongMetaData>;
        let basic_song_data: BasicSongData;
        match get_basic_meta_data(&raw_html, url) {
                Ok(d) => {
                        song_meta_data = extract_meta_data(&raw_html);
                        basic_song_data = d;
                }
                Err(e) => return Err(e)
        }
        let song: Song = Song { lines: song_lines, metadata: song_meta_data, basic_data: basic_song_data };
        Ok(song)
}

pub fn get_basic_meta_data(raw_html: &str, tab_link: &str) -> Result<BasicSongData, Error> {
        validate_html(raw_html)?;
        validate_link(tab_link)?;

        let regex = Regex::new(BASIC_DATA_REGEX).unwrap();
        let captures = regex.captures(raw_html);
        if captures.is_some() {
                let captures = captures.unwrap();
                let song_type: DataSetType;
                match &captures[4] {
                        "Chords" => song_type = DataSetType::Chords,
                        "Tabs" => song_type = DataSetType::Tab,
                        "Bass Tabs" => song_type = DataSetType::Bass,
                        "Ukulele Chords" => song_type = DataSetType::Ukulele,
                        "Drum Tabs" => song_type = DataSetType::Drums,
                        _ => return Err(Error::UnknownType),
                }
                let tab_id = captures[1].to_string();
                let title = captures[2].to_string();
                let artist = captures[3].to_string();
                println!("\"{}\", \"{}\", \"{}\"", title, artist, tab_id);
                let song_basic_meta: BasicSongData = BasicSongData { title: title,
                        artist: artist,
                        tab_link: tab_link.to_string(),
                        tab_id: tab_id,
                        data_type: song_type };
                return Ok(song_basic_meta)
        } else {
                return Err(Error::NoBasicDataMatch)
        }
}

pub fn get_raw_html(url: &str) -> Result<String, ReqError> {
        let mut response =  get(url).call()?;
        let raw_html = response.body_mut().read_to_string()?;
        Ok(raw_html)
}

pub fn validate_html(raw_html: &str) -> Result<(), Error> {
        for item in HTML_BLACKLIST {
                if raw_html.contains(item) {
                        return Err(Error::InvalidPageType)
                }
        }
        if !raw_html.contains(START_OF_CHORDS_DELIM) || !raw_html.contains(END_OF_CHORDS_DELIM) {
                return Err(Error::InvalidPageType)
        }
        Ok(())
}

pub fn get_song_lines(raw_html: &str) -> Result<Vec<Line>, Error> {
        validate_html(raw_html)?;
        let string_parts: Vec<&str> = raw_html.split(END_OF_CHORDS_DELIM).collect();
        let raw_data: &str = string_parts[0].split(START_OF_CHORDS_DELIM).collect::<Vec<&str>>()[1];
        let formatted_string_lines = unescape_string(raw_data);
        Ok(clean_and_evaluate(formatted_string_lines.lines()))
}

fn validate_link(url: &str) -> Result<(), Error> {
        let regex = Regex::new(VALID_LINK_REGEX).unwrap();
        let captures = regex.captures(url);
        match captures {
                Some(_d) => Ok(()),
                None => Err(Error::InvalidURL),
        }
        
}

fn unescape_string(string: &str) -> String{
        decode_html_entities(string).to_string().replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\r", "\r")
                .replace("\\n", "\n")
}

fn extract_meta_data(raw_html: &str) -> Option<SongMetaData> {
        let regex = Regex::new(META_DATA_REGEX).unwrap();
        let captures = regex.captures(raw_html);
        let mut song_metadata: SongMetaData = SongMetaData::default();
        if captures.is_some() {
                let captures = captures.unwrap();
                let mut capture_options: [Option<String>; 4] = [Some(captures[1].to_string()), 
                        Some(captures[2].to_string()), 
                        Some(captures[3].to_string()), 
                        Some(captures[4].to_string())];
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
        return Some(song_metadata)
}

fn clean_and_evaluate(lines: std::str::Lines<'_>) -> Vec<Line> {
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
                if clean_line.contains("[") && clean_line.contains("]") {
                        line_type = DataType::SectionTitle;
                }
                clean_lines.push(Line {line_type: line_type, text_data: clean_line});
        }
        clean_lines
}

fn try_to_fix_url(error: ReqError, url: &str) -> Result<String, ReqError> {
        match error {
                ReqError::BadUri(_e) => return get_raw_html(&("https://".to_owned() + url)),
                _ => return Err(ReqError::BadUri(String::from(url)))
        }
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn validate_url() {
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
                        println!("Testing url: {}", stringify!(get_type(&get_raw_html(check.1).unwrap()).unwrap()));
                        assert_eq!(get_basic_meta_data(&get_raw_html(check.1).unwrap(), check.1).unwrap().data_type, check.0);
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
                        "https://tabs.ultimate-guitar.com/tab/pink-floyd/empty-spaces-bass-147995"];
                for valid_page_url in valid_page_urls {
                        println!("Testing valid url: {}", valid_page_url);
                        assert!(!matches!(validate_html(&get_raw_html(valid_page_url).unwrap()), Err(Error::InvalidPageType)));
                }

                let invalid_page_urls = vec!["https://tabs.ultimate-guitar.com/tab/refused/i-wanna-watch-the-world-burn-guitar-pro-5868920", 
                        "https://tabs.ultimate-guitar.com/tab/refused/rather-be-dead-power-595658", 
                        "https://tabs.ultimate-guitar.com/tab/the-beatles/let-it-be-video-781202",
                        "https://www.youtube.com/watch?v=dQw4w9WgXcQ&list=RDdQw4w9WgXcQ&start_radio=1"];
                for invalid_page_url in invalid_page_urls {
                        println!("Testing invalid url: {}", invalid_page_url);
                        assert!(matches!(validate_html(&get_raw_html(invalid_page_url).unwrap()), Err(Error::InvalidPageType)));
                }
        }

        #[test]
        fn get_basic_data() {
                let test_sets: Vec<(&str, &str, &str, &str)> = vec![("https://tabs.ultimate-guitar.com/tab/queen/dont-stop-me-now-chords-519549",
                                "Dont Stop Me Now", "Queen", "15591"),
                        ("https://tabs.ultimate-guitar.com/tab/rick-astley/never-gonna-give-you-up-chords-521741",
                                "Never Gonna Give You Up", "Rick Astley", "196324"),
                        ("https://tabs.ultimate-guitar.com/tab/led-zeppelin/stairway-to-heaven-tabs-9488",
                                "Stairway To Heaven", "Led Zeppelin", "31683"),
                        ("https://tabs.ultimate-guitar.com/tab/olli-schulz/wenn-es-gut-ist-ukulele-1381967",
                                "Wenn Es Gut Ist", "Olli Schulz", "317511"),
                        ("https://tabs.ultimate-guitar.com/tab/phil-collins/in-the-air-tonight-drums-880599",
                                "In The Air Tonight", "Phil Collins", "138587"),
                        ("https://tabs.ultimate-guitar.com/tab/blink-182/feeling-this-bass-104175",
                                "Feeling This", "Blink-182", "54209"), // The title is actually wrong it the UG meta data. This is not a bug!
                        ("https://tabs.ultimate-guitar.com/tab/pink-floyd/empty-spaces-bass-147995",
                                "Empty Spaces", "Pink Floyd", "17357")];

                for set in test_sets {
                        let result = get_basic_meta_data(&get_raw_html(set.0).unwrap(), set.0).unwrap();
                        assert_eq!(result.title, set.1);
                        assert_eq!(result.artist, set.2);
                        assert_eq!(result.tab_id, set.3);
                }
        }

        #[test]
        fn get_meta_data() {
                let url_meta_data_sets: Vec<(Option<SongMetaData>, &str)> = vec![(Some(SongMetaData { 
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
                for url_meta_data_set in url_meta_data_sets {
                        println!("Testing url: {}", stringify!(get_type(&get_raw_html(url_meta_data_set.1).unwrap()).unwrap()));
                        match extract_meta_data(&get_raw_html(url_meta_data_set.1).unwrap()) {
                                Some(d) => assert_eq!(d, url_meta_data_set.0.unwrap()),
                                None => {
                                        if url_meta_data_set.0.is_some() {
                                                panic!("Found meta data for song without known meta data.")
                                        }
                                },
                        }
                }
        }
}
