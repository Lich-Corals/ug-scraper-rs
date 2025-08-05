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
pub mod types_and_constants {
        use std::fmt;
        use crate::error::UGError;

        /// A list of tab types supported for downloading
        /// 
        /// Includes Chords, Tabs, Bass Tabs, Ukulele Chords and Drum Tabs
        pub const SUPPORTED_DOWNLOAD_TYPES: [DataSetType; 5] = [DataSetType::Chords, DataSetType::Tab, DataSetType::Bass, DataSetType::Ukulele, DataSetType::Drums];

        /// Known types of tab on UG. Includes unsupported ones.
        #[derive(Debug, PartialEq, Default, Eq, Clone, Copy, Hash)]
        pub enum DataSetType {
                #[default]
                Unknown,
                Chords,
                Tab,
                Ukulele,
                Bass,
                Drums,
                Official,
                Pro,
                Power,
                Video,
        }

        impl fmt::Display for DataSetType {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{:?}", self )
                }
        }

        /// Possible types of line in a `Song`
        #[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
        pub enum DataType {
                #[default]
                /// Lines with Chords detected by UG
                Chord,
                /// Plain text
                Lyric,
                /// The title of a song section
                /// 
                /// (e.g.: [chorus], [intro], etc.)
                SectionTitle,
        }

        impl fmt::Display for DataType {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{:?}", self )
                }
        }

        /// A set of data returned as sarch result
        #[derive(Debug, PartialEq, Default, Clone)]
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
        #[derive(Debug, PartialEq, Default, Clone)]
        pub struct Line {
                /// Type data stored on the line
                pub line_type: DataType,
                /// The contents on the line as plain text
                pub text_data: String,
        }

        impl fmt::Display for Line {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{:?}", self )
                }
        }

        /// A full set of available data about a tab on UG
        #[derive(Debug, PartialEq, Default, Clone)]
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
        #[derive(Debug, PartialEq, Default, Clone)]
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
        #[derive(Debug, PartialEq, Default, Clone)]
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
        /// use ug_scraper::types_and_constants::get_data_type;
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
}