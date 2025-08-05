// UG-Tab-Scraper - A basic rust API for getting data from Ultimate Guitar
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
use crate::network::{encode_string, get_raw_html};
use regex::{CaptureMatches, Regex};
use std::str::FromStr;
use ureq::{Error as ReqError};

const VALID_LINK_REGEX: &str = r"https://www\.ultimate-guitar\.com/search\.php\?(?:[search_type=\w+]+&)*value=.+";
const DATA_REGEX: &str = r"(?:&quot;id&quot;:(\d{2,}).+?song_id&quot;:(\d+).+?song_name&quot;:&quot;([^&]+).*?artist_name&quot;:&quot;([^&]+).*?type&quot;:&quot;([\w\s]+).+?votes&quot;:(\d*).*?rating&quot;:([\d\.]+)).*?&quot;tab_url&quot;:&quot;(https://tabs\.ultimate-guitar\.com/tab/[^/\.]+/[^/\.&]+)&quot;";
const BASE_SEARCH_URL: &str = "https://www.ultimate-guitar.com/search.php?search_type=title&value=";

pub fn get_search_results(query: &str) -> Result<Vec<SearchResult>, Error> {
        let search_url: String = BASE_SEARCH_URL.to_string() + &encode_string(query);
        validate_link(&search_url)?;
        let raw_html: String;
        match get_raw_html(&search_url) {
                Ok(d) => raw_html = d,
                Err(e) => match e {
                        ReqError::StatusCode(c) => match c {
                                404 => return Ok(vec![]),
                                _ => return Err(Error::RequestError(e.to_string())),
                        },
                        _ => return Err(Error::RequestError(e.to_string())),
                },
        }

        let regex = Regex::new(DATA_REGEX).unwrap();
        let captures = regex.captures_iter(&raw_html);
        match unwrap_results(captures) {
                Ok(r) => Ok(r),
                Err(_e) => Err(Error::UnexpectedResults),
        }
}

fn unwrap_results(matches: CaptureMatches) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let mut results: Vec<SearchResult> = Vec::new();
        for regex_match in matches {
                let result: SearchResult = SearchResult { song_id: u32::from_str(&regex_match[2])?,
                        tab_id: u32::from_str(&regex_match[1])?,
                        title: regex_match[3].to_string(),
                        artist: regex_match[4].to_string(),
                        data_type: get_data_type(&regex_match[5]).unwrap_or(DataSetType::default()),
                        rating_count: u32::from_str(&regex_match[6])?,
                        rating_value: f32::from_str(&regex_match[7])?,
                        url: regex_match[8].to_string() };
                println!("id: {}, tab: {}, title: {}, artist: {}, type: {:?}, ratings: {}, rating: {}, url: {}", result.song_id, result.tab_id, result.title, result.artist, result.data_type, result.rating_count, result.rating_value, result.url);
                results.push(result);
        }
        Ok(results)
}

fn validate_link(url: &str) -> Result<(), Error> {
        let regex = Regex::new(VALID_LINK_REGEX).unwrap();
        let captures = regex.captures(url);
        match captures {
                Some(_d) => Ok(()),
                None => Err(Error::InvalidURL),
        }
        
}

#[cfg(test)]
mod tests {
    use crate::search_scraper::{get_search_results, validate_link};

        #[test]
        fn search_link_validation() {
                let valid_links: Vec<&str> = vec!["https://www.ultimate-guitar.com/search.php?search_type=title&value=never%20gonna%20give%20you%20up#",
                        "https://www.ultimate-guitar.com/search.php?value=never%20gonna%20give%20you%20up#"];
                let invalid_links: Vec<&str> = vec!["https://www.ultimate-guitar.com/search.php"];
                for link in valid_links {
                        assert_eq!(validate_link(link), Ok(()));
                }
                for link in invalid_links {
                        assert_ne!(validate_link(link), Ok(()));
                }
        }

        #[test]
        fn search_results() {
                let valid_search_queries: Vec<&str> = vec!["NEVER GONNA GIVE YOU UP", "let it be", "Don't stop me now"];
                for query in valid_search_queries {
                        assert!(get_search_results(query).unwrap().len() >= 1);
                }
                let no_result_queries: Vec<&str> = vec!["this should_not return any #results!"];
                for query in no_result_queries {
                        assert!(get_search_results(query).unwrap().len() == 0);
                }
        }
}