use phf::phf_map;
use regex::{Regex, Captures, RegexBuilder};
use serde::{Serialize};

#[derive(Debug, Default, Serialize, std::cmp::PartialEq)]
pub struct DateTime {
    pub hour: Option<u64>,
    pub min: Option<u64>,
    pub sec: Option<u64>,
    pub sec_fraction: Option<f64>,
    pub year: Option<i64>,
    pub mon: Option<u64>,
    pub mday: Option<u64>,
    pub yday: Option<i64>,
    pub wday: Option<i64>,
    pub cwyear: Option<i64>,
    pub cweek: Option<u64>,
    pub cwday: Option<u64>,
    pub offset: Option<i64>,
    pub zone: Option<String>,
    pub bc: bool,
    pub comp: Option<bool>,
}

const SPACE: &str = " ";
const HAVE_ALPHA: u64 = 1<<0;
const HAVE_DIGIT: u64 = 1<<1;
const HAVE_DASH: u64 = 1<<2;
const HAVE_DOT: u64 = 1<<3;
const HAVE_SLASH: u64 = 1<<4;

fn check_classes(string: &str, classes: u64) -> bool {
    let mut class: u64 = 0;
    for i in string.chars() {
        if i.is_alphabetic() {
            class |= HAVE_ALPHA;
        } else if i.is_numeric() {
            class |= HAVE_DIGIT;
        } else if i == '-' {
            class |= HAVE_DASH;
        } else if i == '.' {
            class |= HAVE_DOT;
        } else if i == '/' {
            class |= HAVE_SLASH;
        }
        if (class & classes) == classes {
            return true
        }
    }
    false
}

static DAYS_HASH: phf::Map<&'static str, i64> = phf_map! {
    "sun" => 0,
    "mon" => 1,
    "tue" => 2,
    "wed" => 3,
    "thu" => 4,
    "fri" => 5,
    "sat" => 6,
    };

static MONTHS_HASH: phf::Map<&'static str, &'static str> = phf_map! {
    "jan" => "01",
    "feb" => "02",
    "mar" => "03",
    "apr" => "04",
    "may" => "05",
    "jun" => "06",
    "jul" => "07",
    "aug" => "08",
    "sep" => "09",
    "oct" => "10",
    "nov" => "11",
    "dec" => "12",
    };

fn gengo(c: char) -> u64 {
    match c {
        'M' => 1867,
        'm' => 1867,
        'T' => 1911,
        't' => 1911,
        'S' => 1925,
        's' => 1925,
        'H' => 1988,
        'h' => 1988,
        'R' => 2018,
        'r' => 2018,
        _ => 0,
    }
}


fn day_num(date: &str) -> i64 {
    *DAYS_HASH.get(&date.to_lowercase()).unwrap_or(&0)
}

fn months_num(date: &str) -> &'static str {
    *MONTHS_HASH.get(&date.to_lowercase()).unwrap_or(&"01")
}

fn slice_number_part(string: &str) -> (&str, &str, &str) {
    if let Some(index) = string.find(|c:char| !c.is_ascii_digit()) {
        (&string[0..index], &string[index..=index], &string[(index+1)..])
    } else {
        (string, "", "")
    }
}

fn slice_signed_number_part(string: &str) -> Option<(&str, &str, &str)> {
    if let Some(index) = string.find(|c:char| c.is_ascii_digit() || c == '+' || c == '-' ) {
        let first =  &string[index..=index];
        let (sign, rest) = if first == "+" || first == "-" {
            (first, &string[(index+1)..])
        } else {
            ("", &string[index..])
        };

        let (number_slice, remaining) = if let Some(index) = rest.find(|c:char| !c.is_ascii_digit()) {
            (&rest[0..index], &rest[(index+1)..])
        } else {
            (rest, "")
        };

        Some((sign, number_slice, remaining))
    } else {
        None
    }
}


