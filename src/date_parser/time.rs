use std::str::FromStr;

use phf::phf_map;
use regex::{Regex, RegexBuilder};
use chrono::{offset::TimeZone, DateTime, Datelike, FixedOffset, Timelike};



/// Return the number of seconds the specified time zone differs
/// from UTC.
///
/// Numeric time zones that include minutes, such as
/// <code>-10:00</code> or <code>+1330</code> will work, as will
/// simpler hour-only time zones like <code>-10</code> or
/// <code>+13</code>.
///
/// Textual time zones listed in ZoneOffset are also supported.
///
/// If the time zone does not match any of the above, +zone_offset+
/// will check if the local time zone (both with and without
/// potential Daylight Saving \Time changes being in effect) matches
/// +zone+. Specifying a value for +year+ will change the year used
/// to find the local time zone.
///
/// If +zone_offset+ is unable to determine the offset, nil will be
/// returned.
///
///
///     Time.zone_offset("EST") #=> -18000
///
/// You must require 'time' to use this method.

static ZONE_OFFSET: phf::Map<&'static str, i32> = phf_map! {
    "UTC" => 0,
    //  # ISO 8601
    "Z" => 0,
    //  # RFC 822
    "UT" => 0, "GMT" => 0,
    "EST" => -5, "EDT" => -4,
    "CST" => -6, "CDT" => -5,
    "MST" => -7, "MDT" => -6,
    "PST" => -8, "PDT" => -7,
    //  # Following definition of military zones is original one.
    //  # See RFC 1123 and RFC 2822 for the error in RFC 822.
    "A" => 1, "B" => 2, "C" => 3, "D" => 4,  "E" => 5,  "F" => 6,
    "G" => 7, "H" => 8, "I" => 9, "K" => 10, "L" => 11, "M" => 12,
    "N" => -1, "O" => -2, "P" => -3, "Q" => -4,  "R" => -5,  "S" => -6,
    "T" => -7, "U" => -8, "V" => -9, "W" => -10, "X" => -11, "Y" => -12,
};


fn zone_offset(zone: &str, year: Option<i32>) -> Option<i32> {
    let zone = zone.to_ascii_uppercase();

    lazy_static! {
        static ref ZONE_NUM_REGEX: Regex = RegexBuilder::new(r"\A([+-])(\d\d)(:?)(\d\d)(?:(:?)(\d\d))?\z")
            .case_insensitive(true)
            .ignore_whitespace(true)
            .build()
            .unwrap();
        static ref ZONE_MATCH2_REGEX: Regex = RegexBuilder::new(r"\A[+-]\d\d\z")
            .case_insensitive(true)
            .ignore_whitespace(true)
            .build()
            .unwrap();
    }

    if let Some(captures) = ZONE_NUM_REGEX.captures(&zone) {
        // Check if the third and 6th values match. Trick to do matching without regex backreference
        // To simulate ruby regex:
        // /\A([+-])(\d\d)(:?)(\d\d)(?:\3(\d\d))?\z/
        //                              ^
        //                              |
        //                             this( \3 )
        if captures.get(5).is_none() || (captures.get(3).map(|x| x.as_str()) == captures.get(5).map(|x| x.as_str())) {
            let sign = if captures.get(1).unwrap().as_str() == "-" {
                -1
            } else {
                1
            };
            let hour = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let mins = captures.get(4).map(|x| x.as_str().parse::<i32>().unwrap_or_default()).unwrap_or_default();
            let secs = captures.get(6).map(|x| x.as_str().parse::<i32>().unwrap_or_default()).unwrap_or_default();
            let offset = Some(sign * (( hour * 60 + mins) * 60 + secs));
            return offset;
        }
    }
    if ZONE_MATCH2_REGEX.is_match(&zone) {
        return Some(zone.parse::<i32>().unwrap() * 3600);
    }
    if let Some(offset) = ZONE_OFFSET.get(&zone) {
        return Some(offset * 3600);
    }

    let year = year.unwrap_or_else(|| {
        let now_fixed_offset: chrono::DateTime<chrono::FixedOffset> = chrono::Local::now().into();
        now_fixed_offset.year()
    });

    if let Ok(tz_str) = iana_time_zone::get_timezone() {
        if let Ok(tz) = tz_str.parse::<chrono_tz::Tz>() {
            let dt = tz.ymd(year, 1, 1).and_hms(0, 0, 0);
            let off = dt.offset().to_string();
            if off.to_uppercase() == zone {
                return Some(dt.fixed_offset().offset().local_minus_utc());
            }

            let dt = tz.ymd(year, 7, 1).and_hms(0, 0, 0);
            let off = dt.offset().to_string();
            if off.to_uppercase() == zone {
                return Some(dt.fixed_offset().offset().local_minus_utc());
            }
        };
    }
    None
}

