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

pub mod tab_scraper;
pub mod search_scraper;
pub mod network;

pub mod types_and_constants {
        pub const SUPPORTED_DOWNLOAD_TYPES: [DataSetType; 5] = [DataSetType::Chords, DataSetType::Tab, DataSetType::Bass, DataSetType::Ukulele, DataSetType::Drums];

        #[derive(Debug, PartialEq)]
        pub enum Error {
                InvalidPageTypeError,
                UnknownTypeError,
                NoBasicDataMatchError,
                InvalidURLError,
                UnexpectedWebResultError,
                RequestError(String),
        }

        #[derive(Debug, PartialEq, Default)]
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

        #[derive(Debug)]
        pub enum DataType {
                Chord,
                Lyric,
                SectionTitle,
                SongTitle,
                CapoPosition,
                Tuning,
                TuningName,
                Tonality,
        }

        #[derive(Debug)]
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

        #[derive(Debug)]
        pub struct Line {
                pub line_type: DataType,
                pub text_data: String,
        }

        #[derive(Debug)]
        pub struct Song {
                pub lines: Vec<Line>,
                pub metadata: Option<SongMetaData>,
                pub basic_data: BasicSongData,
        }

        #[derive(Debug, PartialEq, Default)]
        pub struct BasicSongData {
                pub title: String,
                pub artist: String,
                pub tab_link: String,
                pub song_id: u32,
                pub tab_id: u32,
                pub data_type: DataSetType,
        }

        #[derive(Debug, PartialEq, Default)]
        pub struct SongMetaData {
                pub capo: Option<String>,
                pub tonality: Option<String>,
                pub tuning_name: Option<String>,
                pub tuning: Option<String>,
        }

        pub fn get_data_type(type_string: &str) -> Result<DataSetType, Error> {
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
                        _ => Err(Error::UnknownTypeError),
                }
        }
}