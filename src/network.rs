// UG-Tab-Scraper - A basic rust API for getting data from Ultimate Guitar
// Copyright (C) 2025  Linus Tibert
//
// This program was originally published under the MIT licence as seen
// here: https://github.com/Lich-Corals/ug-tab-scraper-rs/blob/mistress/LICENCE

use ureq::{get, Error as ReqError};
use html_escape::{decode_html_entities, encode_double_quoted_attribute};

pub fn get_raw_html(url: &str) -> Result<String, ReqError> {
        let mut response =  get(url).call()?;
        let raw_html = response.body_mut().read_to_string()?;
        Ok(raw_html)
}

pub fn unescape_string(string: &str) -> String{
        decode_html_entities(string).to_string().replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\r", "\r")
                .replace("\\n", "\n")
}

pub fn encode_string(string: &str) -> String {
        encode_double_quoted_attribute(string).to_string().replace(" ", "%20")
}

pub fn try_to_fix_url(error: ReqError, url: &str) -> Result<String, ReqError> {
        match error {
                ReqError::BadUri(_e) => return get_raw_html(&("https://".to_owned() + url)),
                _ => return Err(ReqError::BadUri(String::from(url)))
        }
}