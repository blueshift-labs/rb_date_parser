mod parse;
mod zonetab;
pub mod time;
#[cfg(test)]
mod test_parse;
#[cfg(test)]
mod test_time;

pub use parse::date_parse;
pub use parse::date_parse2;

use serde::Serialize;
#[derive(Debug, Default, Serialize, std::cmp::PartialEq)]
pub struct DateTime {
    pub hour: Option<u32>,
    pub min: Option<u32>,
    pub sec: Option<u32>,
    pub sec_fraction: Option<f64>,
    pub year: Option<i32>,
    pub mon: Option<u32>,
    pub mday: Option<u32>,
    pub yday: Option<i32>,
    pub wday: Option<i32>,
    pub cwyear: Option<i32>,
    pub cweek: Option<u32>,
    pub cwday: Option<u32>,
    pub offset: Option<i32>,
    pub zone: Option<String>,
    pub bc: bool,
    pub comp: Option<bool>,
}

