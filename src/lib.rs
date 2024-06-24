#[macro_use]
extern crate lazy_static;

pub mod date_parser;

use thiserror::Error;
use std::result;

pub use date_parser::time::parse;
pub use date_parser::time::parse_with_custom_time_and_year;

/// Convenience type alias for parse errors
pub type Result<T, E = ParseError> = result::Result<T, E>;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("no time information in {0}")]
    MissingTimeInformationError(String),
    #[error("{0} value {1} out of range")]
    OutOfRangeError(String, String),
    #[error("Error creating date")]
    DateError(),
    #[error("internal error: {0}")]
    InternalError(String),
    #[error("offset out of bounds")]
    OffsetOutOfBounds,
}

pub use date_parser::DateTime;

pub fn get_current_timezone() -> chrono_tz::Tz {
    lazy_static! {
        static ref LOCAL_TZ: chrono_tz::Tz = {
            let tz_str = iana_time_zone::get_timezone().expect("Error getting local timezone");
            let tz: chrono_tz::Tz = tz_str.parse().expect("Error parsing local timezone");
            tz
        };
    }
    *LOCAL_TZ
}
