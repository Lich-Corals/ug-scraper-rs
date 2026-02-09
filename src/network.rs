// UG-Scraper - A basic rust API for getting data from Ultimate Guitar
// Copyright (C) 2025  Linus Tibert
//
// This program was originally published under the MIT licence as seen
// here: https://github.com/Lich-Corals/ug-tab-scraper-rs/blob/mistress/LICENCE

use html_escape::decode_html_entities;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use ureq::{get, Error as ReqError};

/// Returns the raw HTML of a given URL wraped in an Result.
///
/// ## Example:
/// ```
/// use ug_scraper::network::get_raw_html;
///
/// let raw_html: String = get_raw_html("https://tabs.ultimate-guitar.com/tab/rick-astley/never-gonna-give-you-up-chords-521741").unwrap_or_default();
/// ```
///
/// ## Possible errors
/// * `ureq::Error::*`
pub fn get_raw_html(url: &str) -> Result<String, ReqError> {
        let mut response = get(url).call()?;
        let raw_html = response.body_mut().read_to_string()?;
        Ok(raw_html)
}

/// Returns a String with common escaped characters unescaped.
///
/// ## Example:
/// ```
/// use ug_scraper::network::unescape_string;
///
/// let escaped_string: &str = "This\\t is a tab.";
/// let clean_string: String = unescape_string(escaped_string);
/// // Returns:
/// // "This     is a tab"
/// ```
pub fn unescape_string(string: &str) -> String {
        decode_html_entities(string)
                .to_string()
                .replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\r", "\r")
                .replace("\\\"", "\"")
}

/// Applies basic encoding for use as an argument for a URL
///
/// ## Example:
/// ```
/// use ug_scraper::network::encode_string;
///
/// let encoded_string: String = encode_string("This is an example & \"");
/// // Returns:
/// // "This%20is%20an%20example%20&amp;%20&quot;"
/// ```
pub fn encode_string(string: &str) -> String {
        //encode_double_quoted_attribute(string).to_string().replace(" ", "%20")
        percent_encode(string.as_bytes(), NON_ALPHANUMERIC).to_string()
}