fn validate_and_set_date(datetime: &mut DateTime, yr: &Option<&str>, m: &Option<&str>, d: &Option<&str>, is_bc: bool) {
    lazy_static! {
        static ref YEAR_DIGITS_REGEX: Regex = Regex::new(r"[^-+\d]*([+-]?)(\d+)([^\d]*)").unwrap();
        static ref DIGITS_REGEX: Regex = Regex::new(r"\D*(\d+)\D*").unwrap();
    }

    let mut comp: Option<bool>  = None;

    let mut year =  yr.and_then(|x| if x.is_empty() {None} else {Some(x.to_owned())});
    let mut month =  m.and_then(|x| if x.is_empty() {None} else {Some(x.to_owned())});
    let mut day =  d.and_then(|x| if x.is_empty() {None} else {Some(x.to_owned())});

    // If day is empty, then
    if year.is_some() && month.is_some() && day.is_none() {
        let oy  = year;
        let om  = month;
        let od  = day;

        year = od;
        month = oy;
        day = om;
    }

    if year.is_none() {
        if let Some(dval) = &day {
            if dval.len() > 2 {
                year = day;
                day = None;
            }
        }
        if let Some(dval) = &day {
            if !dval.is_empty() && dval.starts_with('\'') {
                year = day;
                day = None;
            }
        }
    }

    if let Some(yr_str) = year {
        if let Some((sign, digits, rest)) = slice_signed_number_part(&yr_str) {
            if !rest.is_empty() {
                year = day;
                day = Some(format!("{}{}", sign, digits));
            } else {
                year = Some(yr_str);
            }
        } else {
            year = Some(yr_str);
        }
    }

    if let Some(mth_str) = month {
        if mth_str.starts_with('\'') || mth_str.len() > 2 {
            let tmp = year;
            year = Some(mth_str);
            month = day;
            day =  tmp;
        } else {
            month = Some(mth_str);
        }
    }

    if let Some(day_str) = day {
        //get rid of leading 0s if any from date
        //let day_str = day_str.trim_start_matches('0').to_owned();
        if day_str.starts_with('\'') || day_str.len() > 2 {
            let tmp = year;
            year = Some(day_str);
            day =  tmp;
        } else {
            day = Some(day_str);
        }
    }

    if let Some(yr_str) = year {
        if let Some((sign, digits, _rest)) = slice_signed_number_part(&yr_str) {
            if !sign.is_empty() || digits.len() > 2 {
                comp = Some(false);
            }
            datetime.year = Some(format!("{}{}", sign, digits).parse::<i64>().unwrap());
        }
    }

    if let Some(month_str) = month {
        if let Some(cap) = DIGITS_REGEX.captures(&month_str) {
            if let Some(mtch) = cap.get(1) {
                let  digits = mtch.as_str();
                datetime.mon = Some(digits.parse::<u64>().unwrap());
            }
        }
    }

    if let Some(day_str) = day {
        if let Some(cap) = DIGITS_REGEX.captures(&day_str) {
            if let Some(mtch) = cap.get(1) {
                let  digits = mtch.as_str();
                datetime.mday = Some(digits.parse::<u64>().unwrap());
            }
        }
    }

    datetime.bc = is_bc;
    if comp.is_some() {
        datetime.comp = comp;
    }

}

