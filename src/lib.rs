// UG-Scraper - A basic rust API for getting data from Ultimate Guitar
// Copyright (C) 2025  Linus Tibert
//
// This program was originally published under the MIT licence as seen
// here: https://github.com/Lich-Corals/ug-tab-scraper-rs/blob/mistress/LICENCE

/// API for getting a tab from UG
pub mod tab_scraper;
/// API for searching tabs on UG
pub mod search_scraper;
/// Functions used by other modules for network access
pub mod network;

/// Errors possibly occuring in the crate
pub mod error {
        use std::error::Error;
        use std::fmt;

        /// Possible errors
        #[derive(Debug, PartialEq, Clone, Eq, Hash)]
        pub enum UGError {
                /// Occurs when an unsupported HTML is attempted to be evaluated.
                InvalidHTMLError,
                /// Occurs when an unsupported URL is attempted to be downloaded as a tab.
                InvalidURLError,
                /// Occurs when a tab without any available metadata is attempted to be downloaded.
                NoBasicDataMatchError,
                /// Occurs when any data extracting function gets unexpected data from UG.
                /// 
                /// E.g. if a string value is found in a place where a float is expected.
                UnexpectedWebResultError,
                /// Is returned by types_and_values::get_data_type() if the provided string does not match any known type of tab.
                UnknownTypeError,
        }

        impl fmt::Display for UGError {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.clone().to_string())
                }
        }

        impl Error for UGError {}

        impl UGError {
                /// Returns a brief description of the error in string format.
                pub fn to_string(self) -> String {
                        match self {
                                UGError::InvalidHTMLError => "The type of this page is not readable for this API.".to_string(),
                                UGError::InvalidURLError => "The URL does not match any known UG sites.".to_string(),
                                UGError::NoBasicDataMatchError => "Could not find any basic data for the page.".to_string(),
                                UGError::UnexpectedWebResultError => "Failed to analyze downloaded results.".to_string(),
                                UGError::UnknownTypeError => "The type supplied by UG is not known.".to_string(),
                        }
                }
        }
}

/// Types, constants and closely associated functions which are used by across the crate
/// 
/// All of the defined types have the [`serde::Serialize`] and [`serde::Deserialize`] traits.
pub mod types {
        use std::fmt;
        use crate::error::UGError;
        use serde::{Deserialize, Serialize};

        /// A list of tab types supported for downloading
        /// 
        /// Includes Chords, Tabs, Bass Tabs, Ukulele Chords and Drum Tabs
        pub const SUPPORTED_DOWNLOAD_TYPES: [DataSetType; 5] = [DataSetType::Chords, DataSetType::Tab, DataSetType::Bass, DataSetType::Ukulele, DataSetType::Drums];

        /// Known types of tab on UG. Includes unsupported ones.
        #[derive(Debug, PartialEq, Default, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
        pub enum DataSetType {
                #[default]
                Unknown,
                /// Downloading supported
                Chords,
                /// Downloading supported
                Tab,
                /// Downloading supported
                Ukulele,
                /// Downloading supported
                Bass,
                /// Downloading supported
                Drums,
                /// Downloading not supported
                Official,
                /// Downloading not supported
                Pro,
                /// Downloading not supported
                Power,
                /// Downloading not supported
                Video,
        }

