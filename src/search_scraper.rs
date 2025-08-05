// UG-Scraper - A basic rust API for getting data from Ultimate Guitar
// Copyright (C) 2025  Linus Tibert
//
// This program was originally published under the MIT licence as seen
// here: https://github.com/Lich-Corals/ug-tab-scraper-rs/blob/mistress/LICENCE

use crate::types_and_constants::*;
use crate::network::{encode_string, get_raw_html, unescape_string};
use regex::{CaptureMatches, Regex};
use std::str::FromStr;
use ureq::{Error as ReqError};

const DATA_REGEX: &str = r"(?:&quot;id&quot;:(\d{2,}).+?song_id&quot;:(\d+).+?song_name&quot;:&quot;(.*?)&quot;.*?artist_name&quot;:&quot;(.*?)&quot;.*?type&quot;:&quot;(.+?)&quot;.+?votes&quot;:(\d*).*?rating&quot;:([\d\.]+)).*?&quot;tab_url&quot;:&quot;(https:\/\/tabs\.ultimate-guitar\.com\/tab\/.*?\d+)";
const BASE_SEARCH_URL: &str = "https://www.ultimate-guitar.com/search.php?search_type=title&value=";

/// Get search results for a query
/// 
/// ## Arguments:
/// * `query`: The query to search for; can be a song title or an author.
/// * `max_additional_pages`: The amount of pages to search after the first page of results
///     * The function will automatically stop searching if a page is empty. If you want all results, you may use `u8::MAX` without having any performance problems.
/// 
/// ## Example:
/// ```
/// use ug_scraper::search_scraper::get_search_results;
/// 
/// // Only gets results of the first page
/// get_search_results("Never gonna give you up", 0);
/// 
/// // Gets results of the first three pages
/// get_search_results("Never gonna give you up", 2);
/// ```
/// 
/// Returns an empty vector if there are no results.
/// 
/// ## Possible errors
/// * `ureq::Error::*`
/// * `ug_scraper::error::UGError::UnexpectedWebResultError`
pub fn get_search_results(query: &str, max_additional_pages: u8) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let mut results: Vec<SearchResult> = vec![];
        for i in 1..max_additional_pages {
                match search_page(query, i) {
                        Ok(mut r) => {
                                if r.len() == 0 {
                                        return Ok(results)
                                }
                                results.append(&mut r);
                        },
                        Err(e) => return Err(e)
                }
        }
        Ok(results)
}

/// Get search results of a single page
/// 
/// ## Arguments:
/// * `query`: The query to search for; can be a song title or an author.
/// * `page_to_search`: The page of results to search
/// 
/// ## Example:
/// ```
/// use ug_scraper::search_scraper::search_page;
/// 
/// // Returns results from results page 1
/// search_page("We are number one", 1);
/// 
/// // Returns an empty Vec because results page 0 is searched
/// search_page("empty spaces", 0);
/// ```
/// 
/// ## Possible errors
/// * `ureq::Error::*`
/// * `ug_scraper::error::UGError::UnexpectedWebResultError`
pub fn search_page(query: &str, page_to_search: u8) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let search_url: String = BASE_SEARCH_URL.to_string() + &encode_string(query) + "&page=" + &page_to_search.to_string();
        let raw_html: String;
        match get_raw_html(&search_url) {
                Ok(d) => raw_html = d,
                Err(e) => match e {
                        ReqError::StatusCode(c) => match c {
                                404 => return Ok(vec![]),
                                _ => return Err(e.into()),
                        },
                        _ => return Err(e.into()),
                },
        }
        let regex = Regex::new(DATA_REGEX).unwrap();
        let captures = regex.captures_iter(&raw_html);
        match unwrap_results(captures) {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into()),
        }
}

fn unwrap_results(matches: CaptureMatches) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let mut results: Vec<SearchResult> = Vec::new();
        for regex_match in matches {
                let basic_data: BasicSongData = BasicSongData { 
                        song_id: u32::from_str(&regex_match[2])?,
                        tab_id: u32::from_str(&regex_match[1])?,
                        title: unescape_string(&regex_match[3]).to_string(),
                        artist: unescape_string(&regex_match[4]).to_string(),
                        data_type: get_data_type(&regex_match[5]).unwrap_or(DataSetType::default()), 
                        tab_link: unescape_string(&regex_match[8]).to_string()};
                let result: SearchResult = SearchResult { 
                        basic_data: basic_data,
                        rating_count: u32::from_str(&regex_match[6])?,
                        rating_value: f32::from_str(&regex_match[7])?};
                results.push(result);
        }
        Ok(results)
}

#[cfg(test)]
mod tests {
    use std::u8;
    use crate::{search_scraper::{get_search_results, search_page}};
        #[test]
        fn search_results() {
                let valid_search_queries: Vec<&str> = vec!["zu spät die ärzte", "NEVER GONNA GIVE you up", "Don't stop me now", "Bloc party"];
                for query in valid_search_queries {
                        assert!(get_search_results(query, u8::MAX).unwrap().len() >= 1);
                }
                let no_result_queries: Vec<&str> = vec!["this should_not return any #results!"];
                for query in no_result_queries {
                        assert!(get_search_results(query, 1).unwrap().len() == 0);
                }
        }

        #[test]
        fn search_single_page() {
                let valid_search_queries: Vec<&str> = vec!["NEVER GONNA GIVE you up"];
                for query in valid_search_queries {
                        for i in 1..2 {
                                assert!(search_page(query, i).unwrap().len() >= 1);
                        }
                }
        }
}