fn parse_eu(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref EU_REGEX: Regex = RegexBuilder::new(r"
            ('?\d+)[^-\d\s]*
            \s*
            (jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)[^-\d\s']*
            (?:
                \s*
                (c(?:e|\.e\.)|b(?:ce|\.c\.e\.)|a(?:d|\.d\.)|b(?:c|\.c\.))?
                \s*
                ('?-?\d+(?:(?:st|nd|rd|th)\b)?)
            )?
        ")
        .case_insensitive(true)
        .ignore_whitespace(true)
        .build()
        .unwrap();
    }
    let mut matched = false;
    let result = EU_REGEX.replace(string, |caps: &Captures| {

        matched =  true;
        let day = caps.get(1).map(|x| x.as_str());
        let mon = caps.get(2).map(|x| months_num(x.as_str()));

        let is_bc = match caps.get(3) {
            None => false,
            Some(cap) => {
                if let Some(char) = cap.as_str().chars().next() {
                    char == 'b' || char == 'B'
                } else {
                    false
                }
            }
        };
        let year = caps.get(4).map(|x| x.as_str());

        validate_and_set_date(datetime, &year, &mon, &day, is_bc);


        SPACE
    });
    *string = result.to_string();
    matched
}


fn parse_us(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref US_REGEX: Regex = RegexBuilder::new(r"
            \b(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)[^-\d\s']*
                \s*
                ('?\d+)[^-\d\s']*
                (?:
                    \s*,?
                    \s*
                    (c(?:e|\.e\.)|b(?:ce|\.c\.e\.)|a(?:d|\.d\.)|b(?:c|\.c\.))?
                    \s*
                    ('?-?\d+)
            )?

        ")
        .case_insensitive(true)
        .ignore_whitespace(true)
        .build()
        .unwrap();
    }
    let mut matched = false;
    let result = US_REGEX.replace(string, |caps: &Captures| {
        matched =  true;
        let mon = caps.get(1).map(|x| months_num(x.as_str()));
        let day = caps.get(2).map(|x| x.as_str());

        let is_bc = match caps.get(3) {
            None => false,
            Some(cap) => {
                if let Some(char) = cap.as_str().chars().next() {
                    char == 'b' || char == 'B'
                } else {
                    false
                }
            }
        };
        let year = caps.get(4).map(|x| x.as_str());

        validate_and_set_date(datetime, &year, &mon, &day, is_bc);


        SPACE
    });
    *string = result.to_string();
    matched
}


fn parse_iso(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref ISO_REGEX: Regex = RegexBuilder::new(r"('?[-+]?\d+)-(\d+)-('?-?\d+)")
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = ISO_REGEX.replace(string, |caps: &Captures| {
        matched =  true;
        let year = caps.get(1).map(|x| x.as_str());
        let mon = caps.get(2).map(|x| x.as_str());
        let day = caps.get(3).map(|x| x.as_str());
        validate_and_set_date(datetime, &year, &mon, &day, false);


        SPACE
    });
    *string = result.to_string();
    matched
}


fn parse_iso_21(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref ISO21_REGEX: Regex = RegexBuilder::new(r"\b(\d{2}|\d{4})?-?w(\d{2})(?:-?(\d))?\b")
                        .case_insensitive(true)
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = ISO21_REGEX.replace(string, |caps: &Captures| {
        matched =  true;
        let cwyear = caps.get(1).map(|x| x.as_str()).map(|x| x.parse::<i64>().unwrap());
        let cweek = caps.get(2).map(|x| x.as_str()).map(|x| x.parse::<i64>().unwrap());
        let cwday = caps.get(3).map(|x| x.as_str()).map(|x| x.parse::<i64>().unwrap());
        if let Some(n) = cwyear {
            datetime.cwyear = Some(n);
        }
        if let Some(n) = cweek {
            datetime.cweek = Some(n as u64);
        }
        if let Some(n) = cwday {
            datetime.cwday = Some(n as u64);
        }


        SPACE
    });
    *string = result.to_string();
    matched
}


fn parse_iso_22(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref ISO22_REGEX: Regex = RegexBuilder::new(r"-w-(\d)\b")
                        .case_insensitive(true)
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = ISO22_REGEX.replace(string, |caps: &Captures| {
        matched =  true;
        let cwday = caps.get(1).map(|x| x.as_str()).map(|x| x.parse::<i64>().unwrap());
        if let Some(n) = cwday {
            datetime.cwday = Some(n as u64);
        }

        SPACE
    });
    *string = result.to_string();
    matched
}


fn parse_iso_23(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref ISO23_REGEX: Regex = RegexBuilder::new(r"--(\d{2})?-(\d{2})\b")
                        .case_insensitive(false)
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = ISO23_REGEX.replace(string, |caps: &Captures| {
        matched =  true;
        let mon = caps.get(1).map(|x| x.as_str()).map(|x| x.parse::<i64>().unwrap());
        let day = caps.get(2).map(|x| x.as_str()).map(|x| x.parse::<i64>().unwrap());
        if let Some(n) = mon {
            datetime.mon = Some(n as u64);
        }
        if let Some(n) = day {
            datetime.mday = Some(n as u64);
        }

        SPACE
    });
    *string = result.to_string();
    matched
}


fn parse_iso_24(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref ISO24_REGEX: Regex = RegexBuilder::new(r"--(\d{2})(\d{2})?\b")
                        .case_insensitive(false)
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = ISO24_REGEX.replace(string, |caps: &Captures| {
        matched =  true;
        let mon = caps.get(1).map(|x| x.as_str()).map(|x| x.parse::<i64>().unwrap());
        let day = caps.get(2).map(|x| x.as_str()).map(|x| x.parse::<i64>().unwrap());
        if let Some(n) = mon {
            datetime.mon = Some(n as u64);
        }
        if let Some(n) = day {
            datetime.mday = Some(n as u64);
        }

        SPACE
    });
    *string = result.to_string();
    matched
}


fn parse_iso_25(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref ISO25_REGEX1: Regex = RegexBuilder::new(r"[,.](\d{2}|\d{4})-\d{3}\b")
                        .case_insensitive(false)
                        .build()
                        .unwrap();
        static ref ISO25_REGEX2: Regex = RegexBuilder::new(r"\b(\d{2}|\d{4})-(\d{3})\b")
                        .case_insensitive(false)
                        .build()
                        .unwrap();
    }
    let mut matched = false;

    if ISO25_REGEX1.is_match(string) {
        return false;
    }

    let result = ISO25_REGEX2.replace(string, |caps: &Captures| {
        matched =  true;
        let y = caps.get(1).map(|x| x.as_str()).map(|x| x.parse::<i64>().unwrap());
        let d = caps.get(2).map(|x| x.as_str()).map(|x| x.parse::<i64>().unwrap());
        if let Some(n) = y {
            datetime.year = Some(n);
        }
        if let Some(n) = d {
            datetime.yday = Some(n);
        }

        SPACE
    });
    *string = result.to_string();
    matched
}


fn parse_iso_26(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref ISO26_REGEX1: Regex = RegexBuilder::new(r"\d-\d{3}\b")
                        .case_insensitive(false)
                        .build()
                        .unwrap();
        static ref ISO26_REGEX2: Regex = RegexBuilder::new(r"\b-(\d{3})\b")
                        .case_insensitive(false)
                        .build()
                        .unwrap();
    }
    let mut matched = false;

    if ISO26_REGEX1.is_match(string) {
        return false;
    }

    let result = ISO26_REGEX2.replace(string, |caps: &Captures| {
        matched =  true;
        let d = caps.get(1).map(|x| x.as_str()).map(|x| x.parse::<i64>().unwrap());
        if let Some(n) = d {
            datetime.yday = Some(n);
        }

        SPACE
    });
    *string = result.to_string();
    matched
}


fn parse_iso2(string: &mut String, datetime: &mut DateTime) -> bool {
    if parse_iso_21(string, datetime) {
        return  true;
    }
    if parse_iso_22(string, datetime) {
        return  true;
    }
    if parse_iso_23(string, datetime) {
        return  true;
    }
    if parse_iso_24(string, datetime) {
        return  true;
    }
    if parse_iso_25(string, datetime) {
        return  true;
    }
    if parse_iso_26(string, datetime) {
        return  true;
    }
    false
}




fn parse_jis(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref JIS_REGEX: Regex = RegexBuilder::new(r"\b([mtshr])(\d+)\.(\d+)\.(\d+)")
                        .case_insensitive(true)
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = JIS_REGEX.replace(string, |caps: &Captures| {
        matched =  true;
        let ep = caps.get(1)
                                .map(|x| x.as_str())
                                .and_then(|x| x.chars().next())
                                    .map(|x| gengo(x) as i64)
                                .unwrap_or_default();

        if let Some(x) = caps.get(2)
                .map(|x| x.as_str())
                .map(|x| x.parse::<i64>().unwrap())
                .map(|x| x + ep) { datetime.year = Some(x); }

        if let Some(x) = caps.get(3)
                .map(|x| x.as_str())
                .map(|x| x.parse::<i64>().unwrap()) { datetime.mon = Some(x as u64); }


        if let Some(x) = caps.get(4)
                .map(|x| x.as_str())
                .map(|x| x.parse::<i64>().unwrap()) { datetime.mday = Some(x as u64); }


        SPACE
    });
    *string = result.to_string();
    matched
}



fn parse_vms11(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref VMS11_REGEX: Regex = RegexBuilder::new(r"
                        ('?-?\d+)-(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)[^-/.]*
                        -('?-?\d+)
                        ")
                        .ignore_whitespace(true)
                        .case_insensitive(true)
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = VMS11_REGEX.replace(string, |caps: &Captures| {
        matched =  true;

        let day = caps.get(1).map(|x| x.as_str());
        let mon = caps.get(2).map(|x| months_num(x.as_str()));
        let year = caps.get(3).map(|x| x.as_str());

        validate_and_set_date(datetime, &year, &mon, &day, false);


        SPACE
    });
    *string = result.to_string();
    matched
}


fn parse_vms12(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref VMS11_REGEX: Regex = RegexBuilder::new(r"
                        \b(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)[^-/.]*
                        -('?-?\d+)(?:-('?-?\d+))?
                        ")
                        .ignore_whitespace(true)
                        .case_insensitive(true)
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = VMS11_REGEX.replace(string, |caps: &Captures| {
        matched =  true;
        let mon = caps.get(1).map(|x| months_num(x.as_str()));
        let day = caps.get(2).map(|x| x.as_str());
        let year = caps.get(3).map(|x| x.as_str());

        validate_and_set_date(datetime, &year, &mon, &day, false);


        SPACE
    });
    *string = result.to_string();
    matched
}

fn parse_vms(string: &mut String, datetime: &mut DateTime) -> bool {
    if parse_vms11(string, datetime) {
        return  true;
    }
    if parse_vms12(string, datetime) {
        return  true;
    }
    false
}


fn parse_sla(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref SLA_REGEX: Regex = RegexBuilder::new(r"
                            ('?-?\d+)/\s*('?\d+)(?:\D\s*('?-?\d+))?
                        ")
                        .ignore_whitespace(true)
                        .case_insensitive(true)
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = SLA_REGEX.replace(string, |caps: &Captures| {
        matched =  true;
        let year = caps.get(1).map(|x| x.as_str());
        let mon = caps.get(2).map(|x| x.as_str());
        let day = caps.get(3).map(|x| x.as_str());
        validate_and_set_date(datetime, &year, &mon, &day, false);


        SPACE
    });
    *string = result.to_string();
    matched
}


fn parse_dot(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref DOT_REGEX: Regex = RegexBuilder::new(r"
                            ('?-?\d+)\.\s*('?\d+)\.\s*('?-?\d+)
                        ")
                        .ignore_whitespace(true)
                        .case_insensitive(true)
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = DOT_REGEX.replace(string, |caps: &Captures| {
        matched =  true;
        let year = caps.get(1).map(|x| x.as_str());
        let mon = caps.get(2).map(|x| x.as_str());
        let day = caps.get(3).map(|x| x.as_str());
        validate_and_set_date(datetime, &year, &mon, &day, false);


        SPACE
    });
    *string = result.to_string();
    matched
}


fn digits_str_to_int(string: &str, start: usize, size: usize) -> Option<i64> {
    let len = string.len();
    let end = std::cmp::min(start+size,len);
    (string[start..end]).parse::<i64>().ok()
}

fn parse_ddd(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref DDD_REGEX: Regex = RegexBuilder::new(r"
                        ([-+]?)(\d{2,14})
                            (?:
                                \s*
                                t?
                                \s*
                                (\d{2,6})?(?:[,.](\d*))?
                            )?
                            (?:
                                \s*
                                (
                                    z\b
                                |
                                    [-+]\d{1,4}\b
                                |
                                    \[[-+]?\d[^\]]*\]
                                )
                            )?
                        ")
                        .case_insensitive(true)
                        .ignore_whitespace(true)
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = DDD_REGEX.replace(string, |caps: &Captures| {
        matched =  true;
        let is_negative_sign  =caps.get(1)
                .map(|x| x.as_str())
                .map(|x| x == "-")
                .unwrap_or_default();
        let sign = if is_negative_sign { -1 } else { 1 };
        let s2  =caps.get(2)
                .map(|x| x.as_str()).unwrap();
        let s3  =caps.get(3)
                .map(|x| x.as_str());
        let s4  =caps.get(4)
                .map(|x| x.as_str());
        let s5  =caps.get(5)
                .map(|x| x.as_str());

        let l2 = s2.len();

        match l2 {
            2 => {
                if s3.is_none() && s4.is_some() {
                    if let Some(num) = digits_str_to_int(s2, l2-2, 2) {
                        datetime.sec = Some(num as u64);
                    }
                } else if let Some(num) = digits_str_to_int(s2, 0, 2) {
                    datetime.mday = Some(num as u64);
                }
            },
            4 => {
                if s3.is_none() && s4.is_some() {
                    if let Some(num) = digits_str_to_int(s2, l2-2, 2) {
                        datetime.sec = Some(num as u64);
                    }
                    if let Some(num) = digits_str_to_int(s2, l2-4, 2) {
                        datetime.min = Some(num as u64);
                    }
                } else {
                    if let Some(num) = digits_str_to_int(s2, 0, 2) {
                        datetime.mon = Some(num as u64);
                    }
                    if let Some(num) = digits_str_to_int(s2, 2, 2) {
                        datetime.mday = Some(num as u64);
                    }
                }
            },
            6 => {
                if s3.is_none() && s4.is_some() {
                    if let Some(num) = digits_str_to_int(s2, l2-2, 2) {
                        datetime.sec = Some(num as u64);
                    }
                    if let Some(num) = digits_str_to_int(s2, l2-4, 2) {
                        datetime.min = Some(num as u64);
                    }
                    if let Some(num) = digits_str_to_int(s2, l2-6, 2) {
                        datetime.hour = Some(num as u64);
                    }
                } else {
                    if let Some(num) = digits_str_to_int(s2, 0, 2) {
                        datetime.year = Some(sign * num);
                    }
                    if let Some(num) = digits_str_to_int(s2, 2, 2) {
                        datetime.mon = Some(num as u64);
                    }
                    if let Some(num) = digits_str_to_int(s2, 4, 2) {
                        datetime.mday = Some(num as u64);
                    }
                }
            },
            8 | 10 | 12 | 14 => {
                if s3.is_none() && s4.is_some() {
                    if let Some(num) = digits_str_to_int(s2, l2-2, 2) {
                        datetime.sec = Some(num as u64);
                    }
                    if let Some(num) = digits_str_to_int(s2, l2-4, 2) {
                        datetime.min = Some(num as u64);
                    }
                    if let Some(num) = digits_str_to_int(s2, l2-6, 2) {
                        datetime.hour = Some(num as u64);
                    }
                    if let Some(num) = digits_str_to_int(s2, l2-8, 2) {
                        datetime.mday = Some(num as u64);
                    }
                    if l2 >= 10 {
                        if let Some(num) = digits_str_to_int(s2, l2-10, 2) {
                            datetime.mon = Some(num as u64);
                        }
                    }
                    if l2 == 12 {
                        if let Some(num) = digits_str_to_int(s2, l2-12, 2) {
                            datetime.year = Some(sign * num);
                        }
                    }
                    if l2 == 14 {
                        if let Some(num) = digits_str_to_int(s2, l2-14, 4) {
                            datetime.year = Some(sign * num);
                        }
                        datetime.comp = Some(false);
                    }
                } else {
                    if let Some(num) = digits_str_to_int(s2, 0, 4) {
                        datetime.year = Some(sign * num);
                    }
                    if let Some(num) = digits_str_to_int(s2, 4, 2) {
                        datetime.mon = Some(num as u64);
                    }
                    if let Some(num) = digits_str_to_int(s2, 6, 2) {
                        datetime.mday = Some(num as u64);
                    }
                    if l2 >= 10 {
                        if let Some(num) = digits_str_to_int(s2, 8, 2) {
                            datetime.hour = Some(num as u64);
                        }
                    }
                    if l2 >= 12 {
                        if let Some(num) = digits_str_to_int(s2, 10, 2) {
                            datetime.min = Some(num as u64);
                        }
                    }
                    if l2 == 14 {
                        if let Some(num) = digits_str_to_int(s2, 12, 2) {
                            datetime.sec = Some(num as u64);
                        }
                        datetime.comp = Some(false);
                    }
                }
            },
            3 => {
                if s3.is_none() && s4.is_some() {
                    if let Some(num) = digits_str_to_int(s2, l2-2, 2) {
                        datetime.sec = Some(num as u64);
                    }
                    if let Some(num) = digits_str_to_int(s2, l2-3, 1) {
                        datetime.min = Some(num as u64);
                    }
                } else if let Some(num) = digits_str_to_int(s2, 0, 3) {
                    datetime.yday = Some(num);
                }
            },
            5 => {
                if s3.is_none() && s4.is_some() {
                    if let Some(num) = digits_str_to_int(s2, l2-2, 2) {
                        datetime.sec = Some(num as u64);
                    }
                    if let Some(num) = digits_str_to_int(s2, l2-4, 2) {
                        datetime.min = Some(num as u64);
                    }
                    if let Some(num) = digits_str_to_int(s2, l2-5, 1) {
                        datetime.hour = Some(num as u64);
                    }
                } else{
                    if let Some(num) = digits_str_to_int(s2, 0, 2) {
                        datetime.year = Some(sign * num);
                    }
                    if let Some(num) = digits_str_to_int(s2, 2, 3) {
                        datetime.yday = Some(num);
                    }
                }
            },
            7 => {
                if s3.is_none() && s4.is_some() {
                    if let Some(num) = digits_str_to_int(s2, l2-2, 2) {
                        datetime.sec = Some(num as u64);
                    }
                    if let Some(num) = digits_str_to_int(s2, l2-4, 2) {
                        datetime.min = Some(num as u64);
                    }
                    if let Some(num) = digits_str_to_int(s2, l2-6, 2) {
                        datetime.hour = Some(num as u64);
                    }
                    if let Some(num) = digits_str_to_int(s2, l2-7, 1) {
                        datetime.mday = Some(num as u64);
                    }
                } else{
                    if let Some(num) = digits_str_to_int(s2, 0, 4) {
                        datetime.year = Some(sign * num);
                    }
                    if let Some(num) = digits_str_to_int(s2, 4, 3) {
                        datetime.yday = Some(num);
                    }
                }
            },
            _ => {},
        }

        if let Some(s3) = s3 {
            let l3 = s3.len();
            if s4.is_some() {
                match l3 {
                    2 | 4 | 6 => {
                        if let Some(num) = digits_str_to_int(s3, l3-2, 2) {
                            datetime.sec = Some(num as u64);
                        }
                        if l3 >= 4 {
                            if let Some(num) = digits_str_to_int(s3, l3-4, 2) {
                                datetime.min = Some(num as u64);
                            }
                        }
                        if l3 >= 6 {
                            if let Some(num) = digits_str_to_int(s3, l3-6, 2) {
                                datetime.hour = Some(num as u64);
                            }
                        }
                    },
                    _ => {},
                }

            } else {
                match l3 {
                    2 | 4 | 6 => {
                        if let Some(num) = digits_str_to_int(s3, 0, 2) {
                            datetime.hour = Some(num as u64);
                        }
                        if l3 >= 4 {
                            if let Some(num) = digits_str_to_int(s3, 2, 2) {
                                datetime.min = Some(num as u64);
                            }
                        }
                        if l3 >= 6 {
                            if let Some(num) = digits_str_to_int(s3, 4, 2) {
                                datetime.sec = Some(num as u64);
                            }
                        }
                    },
                    _ => {},
                }
            }
        }
        if let Some(s4) = s4 {
            let frac_str = format!("0.{}", s4);
            if let Ok(fraction) = frac_str.parse::<f64>() {
                datetime.sec_fraction = Some(fraction);
            }
        }

        if let Some(s5) = s5 {
            datetime.zone = Some(s5.to_owned());
            if  s5.starts_with('[') {
                let trimmed = s5.trim_start_matches('[').trim_end_matches(']');
                if let Some((s5, zone)) = trimmed.split_once(':') {
                    datetime.zone = Some(zone.to_owned());
                    datetime.offset = date_zone_to_diff(s5);
                } else {
                    datetime.zone = Some(trimmed.to_owned());
                    if let Some(first_char) = trimmed.chars().next() {
                        if ('0'..='9').contains(&first_char) {
                            datetime.offset =date_zone_to_diff(&format!("+{}", trimmed));
                        } else {
                            datetime.offset = date_zone_to_diff(trimmed);
                        }
                    }

                }
            }
        }

        SPACE
    });
    *string = result.to_string();
    matched
}



fn date_zone_to_diff(zone: &str) -> Option<i64> {
    lazy_static! {
        static ref STD_TIME_SUFFIX: Regex = RegexBuilder::new(r"(.*)\s*(?i:standard time)")
                                                .case_insensitive(true)
                                                .build().unwrap();
        static ref DAYLIGHT_TIME_SUFFIX: Regex = RegexBuilder::new(r"(.*)\s*(?i:daylight time)")
                                                .case_insensitive(true)
                                                .build().unwrap();
        static ref DST_SUFFIX: Regex = RegexBuilder::new(r"(.*)\s*(?i:dst)")
                                                .case_insensitive(true)
                                                .build().unwrap();
        static ref GMT_UTC_PREFIX: Regex = RegexBuilder::new(r"(?i:gmt|utc)?(\S*)")
                                                .case_insensitive(true)
                                                .build().unwrap();
    }
    let mut zone_str =  zone;
    let mut  dst =  false;

    if let Some(cap) = STD_TIME_SUFFIX.captures(zone) {
        if let Some(capture) =  cap.get(1) {
            zone_str =  capture.as_str();
        }
    } else if let Some(cap) = DAYLIGHT_TIME_SUFFIX.captures(zone) {
        if let Some(capture) =  cap.get(1) {
            zone_str =  capture.as_str();
        }
        dst = true;
    } else if let Some(cap) = DST_SUFFIX.captures(zone) {
        if let Some(capture) =  cap.get(1) {
            zone_str =  capture.as_str();
        }
        dst = true;
    }

    if let Some(zone_offset) = super::zonetab::zone_offset(zone_str) {
		let offset = if dst { zone_offset + 3600 } else { zone_offset };
        return Some(offset);
    }

    if let Some(cap) = GMT_UTC_PREFIX.captures(zone_str) {
        if let Some(capture) =  cap.get(1) {
            zone_str =  capture.as_str();
        }
    }

    if zone_str.starts_with(&['+','-'][..]){
        let (sign, rest) = zone_str.split_at(1);
        let sign =  if sign.starts_with('-') { -1 } else { 1 };

        let (hours_str, split_char, rest) = slice_number_part(rest);
        let mut hours = hours_str.parse::<i64>().unwrap();
        let mut  mins =  0;
        let mut  secs =  0;

        if split_char == ":" {
            let (mins_str, split_char, rest) = slice_number_part(rest);
            let mins = mins_str.parse::<i64>().unwrap_or_default();
            if split_char == ":" {
                if let Ok(secs) = rest.parse::<i64>() {
                    return Some(sign * (secs + 60*mins + 3600*hours));
                }
            }
            return Some(sign * (60*mins + 3600*hours));
        }

        if split_char == "," || split_char == "." {
            let (mins_str, _split_char, _rest) = slice_number_part(rest);
            let mut mins = 0.0;
            if !mins_str.is_empty() {
                let frac_str = format!("0.{}", mins_str);
                if let Ok(fraction) = frac_str.parse::<f64>() {
                    mins = fraction;
                }
            }
            return Some(sign * (hours * 3600 + (mins * 3600.0) as i64));
        }
        let l = hours_str.len();
        if  l >= 2 {
            if l >= 1 {
                hours = hours_str[0..(2 - l % 2)].parse::<i64>().unwrap();
            }
            if l >= 3 {
                mins = hours_str[(2 - l % 2)..(4 - l % 2)].parse::<i64>().unwrap();
            }
            if l >= 5 {
                secs = hours_str[(4 - l % 2)..(6 - l % 2)].parse::<i64>().unwrap();
            }
        }
        return Some(sign * (hours * 3600 + mins * 60 + secs));
    }
    None
}



fn parse_year(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref YEAR_REGEX: Regex = RegexBuilder::new(r"'(\d+)\b")
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = YEAR_REGEX.replace(string, |caps: &Captures| {
        matched =  true;
        if let Some(x) = caps.get(1)
                .map(|x| x.as_str())
                .map(|x| x.parse::<i64>().unwrap()) { datetime.year = Some(x); }


        SPACE
    });
    *string = result.to_string();
    matched
}


fn parse_mon(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref MON_REGEX: Regex = RegexBuilder::new(r"\b(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)\S*")
                        .case_insensitive(true)
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = MON_REGEX.replace(string, |caps: &Captures| {
        matched =  true;
        if let Some(x) = caps.get(1)
                .map(|x| months_num(x.as_str()))
                .map(|x| x.parse::<u64>().unwrap()) { datetime.mon = Some(x); }


        SPACE
    });
    *string = result.to_string();
    matched
}


fn parse_mday(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref MDAY_REGEX: Regex = RegexBuilder::new(r"(\d+)(st|nd|rd|th)\b")
                        .case_insensitive(true)
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = MDAY_REGEX.replace(string, |caps: &Captures| {
        matched =  true;
        if let Some(x) = caps.get(1)
                .map(|x| x.as_str())
                .map(|x| x.parse::<u64>().unwrap()) { datetime.mday = Some(x); }


        SPACE
    });
    *string = result.to_string();
    matched
}


fn parse_day(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref RE: Regex = RegexBuilder::new(r"\b(sun|mon|tue|wed|thu|fri|sat)[^-/\d\s]*").case_insensitive(true).build().unwrap();
    }
    let result = RE.replace(string, |caps: &Captures| {
        if let Some(cap) = &caps.get(1) {
            datetime.wday = Some(day_num(cap.as_str()));
        }

        SPACE
    });
    *string = result.to_string();
    true
}

fn parse_time(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref TIME_WITH_ZONE: Regex = RegexBuilder::new(r"
            (
                (?:
                \d+\s*:\s*\d+
                (?:
                    \s*:\s*\d+(?:[,.]\d*)?
                )?
                |
                \d+\s*h(?:\s*\d+m?(?:\s*\d+s?)?)?
                )
                (?:
                \s*
                [ap](?:m\b|\.m\.)
                )?
            |
                \d+\s*[ap](?:m\b|\.m\.)
            )
            (?:
                \s*
                (
                (?:gmt|utc?)?[-+]\d+(?:[,.:]\d+(?::\d+)?)?
                |
                (?-i:[[:alpha:].\s]+)(?:standard|daylight)\stime\b
                |
                (?-i:[[:alpha:]]+)(?:\sdst)?\b
                )
            )?
        ")
        .case_insensitive(true)
        .ignore_whitespace(true)
        .build()
        .unwrap();

        static ref TIME: Regex = RegexBuilder::new(r"
            \A(\d+)h?
                (?:\s*:?\s*(\d+)m?
                    (?:
                        \s*:?\s*(\d+)(?:[,.](\d+))?s?
                    )?
                )?
            (?:\s*([ap])(?:m\b|\.m\.))?
        ")
        .case_insensitive(true)
        .ignore_whitespace(true)
        .build()
        .unwrap();
    }

    let result = TIME_WITH_ZONE.replace(string, |caps: &Captures| {
        if let Some(zone) = &caps.get(2) {
            datetime.zone =  Some(zone.as_str().to_string());
        }
        if let Some(time_part) = &caps.get(1) {
            if let Some(time_caps) = TIME.captures(time_part.as_str()) {
                if let Some(hour_str) = &time_caps.get(1) {
                    if let Ok(hour) = hour_str.as_str().parse::<i64>() {
                        datetime.hour = Some(hour as u64);
                    }
                }
                if let Some(min_str) = &time_caps.get(2) {
                    if let Ok(min) = min_str.as_str().parse::<i64>() {
                        datetime.min = Some(min as u64);
                    }
                }
                if let Some(sec_str) = &time_caps.get(3) {
                    if let Ok(sec) = sec_str.as_str().parse::<i64>() {
                        datetime.sec = Some(sec as u64);
                    }
                }
                if let Some(fraction_str) = &time_caps.get(4) {
                    let frac_str = format!("0.{}", fraction_str.as_str());
                    if let Ok(fraction) = frac_str.parse::<f64>() {
                        datetime.sec_fraction = Some(fraction);
                    }
                }
                if let Some(am_pm_str) = &time_caps.get(5) {
                    if let Some(hr) = datetime.hour {
                        datetime.hour = Some(hr%12);
                    }

                    if am_pm_str.as_str().to_lowercase() == "p" {
                        if let Some(hr) = datetime.hour {
                            datetime.hour = Some(hr+12);
                        }
                    }
                }
            }
        }

        SPACE
    });
    *string = result.to_string();
    true
}

fn parse_bc(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref BC_REGEX: Regex = RegexBuilder::new(r"\b(bc\b|bce\b|b\.c\.|b\.c\.e\.)")
                        .ignore_whitespace(true)
                        .case_insensitive(true)
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = BC_REGEX.replace(string, |_caps: &Captures| {
        matched =  true;
        datetime.bc = true;

        SPACE
    });
    *string = result.to_string();
    matched
}

fn parse_frag(string: &mut String, datetime: &mut DateTime) -> bool {
    lazy_static! {
        static ref FRAG_REGEX: Regex = RegexBuilder::new(r"\A\s*(\d{1,2})\s*\z")
                        .ignore_whitespace(true)
                        .case_insensitive(true)
                        .build()
                        .unwrap();
    }
    let mut matched = false;
    let result = FRAG_REGEX.replace(string, |caps: &Captures| {
        matched =  true;

        if let Some(n) = caps.get(1)
                .map(|x| x.as_str())
                .map(|x| x.parse::<i64>().unwrap()) {
            if datetime.hour.is_some() && datetime.mday.is_none() && n >= 1 && n <= 31 {
                datetime.mday = Some(n as u64);
            }
            if datetime.mday.is_some() && datetime.hour.is_none() && n >= 0 && n <= 24 {
                datetime.hour = Some(n as u64);
            }
        }


        SPACE
    });
    *string = result.to_string();
    matched
}

// Parses the given representation of date and time, and returns a
// hash of parsed elements. This method does not function as a validator.
// If the optional second argument is true and the detected year is in the
//  range “00” to “99”, considers the year a 2-digit form and makes it full.
// Date._parse('2001-02-03') #=> {:year=>2001, :mon=>2, :mday=>3}
pub fn date_parse(string: &str, comp: bool) -> DateTime {
    let (dt, _unparsed) = date_parse2(string, comp);
    dt
}

// Returns the remaining portion of the string after processing  along with datetime
pub fn date_parse2(string: &str, comp: bool) -> (DateTime, String) {
    lazy_static! {
        static ref NON_DATE_CHARS_RE: Regex = Regex::new(r"[^-+',./:@[:alnum:]\[\]]+").unwrap();
    }

    let string = string.to_owned();

    let mut string = NON_DATE_CHARS_RE.replace_all(&string, SPACE).to_string();
    let mut datetime  = DateTime { comp: Some(comp), ..Default::default() };

    if check_classes(&string, HAVE_ALPHA) {
        parse_day(&mut string, &mut datetime);
    }

    if check_classes(&string, HAVE_DIGIT) {
        parse_time(&mut string, &mut datetime);
    }

    let mut matched = false;
    if check_classes(&string, HAVE_ALPHA | HAVE_DIGIT) {
        matched =  parse_eu(&mut string, &mut datetime);

        if !matched {
            matched = parse_us(&mut string, &mut datetime);
        }

    }
    if !matched && check_classes(&string, HAVE_DIGIT | HAVE_DASH) {
        matched = parse_iso(&mut string, &mut datetime);
    }

    if !matched && check_classes(&string, HAVE_DIGIT | HAVE_DOT) {
        matched = parse_jis(&mut string, &mut datetime);
    }

    if !matched && check_classes(&string, HAVE_ALPHA | HAVE_DIGIT | HAVE_DASH) {
        matched = parse_vms(&mut string, &mut datetime);
    }

    if !matched && check_classes(&string, HAVE_DIGIT | HAVE_SLASH) {
        matched = parse_sla(&mut string, &mut datetime);
    }
    if !matched && check_classes(&string, HAVE_DIGIT | HAVE_DOT) {
        matched = parse_dot(&mut string, &mut datetime);
    }
    if !matched && check_classes(&string, HAVE_DIGIT) {
        matched = parse_iso2(&mut string, &mut datetime);
    }
    if !matched && check_classes(&string, HAVE_DIGIT) {
        matched = parse_year(&mut string, &mut datetime);
    }
    if !matched && check_classes(&string, HAVE_ALPHA) {
        matched = parse_mon(&mut string, &mut datetime);
    }
    if !matched && check_classes(&string, HAVE_DIGIT) {
        matched = parse_mday(&mut string, &mut datetime);
    }
    if !matched && check_classes(&string, HAVE_DIGIT) {
        parse_ddd(&mut string, &mut datetime);
    }

    if check_classes(&string, HAVE_ALPHA) {
        parse_bc(&mut string, &mut datetime);

    }
    if check_classes(&string, HAVE_DIGIT) {
        parse_frag(&mut string, &mut datetime);
    }

    if datetime.bc {
        if let Some(cwyear) =  datetime.cwyear {
            datetime.cwyear = Some(-cwyear + 1);
        }
        if let Some(year) =  datetime.year {
            datetime.year = Some(-year + 1);
        }
    }

    if let Some(true) = datetime.comp.take() {
        if let Some(cwyear) = datetime.cwyear {
            if (0..=99).contains(&cwyear) {
                if cwyear >= 69 {
                    datetime.cwyear = Some(cwyear + 1900);
                } else {
                    datetime.cwyear = Some(cwyear + 2000);
                }
            }
        }

        if let Some(year) = datetime.year {
            if (0..=99).contains(&year) {
                if year >= 69 {
                    datetime.year = Some(year + 1900);
                } else {
                    datetime.year = Some(year + 2000);
                }
            }
        }
    }

    if let Some(zone) = &datetime.zone {
        if datetime.offset.is_none() {
            datetime.offset = date_zone_to_diff(zone.as_str());
        }
    }

    (datetime, string.trim().to_owned())
}