// * +0000
// In RFC 2822, +0000 indicate a time zone at Universal Time.
// Europe/Lisbon is "a time zone at Universal Time" in Winter.
// Atlantic/Reykjavik is "a time zone at Universal Time".
// Africa/Dakar is "a time zone at Universal Time".
// So +0000 is a local time such as Europe/London, etc.
// * GMT
// GMT is used as a time zone abbreviation in Europe/London,
// Africa/Dakar, etc.
// So it is a local time.

// * -0000, -00:00
// In RFC 2822, -0000 the date-time contains no information about the
// local time zone.
// In RFC 3339, -00:00 is used for the time in UTC is known,
// but the offset to local time is unknown.
// They are not appropriate for specific time zone such as
// Europe/London because time zone neutral,
// So -00:00 and -0000 are treated as UTC.
fn is_zone_utc(zone: &str) -> bool {
    lazy_static! {
        static ref ZONE_UTC_REGEX: Regex = RegexBuilder::new(r"\A(?:-00:00|-0000|-00|UTC|Z|UT)\z")
            .case_insensitive(true)
            .ignore_whitespace(true)
            .build()
            .unwrap();
    }
    ZONE_UTC_REGEX.is_match(zone)
}

const LEAP_YEAR_MONTH_DAYS: [i32;12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const COMMON_YEAR_MONTH_DAYS: [i32;12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn month_days(y: i32, m: i32) -> crate::Result<i32> {
    if !(1..=12).contains(&m) {
        return Err(crate::ParseError::OutOfRangeError("month".to_owned(), m.to_string()));
    }
    if ((y % 4 == 0) && (y % 100 != 0)) || (y % 400 == 0) {
        Ok(LEAP_YEAR_MONTH_DAYS[(m-1) as usize])
    } else {
        Ok(COMMON_YEAR_MONTH_DAYS[(m-1) as usize])
    }
}


fn divmod(num: i32, val: i32) -> (i32, i32) {
    (num.div_euclid(val), num.rem_euclid(val))
}

#[allow(clippy::comparison_chain)] // To make this look exactly like the ruby code
fn apply_offset(
    mut year: i32,
    mut mon: i32,
    mut day: i32,
    mut hour: i32,
    mut min: i32,
    mut sec: i32,
    mut off: i32,
) -> (i32, i32, i32, i32, i32, i32) {
    let mut o: i32 = 0;
    if off < 0 {
        off = -off;

        (off, o) = divmod(off, 60);
        if o != 0 {
            sec += o;
            (o, sec) = divmod(sec, 60);
            off += o;
        }

        (off, o) = divmod(off, 60);
        if o != 0 {
            min += o;
            (o, min) = divmod(min, 60);
            off += o;
        }

        (off, o) = divmod(off, 24);
        if o != 0 {
            hour += o;
            (o, hour) = divmod(hour, 24);
            off += o;
        }

        if off != 0 {
            day += off;
            let days = month_days(year, mon);
            if days.is_ok() && days.unwrap() < day {
                mon += 1;
                if 12 < mon {
                    mon = 1;
                    year += 1;
                }
                day = 1;
            }
        }
    } else if 0 < off {
        (off, o) = divmod(off, 60);
        if o != 0 {
            sec -= o;
            (o, sec) = divmod(sec, 60);
            off -= o;
        }

        (off, o) = divmod(off, 60);
        if o != 0 {
            min -= o;
            (o, min) = divmod(min, 60);
            off -= o;
        }

        (off, o) = divmod(off, 24);
        if o != 0 {
            hour -= o;
            (o, hour) = divmod(hour, 24);
            off -= o;
        }

        if off != 0 {
            day -= off;
            if day < 1 {
                mon -= 1;
                if mon < 1 {
                    year -= 1;
                    mon = 12;
                }
                day = month_days(year, mon).unwrap();
            }
        }
    }
    (year, mon, day, hour, min, sec)
}


fn validated_ymd(year: i32, month: i32, day: i32) -> crate::Result<(i32, u32, u32)> {
    if !(1..=12).contains(&month) {
        return Err(crate::ParseError::OutOfRangeError("month".to_owned(), month.to_string()));
    }
    let max_days = month_days(year, month)?;
    if !(1..=max_days).contains(&day) {
        return Err(crate::ParseError::OutOfRangeError("day".to_owned(), day.to_string()));
    }
    Ok((year, month as u32, day as u32))
}

fn validated_hms(hour: i32, min: i32, sec: i32) -> crate::Result<(u32, u32, u32)> {
    if !(0..=23).contains(&hour) {
        return Err(crate::ParseError::OutOfRangeError("hour".to_owned(), hour.to_string()));
    }
    if !(0..=59).contains(&min) {
        return Err(crate::ParseError::OutOfRangeError("minute".to_owned(), min.to_string()));
    }
    if !(0..=59).contains(&sec) {
        return Err(crate::ParseError::OutOfRangeError("second".to_owned(), sec.to_string()));
    }

    Ok((hour as u32, min as u32, sec as u32))
}
// Takes a string representation of a Time and attempts to parse it
// using a heuristic.
//
// This method **does not** function as a validator.  If the input
// string does not match valid formats strictly, you may get a
// cryptic result.  Should consider to use `Time.strptime` instead
// of this method as possible.
//
//     require 'time'
//
//     Time.parse("2010-10-31") #=> 2010-10-31 00:00:00 -0500
//
// Any missing pieces of the date are inferred based on the current date.
//
//     require 'time'
//
//     # assuming the current date is "2011-10-31"
//     Time.parse("12:00") #=> 2011-10-31 12:00:00 -0500
//
// We can change the date used to infer our missing elements by passing a second
// object that responds to #mon, #day and #year, such as Date, Time or DateTime.
// We can also use our own object.
//
//     require 'time'
//
//     class MyDate
//       attr_reader :mon, :day, :year
//
//       def initialize(mon, day, year)
//         @mon, @day, @year = mon, day, year
//       end
//     end
//
//     d  = Date.parse("2010-10-28")
//     t  = Time.parse("2010-10-29")
//     dt = DateTime.parse("2010-10-30")
//     md = MyDate.new(10,31,2010)
//
//     Time.parse("12:00", d)  #=> 2010-10-28 12:00:00 -0500
//     Time.parse("12:00", t)  #=> 2010-10-29 12:00:00 -0500
//     Time.parse("12:00", dt) #=> 2010-10-30 12:00:00 -0500
//     Time.parse("12:00", md) #=> 2010-10-31 12:00:00 -0500
//
// If a block is given, the year described in +date+ is converted
// by the block.  This is specifically designed for handling two
// digit years. For example, if you wanted to treat all two digit
// years prior to 70 as the year 2000+ you could write this:
//
//     require 'time'
//
//     Time.parse("01-10-31") {|year| year + (year < 70 ? 2000 : 1900)}
//     #=> 2001-10-31 00:00:00 -0500
//     Time.parse("70-10-31") {|year| year + (year < 70 ? 2000 : 1900)}
//     #=> 1970-10-31 00:00:00 -0500
//
// If the upper components of the given time are broken or missing, they are
// supplied with those of +now+.  For the lower components, the minimum
// values (1 or 0) are assumed if broken or missing.  For example:
//
//     require 'time'
//
//     # Suppose it is "Thu Nov 29 14:33:20 2001" now and
//     # your time zone is EST which is GMT-5.
//     now = Time.parse("Thu Nov 29 14:33:20 2001")
//     Time.parse("16:30", now)     #=> 2001-11-29 16:30:00 -0500
//     Time.parse("7/23", now)      #=> 2001-07-23 00:00:00 -0500
//     Time.parse("Aug 31", now)    #=> 2001-08-31 00:00:00 -0500
//     Time.parse("Aug 2000", now)  #=> 2000-08-01 00:00:00 -0500
//
// Since there are numerous conflicts among locally defined time zone
// abbreviations all over the world, this method is not intended to
// understand all of them.  For example, the abbreviation "CST" is
// used variously as:
//
//     -06:00 in America/Chicago,
//     -05:00 in America/Havana,
//     +08:00 in Asia/Harbin,
//     +09:30 in Australia/Darwin,
//     +10:30 in Australia/Adelaide,
//     etc.
//
// Based on this fact, this method only understands the time zone
// abbreviations described in RFC 822 and the system time zone, in the
// order named. (i.e. a definition in RFC 822 overrides the system
// time zone definition.)  The system time zone is taken from
// <tt>Time.local(year, 1, 1).zone</tt> and
// <tt>Time.local(year, 7, 1).zone</tt>.
// If the extracted time zone abbreviation does not match any of them,
// it is ignored and the given time is regarded as a local time.
//
// ArgumentError is raised if Date._parse cannot extract information from
// +date+ or if the Time class cannot represent specified date.
//
// This method can be used as a fail-safe for other parsing methods as:
//
//   Time.rfc2822(date) rescue Time.parse(date)
//   Time.httpdate(date) rescue Time.parse(date)
//   Time.xmlschema(date) rescue Time.parse(date)
//
// A failure of Time.parse should be checked, though.
//
// You must require 'time' to use this method.

// def force_zone!(t, zone, offset=nil)
//   if zone_utc?(zone)
//     t.utc
//   elsif offset ||= zone_offset(zone)
//     # Prefer the local timezone over the fixed offset timezone because
//     # the former is a real timezone and latter is an artificial timezone.
//     t.localtime
//     if t.utc_offset != offset
//       # Use the fixed offset timezone only if the local timezone cannot
//       # represent the given offset.
//       t.localtime(offset)
//     end
//   else
//     t.localtime
//   end
// end

fn to_local_time(t: chrono::DateTime<chrono::FixedOffset>, offset: Option<i32>) -> crate::Result<chrono::DateTime<chrono::FixedOffset>> {
    if let Some(off) = offset {
        let fixed_offset = FixedOffset::east_opt(off).ok_or(crate::ParseError::OffsetOutOfBounds)?;
        return Ok(t.with_timezone(&fixed_offset));
    }

    let now_fixed_offset: chrono::DateTime<chrono::FixedOffset> = chrono::Local::now().into();
    let local_offset = now_fixed_offset.offset();
    Ok(t.with_timezone(local_offset))
}

fn force_zone(time: chrono::DateTime<chrono::FixedOffset>, zone: &str, offset: Option<i32>) -> crate::Result<chrono::DateTime<chrono::FixedOffset>> {
    if is_zone_utc(zone) {
        let res = time.to_utc();
        return Ok(res.fixed_offset());
    }
    let offset = if offset.is_none() {
        zone_offset(zone, None)
    } else {
        offset
    };

    if let Some(off) = offset {
        let time = to_local_time(time, None)?;

        if time.offset().local_minus_utc() != off {
            return to_local_time(time, offset);
        } else {
            return Ok(time);
        }
    }
    to_local_time(time, None)
}


#[allow(clippy::too_many_arguments)]
fn make_time(
    date: &str,
    mut year: Option<i32>,
        yday: Option<i32>,
    mut mon: Option<u32>,
    mut mday: Option<u32>,
    mut hour: Option<u32>,
    mut min: Option<u32>,
    mut sec: Option<u32>,
    sec_fraction: Option<f64>,
    zone: Option<&str>,
    mut now: Option<chrono::DateTime<FixedOffset>>,
) -> crate::Result<chrono::DateTime<FixedOffset>> {
    if yday.is_none() &&
        mon.is_none() &&
        mday.is_none() &&
        hour.is_none() &&
        min.is_none() &&
        sec.is_none() &&
        sec_fraction.is_none()
    {
        return Err(crate::ParseError::MissingTimeInformationError(date.to_owned()));
    }

    let mut off = None;
    let mut off_year = None;
    if year.is_some() || now.is_some() {
        off_year = if year.is_some() {
            year
        } else {
            now.map(|x| x.year())
        };

        if let Some(zone) = &zone {
            off = zone_offset(zone, off_year);
        }
    }

    if let Some(yday) = &yday {
        if !(1..=366).contains(yday) {
            return Err(crate::ParseError::OutOfRangeError("yday".to_owned(), yday.to_string()));
        }

        let mut mon = (yday -1).div_euclid(31) + 1;
        let mut mday: i32 = (yday -1).rem_euclid(31) + 1;

        let t = make_time(date, year, None, Some(mon as u32), Some(mday as u32), hour, min, sec, sec_fraction, zone, now)?;
        let diff = yday - (t.ordinal() as i32);
        if diff == 0 {
            return Ok(t);
        }
        mday += diff;

        if mday > 28 {
            let mon_mday = month_days(year.unwrap(), mon)?;
            if mday > mon_mday {
                mon += 1;
                if mon > 12 {
                    return Err(crate::ParseError::OutOfRangeError("yday".to_owned(), yday.to_string()));
                }
                mday -= mon_mday;
            }
        }
        return make_time(date, year, None, Some(mon as u32), Some(mday as u32), hour, min, sec, sec_fraction, zone, now);
    }

    if let Some(now_dt) = &now {
        if let Some(off) = off {
            if now_dt.offset().local_minus_utc() != off {
                if let Some(offset) = chrono::FixedOffset::east_opt(off) {
                    now = Some(now_dt.with_timezone(&offset));
                }
            }
        } else {
            let now_fixed_offset: chrono::DateTime<chrono::FixedOffset> = chrono::Local::now().into();
            let local_offset = now_fixed_offset.offset();
            now = Some(now_dt.with_timezone(local_offset));
        }
    }
    let mut usec = None;
    if let Some(frac) = sec_fraction {
        usec = Some(frac * 1000000.0);
    }

    if let Some(now_dt) = now {
        loop {
            if year.is_some() {
                break;
            }
            year = Some(now_dt.year());

            if mon.is_some() {
                break;
            }
            mon = Some(now_dt.month());
            if mday.is_some() {
                break;
            }
            mday = Some(now_dt.day());
            if hour.is_some() {
                break;
            }
            hour = Some(now_dt.hour());
            if min.is_some() {
                break;
            }
            min = Some(now_dt.minute());
            if sec.is_some() {
                break;
            }
            sec = Some(now_dt.second());
            if sec_fraction.is_some() {
                break;
            }
            usec = Some((now_dt.timestamp_nanos_opt().unwrap_or_default() as f64)/1000.0);
        }
    }
    let year = year.unwrap_or(1970);
    let mon = mon.unwrap_or(1);
    let mday = mday.unwrap_or(1);
    let hour = hour.unwrap_or(0);
    let min = min.unwrap_or(0);
    let sec = sec.unwrap_or(0);
    let usec = usec.unwrap_or(0.0);

    if Some(year) != off_year {
        off = None;
        if let Some(zone) = zone {
            off = zone_offset(zone, Some(year));
        }
    }
    if let Some(offset) = off {
        let (year, mon, mday, hour, min, sec) = apply_offset(year, mon.try_into().unwrap(), mday.try_into().unwrap(), hour.try_into().unwrap(), min.try_into().unwrap(), sec.try_into().unwrap(), off.unwrap());
        let (year, mon, mday) = validated_ymd(year, mon, mday)?;
        let (hour, min, sec) = validated_hms(hour, min, sec)?;
        let dt: DateTime<FixedOffset> = chrono::Utc.ymd(year, mon, mday).and_hms_micro_opt(hour, min, sec, usec as u32).unwrap().fixed_offset();
        force_zone(dt, zone.unwrap(), Some(offset))
    } else {
        let (year, mon, mday) = validated_ymd(year, mon as i32, mday as i32)?;
        let (hour, min, sec) = validated_hms(hour as i32, min as i32, sec as i32)?;
        let dt = chrono::Local.ymd(year, mon, mday).and_hms_micro_opt(hour, min, sec, usec as u32).unwrap().fixed_offset();
        Ok(dt)
    }
}


pub fn parse_with_custom_time_and_year(
    date: &str,
    now: Option<chrono::DateTime<FixedOffset>>,
    year_fn: Option<fn(i32) -> i32>
)
-> crate::Result<DateTime<FixedOffset>>
{
    let now = now.unwrap_or_else(|| chrono::Local::now().fixed_offset());
    let comp = year_fn.is_none();
    let d = crate::date_parser::date_parse(date, comp);
    let mut year = d.year;
    if year.is_some() && !comp {
        if let Some(func) = year_fn {
            year = Some(func(year.unwrap()));
        }
    }
    let time = make_time(date, year, d.yday, d.mon, d.mday, d.hour, d.min, d.sec, d.sec_fraction, d.zone.as_deref(), Some(now));
    time
}

pub fn parse(date: &str,) -> crate::Result<DateTime<FixedOffset>>
{
    parse_with_custom_time_and_year(date, None, None)
}

pub fn local(year: Option<i32>, month: Option<u32>, day: Option<u32>, hour: Option<u32>, min: Option<u32>, sec: Option<u32>, tz: Option<&str>) -> crate::Result<DateTime<FixedOffset>> {
    let year = year.unwrap_or_else(||{
        chrono::Local::now().year()
    });

    let dt = if let Some(tz_val) = tz {
        // if let Some(val) = chrono_tz::Tz::parse(tz_val) {

        // }

        let fixed_offset = FixedOffset::from_str(tz_val).unwrap();
        fixed_offset.with_ymd_and_hms(
            year,
            month.unwrap_or(1),
            day.unwrap_or(1),
            hour.unwrap_or(0),
            min.unwrap_or(0),
            sec.unwrap_or(0),
            ).earliest()
    } else {
        let tz_str = iana_time_zone::get_timezone().unwrap();
        let tz: chrono_tz::Tz = tz_str.parse().unwrap();
        tz.with_ymd_and_hms(
            year,
            month.unwrap_or(1),
            day.unwrap_or(1),
            hour.unwrap_or(0),
            min.unwrap_or(0),
            sec.unwrap_or(0),
            ).earliest().map(|x| x.fixed_offset())
    };
    if let Some(val) = dt {
        Ok(val)
    } else {
        Err(crate::ParseError::DateError())
    }
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! local_time {
    ($yr:expr) => {{
        $crate::date_parser::time::local(Some($yr),None,None, None,None,None, None).unwrap()
    }};
    ($yr:expr, $mon:expr) => {{
        $crate::date_parser::time::local(Some($yr),Some($mon),None, None,None,None, None).unwrap()
    }};
    ($yr:expr, $mon:expr, $day:expr) => {{
        $crate::date_parser::time::local(Some($yr),Some($mon),Some($day), None,None,None, None).unwrap()
    }};
    ($yr:expr, $mon:expr, $day:expr, $hr:expr) => {{
        $crate::date_parser::time::local(Some($yr),Some($mon),Some($day), Some($hr),None,None, None).unwrap()
    }};
    ($yr:expr, $mon:expr, $day:expr, $hr:expr, $min:expr) => {{
        $crate::date_parser::time::local(Some($yr),Some($mon),Some($day), Some($hr),Some($min),None, None).unwrap()
    }};
    ($yr:expr, $mon:expr, $day:expr, $hr:expr, $min:expr, $sec:expr) => {{
        $crate::date_parser::time::local(Some($yr),Some($mon),Some($day), Some($hr),Some($min),Some($sec), None).unwrap()
    }};
    ($yr:expr, $mon:expr, $day:expr, $hr:expr, $min:expr, $sec:expr, $tz:expr) => {{
        $crate::date_parser::time::local(Some($yr),Some($mon),Some($day), Some($hr),Some($min),Some($sec), Some($tz)).unwrap()
    }};
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! utc_time {
    ($yr:expr) => {{
        $crate::date_parser::time::local(Some($yr),None,None, None,None,None, Some("+00:00")).unwrap()
    }};
    ($yr:expr, $mon:expr) => {{
        $crate::date_parser::time::local(Some($yr),Some($mon),None, None,None,None, Some("+00:00")).unwrap()
    }};
    ($yr:expr, $mon:expr, $day:expr) => {{
        $crate::date_parser::time::local(Some($yr),Some($mon),Some($day), None,None,None, Some("+00:00")).unwrap()
    }};
    ($yr:expr, $mon:expr, $day:expr, $hr:expr) => {{
        $crate::date_parser::time::local(Some($yr),Some($mon),Some($day), Some($hr),None,None, Some("+00:00")).unwrap()
    }};
    ($yr:expr, $mon:expr, $day:expr, $hr:expr, $min:expr) => {{
        $crate::date_parser::time::local(Some($yr),Some($mon),Some($day), Some($hr),Some($min),None, Some("+00:00")).unwrap()
    }};
    ($yr:expr, $mon:expr, $day:expr, $hr:expr, $min:expr, $sec:expr) => {{
        $crate::date_parser::time::local(Some($yr),Some($mon),Some($day), Some($hr),Some($min),Some($sec), Some("+00:00")).unwrap()
    }};
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_completion() {
        let now = local_time!(2001,11,29, 21,26,35);

        assert_eq!(
            local_time!(           2001,11,29,21,12),
            super::parse_with_custom_time_and_year("2001/11/29 21:12", Some(now), None).unwrap()
        );
        assert_eq!(
            local_time!(           2001,11,29),
            super::parse_with_custom_time_and_year("2001/11/29", Some(now), None).unwrap()
        );
        assert_eq!(
            local_time!(            2001,11,29),
            super::parse_with_custom_time_and_year(      "11/29", Some(now), None).unwrap()
        );
        assert_eq!(
            local_time!( 2001,11,29, 10,22),
            super::parse_with_custom_time_and_year(  "10:22", Some(now), None).unwrap()
        );
        assert!(
            super::parse_with_custom_time_and_year(  "foo", Some(now), None).is_err()
        );
    }

    #[test]
    fn test_completion_with_different_timezone() {
        let now_local: DateTime<FixedOffset> = local_time!(2001,2,3,0,0,0,"+09:00");
        let now = now_local.to_utc();
        let t: DateTime<FixedOffset> = super::parse_with_custom_time_and_year("10:20:30 GMT", Some(now.fixed_offset()), None).unwrap();
        assert_eq!(utc_time!(2001,2,2,10,20,30), t);
        assert_eq!(t, t.to_utc());
        assert_eq!(0, t.offset().local_minus_utc());
    }

    #[test]
    fn test_invalid() {
        // Out of range arguments
        assert!(parse("2014-13-13T18:00:00-0900").is_err());
    }

    #[test]
    fn test_zone_0000() {
        assert_eq!(parse("2000-01-01T00:00:00Z").unwrap().offset().utc_minus_local(), 0 );
        assert_eq!(parse("2000-01-01T00:00:00-00:00").unwrap().offset().utc_minus_local(), 0 );
        assert_eq!(parse("2000-01-01T00:00:00-00:00").unwrap().to_rfc3339_opts(chrono::SecondsFormat::Millis, false),  "2000-01-01T00:00:00.000+00:00");
        assert_eq!(parse("2000-01-01T00:00:00-00:00").unwrap().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),  "2000-01-01T00:00:00.000Z");
        assert_eq!(parse("2000-01-01T00:00:00+00:00").unwrap().to_rfc3339_opts(chrono::SecondsFormat::Millis, false),  "2000-01-01T00:00:00.000+00:00");
        assert_eq!(parse("2000-01-01T00:00:00+00:00").unwrap().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),  "2000-01-01T00:00:00.000Z");

        assert_eq!(parse("Sat, 01 Jan 2000 00:00:00 GMT").unwrap().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),  "2000-01-01T00:00:00.000Z");
        assert_eq!(parse("Sat, 01 Jun 2000 00:00:00 GMT").unwrap().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),  "2000-06-01T00:00:00.000Z");

        assert_eq!(parse("Sat, 01 Jan 2000 00:00:00 -0000").unwrap().offset().utc_minus_local(), 0 );
        assert_eq!(parse("Sat, 01 Jan 2000 00:00:00 +0000").unwrap().offset().utc_minus_local(), 0 );
    }

    #[test]
    fn test_parse_now_nil() {
        let now_local: DateTime<FixedOffset> = local_time!(2000,1,1,0,0,0,"+11:00");
        assert_eq!(parse("2000-01-01T00:00:00+11:00").unwrap(), now_local);
    }

    #[test]
    fn test_parse_offset_hour_minute_second() {
        let now_local: DateTime<FixedOffset> = local_time!(-214748);
        assert_eq!(parse("1200-02-15 BC 14:13:20-00").unwrap(), now_local);
        assert_eq!(parse("1200-02-15 BC 14:13:20-00:00").unwrap(), now_local);
        assert_eq!(parse("1200-02-15 BC 14:13:20-00:00:00").unwrap(), now_local);
    }

    #[test]
    fn test_parse_cc_expiry() {
        assert!(parse("26-05").is_ok());
    }

}