        impl fmt::Display for DataSetType {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{:?}", self )
                }
        }

        /// Possible types of line in a `Song`
        #[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash, Serialize, Deserialize)]
        pub enum DataType {
                #[default]
                /// Lines with Chords detected by UG
                Chord,
                /// Plain text
                Lyric,
                /// The title of a song section
                /// 
                /// (e.g.: \[chorus\], \[intro\], etc.)
                SectionTitle,
        }

        impl fmt::Display for DataType {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{:?}", self )
                }
        }

        /// A set of data returned as sarch result
        /// 
        /// This struct is normally automatically generated by [`crate::search_scraper::get_search_results`] or [`crate::search_scraper::search_page`].
        #[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
        pub struct SearchResult {
                /// The basic metadata of the search result (tab)
                pub basic_data: BasicSongData,
                /// Amount of ratings given by users on UG
                pub rating_count: u32,
                /// Rating on UG (0.0 - 5.0)
                pub rating_value: f32,
        }

        impl fmt::Display for SearchResult {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{:?}", self )
                }
        }

        /// A single line of a tab
        /// 
        /// This struct is normally generated by [`crate::tab_scraper::get_tab_lines`].
        #[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
        pub struct Line {
                /// Type data stored on the line
                pub line_type: DataType,
                /// The contents on the line as plain text
                pub text_data: String,
        }

        impl Line {
                /// Replace german chord names with english ones
                /// 
                /// Musical notation is one of the things Germans did their own, slightly more complicated way.
                /// This function will replace german names for chords with their english equivalents.
                /// 
                /// ## Example:
                /// ```
                /// use ug_scraper::types::{Line, DataType};
                /// 
                /// // Create a line with German chord names H, Dmoll, Fis, Es and B
                /// let mut line: Line = Line { line_type: DataType::Chord,
                ///         text_data: "A    H      C Dmoll    Fis Es B".to_string() };
                /// 
                /// // Replace the weïrd chord names
                /// line = line.replace_german_names();
                /// // Returns:
                /// // "A    B      C Dm       F#  Eb Bb"
                /// 
                /// ```
                /// 
                /// Note:
                /// Some of the German chord notations (e.g. Fes or Dmoll) are rarely found in any tabs. 
                /// But to ensure they don't confuse anyone, they are included in this function too.
                pub fn replace_german_names(mut self) -> Line {
                        if self.line_type == DataType::Chord {
                                let entries: [&'static str; 18]      = ["Ces","Cis","Des","Dis","Es","Eis","Fes","Fis","Ges","Gis","As","Ais","H","Bes","His","Bis","dur","moll"];
                                let replacements: [&'static str; 18] = ["Cb ","C# ","Db ","D# ","Eb","E# ","Fb ","F# ","Gb ","G# ","Ab","A# ","B","Bb ","B# ","B# ","maj","m   "];
                                if entries.iter().any(|entry: &&str| self.text_data.contains(entry)) {
                                        self.text_data = self.text_data.replace("B", "Bb")
                                                                       .replace("Bb  ", "Bb ");
                                                                       
                                        for i in 0..entries.len() {
                                                self.text_data = self.text_data.replace(entries[i], replacements[i]);
                                        }
                                }
                        }
                        self
                }
        }

        impl fmt::Display for Line {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{:?}", self )
                }
        }

        /// A full set of available data about a tab on UG
        /// 
        /// This struct is normally generated by [`crate::tab_scraper::get_song_data`].
        #[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
        pub struct Song {
                /// A vector of all lines in the tab
                pub lines: Vec<Line>,
                /// The detailed metadata of the song.
                /// 
                /// This data is optional, because some types of tab (e.g. Drum) don't have any metadata.
                pub metadata: Option<SongMetaData>,
                /// Basic data about the tab
                pub basic_data: BasicSongData,
        }

        impl fmt::Display for Song {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{:?}", self )
                }
        }

        /// Basic metadata every tab has
        /// 
        /// This struct is normally generated by [`crate::tab_scraper::get_basic_metadata`].
        #[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
        pub struct BasicSongData {
                /// Title of the song
                pub title: String,
                /// Name of the artist
                pub artist: String,
                /// Link to the tab
                pub tab_link: String,
                /// UG ID of the song
                /// 
                /// Don't confuse this with the tab ID, which is only for a single tab!
                pub song_id: u32,
                /// UG ID of the tab
                /// 
                /// Don't confuse this with the song ID, which is for every tab of the song!
                pub tab_id: u32,
                /// The type of tab
                pub data_type: DataSetType,
        }

        impl fmt::Display for BasicSongData {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{:?}", self )
                }
        }

        /// Special metadata which is not available for every tab (type)
        /// 
        /// Tabs of the type `Drums` never have this. Bass tabs often don't have.
        /// Many tabs are missing values of the metadata; thus, they are all options.
        /// 
        /// This struct is normally automatically generated when using [`crate::tab_scraper::get_song_data`].
        #[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
        pub struct SongMetaData {
                pub capo: Option<String>,
                pub tonality: Option<String>,
                pub tuning_name: Option<String>,
                pub tuning: Option<String>,
        }

        impl fmt::Display for SongMetaData {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{:?}", self )
                }
        }

        /// Get the data type associated with a string scraped from UG
        /// 
        /// ## Example:
        /// ```
        /// use ug_scraper::types::get_data_type;
        /// 
        /// get_data_type("Chords");
        /// // Returns:
        /// // enum variant DataSetType::Chords
        /// ```
        /// 
        /// ## Supported strings:
        /// * Chords
        /// * Tabs
        /// * Bass Tabs
        /// * Ukulele Chords
        /// * Drum Tabs
        /// * Official
        /// * Pro
        /// * Power
        /// * Video
        ///
        /// Returns `UGError::UnknownTypeError` if type is unknown.
        pub fn get_data_type(type_string: &str) -> Result<DataSetType, UGError> {
                match type_string {
                        "Chords" => Ok(DataSetType::Chords),
                        "Tabs" => Ok(DataSetType::Tab),
                        "Bass Tabs" => Ok(DataSetType::Bass),
                        "Ukulele Chords" => Ok(DataSetType::Ukulele),
                        "Drum Tabs" => Ok(DataSetType::Drums),
                        "Official" => Ok(DataSetType::Official),
                        "Pro" => Ok(DataSetType::Pro),
                        "Power" => Ok(DataSetType::Power),
                        "Video" => Ok(DataSetType::Video),
                        _ => Err(UGError::UnknownTypeError),
                }
        }

        #[cfg(test)]
        mod tests {
            use crate::types::Line;

                #[test]
                fn german_names_replacement() {
                        let german_line: Line = Line { line_type: super::DataType::Chord,
                                text_data: "A    H      C Dmoll    Fis Es B  B".to_string() };
                        let english_line: Line = Line { line_type: super::DataType::Chord,
                                text_data: "A    B      C Dm       F#  Eb Bb Bb".to_string() };
                                assert_eq!(german_line.replace_german_names().text_data, english_line.text_data);
                }
        }
}