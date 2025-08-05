// UG-Tab-Scraper - A basic rust API for getting data from Ultimate Guitar
// Copyright (C) 2025  Linus Tibert
//
// This program was originally published under the MIT licence as seen
// here: https://github.com/Lich-Corals/ug-tab-scraper-rs/blob/mistress/LICENCE

pub mod tab_scraper;
pub mod search_scraper;
pub mod network;

pub mod types_and_constants {
        use std::error::Error;
        use std::fmt;

        pub const SUPPORTED_DOWNLOAD_TYPES: [DataSetType; 5] = [DataSetType::Chords, DataSetType::Tab, DataSetType::Bass, DataSetType::Ukulele, DataSetType::Drums];

        #[derive(Debug, PartialEq, Clone, Eq, Hash)]
        pub enum UGError {
                InvalidPageTypeError,
                NoBasicDataMatchError,
                InvalidURLError,
                UnexpectedWebResultError,
                UnknownTypeError,
        }

        impl fmt::Display for UGError {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.clone().to_string())
                }
        }

        impl Error for UGError {}

        impl UGError {
                pub fn to_string(self) -> String {
                        match self {
                                UGError::InvalidPageTypeError => "The type of this page is not readable for this API.".to_string(),
                                UGError::InvalidURLError => "The URL does not match any known UG sites.".to_string(),
                                UGError::NoBasicDataMatchError => "Could not find any basic data for the page.".to_string(),
                                UGError::UnexpectedWebResultError => "Failed to analyze downloaded results.".to_string(),
                                UGError::UnknownTypeError => "The type supplied by UG is not known.".to_string(),
                        }
                }
        }

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

        #[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
        pub enum DataType {
                #[default]
                Chord,
                Lyric,
                SectionTitle,
                SongTitle,
                CapoPosition,
                Tuning,
                TuningName,
                Tonality,
        }

        impl fmt::Display for DataType {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{:?}", self )
                }
        }


        #[derive(Debug, PartialEq, Default, Clone)]
        pub struct SearchResult {
                pub song_id: u32,
                pub tab_id: u32,
                pub title: String,
                pub artist: String,
                pub data_type: DataSetType,
                pub rating_count: u32,
                pub rating_value: f32,
                pub url: String,
        }

        impl fmt::Display for SearchResult {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{:?}", self )
                }
        }

        #[derive(Debug, PartialEq, Default, Clone)]
        pub struct Line {
                pub line_type: DataType,
                pub text_data: String,
        }

        impl fmt::Display for Line {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{:?}", self )
                }
        }

        #[derive(Debug, PartialEq, Default, Clone)]
        pub struct Song {
                pub lines: Vec<Line>,
                pub metadata: Option<SongMetaData>,
                pub basic_data: BasicSongData,
        }

        impl fmt::Display for Song {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{:?}", self )
                }
        }

        #[derive(Debug, PartialEq, Default, Clone)]
        pub struct BasicSongData {
                pub title: String,
                pub artist: String,
                pub tab_link: String,
                pub song_id: u32,
                pub tab_id: u32,
                pub data_type: DataSetType,
        }

        impl fmt::Display for BasicSongData {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{:?}", self )
                }
        }

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