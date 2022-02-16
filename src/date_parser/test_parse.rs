use crate::date_parser::{date_parse, parse::DateTime};

#[test]
pub fn test_ctime_asctime() {
    //# ctime(3), asctime(3)
    assert_eq!(date_parse("Sat Aug 28 02:55:50 1999",false), DateTime{year: Some(1999),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(55), sec: Some(50), zone: None, offset: None, wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:55:50 02",false), DateTime{year: Some(2),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(55), sec: Some(50), zone: None, offset: None, wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:55:50 02",true), DateTime{year: Some(2002),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(55), sec: Some(50), zone: None, offset: None, wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:55:50 0002",false), DateTime{year: Some(2),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(55), sec: Some(50), zone: None, offset: None, wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:55:50 0002",true), DateTime{year: Some(2),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(55), sec: Some(50), zone: None, offset: None, wday: Some(6), ..Default::default()});

    // # date(1)
    assert_eq!(date_parse("Sat Aug 28 02:29:34 JST 1999",false), DateTime{year: Some(1999),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("JST".to_owned()), offset: Some(9*3600), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 MET DST 1999",false), DateTime{year: Some(1999),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("MET DST".to_owned()), offset: Some(2*3600), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 AMT 1999",false), DateTime{year: Some(1999),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("AMT".to_owned()),offset: None, wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 PMT 1999",false), DateTime{year: Some(1999),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("PMT".to_owned()),offset: None, wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 PMT -1999",false), DateTime{year: Some(-1999),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("PMT".to_owned()),offset: None, wday: Some(6), ..Default::default()});

    assert_eq!(date_parse("Sat Aug 28 02:29:34 JST 02",false), DateTime{year: Some(2),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("JST".to_owned()), offset: Some(9*3600), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 JST 02",true), DateTime{year: Some(2002),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("JST".to_owned()), offset: Some(9*3600), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 JST 0002",false), DateTime{year: Some(2),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("JST".to_owned()), offset: Some(9*3600), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 JST 0002",true), DateTime{year: Some(2),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("JST".to_owned()), offset: Some(9*3600), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 AEST 0002",true), DateTime{year: Some(2),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("AEST".to_owned()), offset: Some(10*3600), wday: Some(6), ..Default::default()});

    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT+09 0002",false), DateTime{year: Some(2),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("GMT+09".to_owned()), offset: Some(9*3600), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT+0900 0002",false), DateTime{year: Some(2),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("GMT+0900".to_owned()), offset: Some(9*3600), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT+09:00 0002",false), DateTime{year: Some(2),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("GMT+09:00".to_owned()), offset: Some(9*3600), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT-09 0002",false), DateTime{year: Some(2),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("GMT-09".to_owned()),offset: Some(-9*3600), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT-0900 0002",false), DateTime{year: Some(2),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("GMT-0900".to_owned()),offset: Some(-9*3600), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT-09:00 0002",false), DateTime{year: Some(2),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("GMT-09:00".to_owned()),offset: Some(-9*3600), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT-090102 0002",false), DateTime{year: Some(2),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("GMT-090102".to_owned()),offset: Some(-9*3600-60-2), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT-09:01:02 0002",false), DateTime{year: Some(2),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("GMT-09:01:02".to_owned()),offset: Some(-9*3600-60-2), wday: Some(6), ..Default::default()});

    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT Standard Time 2000",false), DateTime{year: Some(2000),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("GMT Standard Time".to_owned()), offset: Some(0), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 Mountain Standard Time 2000",false), DateTime{year: Some(2000),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("Mountain Standard Time".to_owned()),offset: Some(-7*3600), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 Mountain Daylight Time 2000",false), DateTime{year: Some(2000),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("Mountain Daylight Time".to_owned()),offset: Some(-6*3600), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 Mexico Standard Time 2000",false), DateTime{year: Some(2000),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("Mexico Standard Time".to_owned()),offset: Some(-6*3600), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 E. Australia Standard Time 2000",false), DateTime{year: Some(2000),mon: Some(8), mday:Some(28),hour: Some(2),min: Some(29), sec: Some(34), zone: Some("E. Australia Standard Time".to_owned()), offset: Some(10*3600), wday: Some(6), ..Default::default()});
}

#[test]
pub fn test_iso8601() {
    assert_eq!(date_parse("1999-05-23 23:55:21",false),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("1999-05-23 23:55:21+0900",false),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("+0900".to_owned()), offset: Some(9*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("1999-05-23 23:55:21-0900",false),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-0900".to_owned()), offset: Some(-9*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("1999-05-23 23:55:21+09:00",false),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("+09:00".to_owned()), offset: Some(9*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("1999-05-23T23:55:21-09:00",false),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-09:00".to_owned()), offset: Some(-9*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("1999-05-23 23:55:21Z",false),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("Z".to_owned()), offset: Some(0), wday: None, ..Default::default()});
    assert_eq!(date_parse("1999-05-23T23:55:21Z",false),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("Z".to_owned()), offset: Some(0), wday: None, ..Default::default()});
    assert_eq!(date_parse("-1999-05-23T23:55:21Z",false),DateTime{year: Some(-1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("Z".to_owned()), offset: Some(0), wday: None, ..Default::default()});
    assert_eq!(date_parse("-1999-05-23T23:55:21Z",true),DateTime{year: Some(-1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("Z".to_owned()), offset: Some(0), wday: None, ..Default::default()});
    assert_eq!(date_parse("19990523T23:55:21Z",false),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("Z".to_owned()), offset: Some(0), wday: None, ..Default::default()});

    assert_eq!(date_parse("+011985-04-12",false),DateTime{year: Some(11985),mon: Some(4), mday:Some(12), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("+011985-04-12T10:15:30",false),DateTime{year: Some(11985),mon: Some(4), mday:Some(12), hour: Some(10), min: Some(15), sec: Some(30), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("-011985-04-12",false),DateTime{year: Some(-11985),mon: Some(4), mday:Some(12), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("-011985-04-12T10:15:30",false),DateTime{year: Some(-11985),mon: Some(4), mday:Some(12), hour: Some(10), min: Some(15), sec: Some(30), zone: None, offset: None, wday: None, ..Default::default()});

    assert_eq!(date_parse("02-04-12",false),DateTime{year: Some(2),mon: Some(4), mday:Some(12), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("02-04-12",true),DateTime{year: Some(2002),mon: Some(4), mday:Some(12), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("0002-04-12",false),DateTime{year: Some(2),mon: Some(4), mday:Some(12), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("0002-04-12",true),DateTime{year: Some(2),mon: Some(4), mday:Some(12), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

    assert_eq!(date_parse("19990523",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("-19990523",true),DateTime{year: Some(-1999),mon: Some(5), mday:Some(23), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("990523",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("0523",false),DateTime{year: None,mon: Some(5), mday:Some(23), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("23",false),DateTime{year: None,mon: None, mday:Some(23), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

    assert_eq!(date_parse("19990523 235521",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("990523 235521",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("0523 2355",false),DateTime{year: None,mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("23 2355",false),DateTime{year: None,mon: None, mday:Some(23), hour: Some(23), min: Some(55), sec: None, zone: None, offset: None, wday: None, ..Default::default()});

    assert_eq!(date_parse("19990523T235521",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("990523T235521",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("19990523T235521.99",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: None, offset: None, wday: None, sec_fraction: Some(0.99), ..Default::default()});
    assert_eq!(date_parse("990523T235521.99",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: None, offset: None, wday: None, sec_fraction: Some(0.99), ..Default::default()});
    assert_eq!(date_parse("0523T2355",false),DateTime{year: None,mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: None, zone: None, offset: None, wday: None, ..Default::default()});

    assert_eq!(date_parse("19990523T235521+0900",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("+0900".to_owned()), offset: Some(9*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("990523T235521-0900",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-0900".to_owned()), offset: Some(-9*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("19990523T235521.99+0900",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("+0900".to_owned()), offset: Some(9*3600), wday: None, sec_fraction: Some(0.99), ..Default::default()});
    assert_eq!(date_parse("990523T235521.99-0900",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-0900".to_owned()), offset: Some(-9*3600), wday: None, sec_fraction: Some(0.99), ..Default::default()});
    assert_eq!(date_parse("0523T2355Z",false),DateTime{year: None,mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: None, zone: Some("Z".to_owned()), offset: Some(0), wday: None, ..Default::default()});

    assert_eq!(date_parse("19990523235521.123456+0900",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("+0900".to_owned()), offset: Some(9*3600), wday: None, sec_fraction: Some(0.123456), ..Default::default()});
    assert_eq!(date_parse("19990523235521.123456-0900",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-0900".to_owned()), offset: Some(-9*3600), wday: None, sec_fraction: Some(0.123456), ..Default::default()});
    assert_eq!(date_parse("19990523235521,123456+0900",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("+0900".to_owned()), offset: Some(9*3600), wday: None, sec_fraction: Some(0.123456), ..Default::default()});
    assert_eq!(date_parse("19990523235521,123456-0900",true),DateTime{year: Some(1999),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-0900".to_owned()), offset: Some(-9*3600), wday: None, sec_fraction: Some(0.123456), ..Default::default()});

    assert_eq!(date_parse("990523235521,123456-0900",false),DateTime{year: Some(99),mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-0900".to_owned()), offset: Some(-9*3600), wday: None, sec_fraction: Some(0.123456), ..Default::default()});
    assert_eq!(date_parse("0523235521,123456-0900",false),DateTime{year: None,mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-0900".to_owned()), offset: Some(-9*3600), wday: None, sec_fraction: Some(0.123456), ..Default::default()});
    assert_eq!(date_parse("23235521,123456-0900",false),DateTime{year: None,mon: None, mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-0900".to_owned()), offset: Some(-9*3600), wday: None, sec_fraction: Some(0.123456), ..Default::default()});
    assert_eq!(date_parse("235521,123456-0900",false),DateTime{year: None,mon: None, mday:None, hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-0900".to_owned()), offset: Some(-9*3600), wday: None, sec_fraction: Some(0.123456), ..Default::default()});
    assert_eq!(date_parse("5521,123456-0900",false),DateTime{year: None,mon: None, mday:None, hour: None, min: Some(55), sec: Some(21), zone: Some("-0900".to_owned()), offset: Some(-9*3600), wday: None, sec_fraction: Some(0.123456), ..Default::default()});
    assert_eq!(date_parse("21,123456-0900",false),DateTime{year: None,mon: None, mday:None, hour: None, min: None, sec: Some(21), zone: Some("-0900".to_owned()), offset: Some(-9*3600), wday: None, sec_fraction: Some(0.123456), ..Default::default()});

    assert_eq!(date_parse("3235521,123456-0900",false),DateTime{year: None,mon: None, mday:Some(3), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-0900".to_owned()), offset: Some(-9*3600), wday: None, sec_fraction: Some(0.123456), ..Default::default()});
    assert_eq!(date_parse("35521,123456-0900",false),DateTime{year: None,mon: None, mday:None, hour: Some(3), min: Some(55), sec: Some(21), zone: Some("-0900".to_owned()), offset: Some(-9*3600), wday: None, sec_fraction: Some(0.123456), ..Default::default()});
    assert_eq!(date_parse("521,123456-0900",false),DateTime{year: None,mon: None, mday:None, hour: None, min: Some(5), sec: Some(21), zone: Some("-0900".to_owned()), offset: Some(-9*3600), wday: None, sec_fraction: Some(0.123456), ..Default::default()});
}

#[test]
pub fn test_reversed_iso8601() {
    // # reversed iso 8601 (?)
    assert_eq!(date_parse("23-05-1999",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("23-05-1999 23:55:21",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("23-05--1999 23:55:21",false), DateTime{year: Some(-1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("23-05-'99",false), DateTime{year: Some(99), mon: Some(5), mday:Some(23), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("23-05-'99",true), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}

#[test]
pub fn test_broken_iso8601() {
    //# broken iso 8601 (?)
    assert_eq!(date_parse("19990523T23:55:21Z",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("Z".to_owned()), offset: Some(0), wday: None, ..Default::default()});
    assert_eq!(date_parse("19990523235521.1234-100",true), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-100".to_owned()), offset: Some(-3600), wday: None, sec_fraction: Some(0.1234), ..Default::default()});
    assert_eq!(date_parse("19990523235521.1234-10",true), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-10".to_owned()), offset: Some(-10*3600), wday: None, sec_fraction: Some(0.1234), ..Default::default()});
}

#[test]
pub fn test_jis_x0301() {
    //# part of jis x0301
    assert_eq!(date_parse("M11.05.23",false), DateTime{year: Some(1878), mon: Some(5), mday:Some(23), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("T11.05.23 23:55:21+0900",false), DateTime{year: Some(1922), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("+0900".to_owned()), offset: Some(9*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("S11.05.23 23:55:21-0900",false), DateTime{year: Some(1936), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-0900".to_owned()), offset: Some(-9*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("S40.05.23 23:55:21+09:00",false), DateTime{year: Some(1965), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("+09:00".to_owned()), offset: Some(9*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("S40.05.23T23:55:21-09:00",false), DateTime{year: Some(1965), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-09:00".to_owned()), offset: Some(-9*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("H11.05.23 23:55:21Z",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("Z".to_owned()), offset: Some(0), wday: None, ..Default::default()});
    assert_eq!(date_parse("H11.05.23T23:55:21Z",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("Z".to_owned()), offset: Some(0), wday: None, ..Default::default()});
    assert_eq!(date_parse("H31.04.30 23:55:21Z",false), DateTime{year: Some(2019), mon: Some(4), mday:Some(30), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("Z".to_owned()), offset: Some(0), wday: None, ..Default::default()});
    assert_eq!(date_parse("H31.04.30T23:55:21Z",false), DateTime{year: Some(2019), mon: Some(4), mday:Some(30), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("Z".to_owned()), offset: Some(0), wday: None, ..Default::default()});
}

#[test]
pub fn test_ofx_date() {
    //# ofx date
    assert_eq!(date_parse("19990523235521",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("19990523235521.123",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: None, offset: None, wday: None, sec_fraction: Some(0.123), ..Default::default()});
    assert_eq!(date_parse("19990523235521.123[-9]",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-9".to_owned()), offset: Some(-9*3600), wday: None, sec_fraction: Some(0.123), ..Default::default()});
    assert_eq!(date_parse("19990523235521.123[+9]",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("+9".to_owned()), offset: Some(9*3600), wday: None, sec_fraction: Some(0.123), ..Default::default()});
    assert_eq!(date_parse("19990523235521.123[9]",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("9".to_owned()), offset: Some(9*3600), wday: None, sec_fraction: Some(0.123), ..Default::default()});
    assert_eq!(date_parse("19990523235521.123[-9.50]",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-9.50".to_owned()), offset: Some(-(9*3600+30*60)), wday: None, sec_fraction: Some(0.123), ..Default::default()});
    assert_eq!(date_parse("19990523235521.123[+9.50]",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("+9.50".to_owned()), offset: Some(9*3600+30*60), wday: None, sec_fraction: Some(0.123), ..Default::default()});
    assert_eq!(date_parse("19990523235521.123[-5:EST]",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("EST".to_owned()), offset: Some(-5*3600), wday: None, sec_fraction: Some(0.123), ..Default::default()});
    assert_eq!(date_parse("19990523235521.123[+9:JST]",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("JST".to_owned()), offset: Some(9*3600), wday: None, sec_fraction: Some(0.123), ..Default::default()});
    assert_eq!(date_parse("19990523235521.123[+12:XXX YYY ZZZ]",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(23), hour: Some(23), min: Some(55), sec: Some(21), zone: Some("XXX YYY ZZZ".to_owned()), offset: Some(12*3600), wday: None, sec_fraction: Some(0.123), ..Default::default()});
    assert_eq!(date_parse("235521.123",false), DateTime{year: None, mon: None, mday:None, hour: Some(23), min: Some(55), sec: Some(21), zone: None, offset: None, wday: None, sec_fraction: Some(0.123), ..Default::default()});
    assert_eq!(date_parse("235521.123[-9]",false), DateTime{year: None, mon: None, mday:None, hour: Some(23), min: Some(55), sec: Some(21), zone: Some("-9".to_owned()), offset: Some(-9*3600), wday: None, sec_fraction: Some(0.123), ..Default::default()});
    assert_eq!(date_parse("235521.123[+9]",false), DateTime{year: None, mon: None, mday:None, hour: Some(23), min: Some(55), sec: Some(21), zone: Some("+9".to_owned()), offset: Some(9*3600), wday: None, sec_fraction: Some(0.123), ..Default::default()});
    assert_eq!(date_parse("235521.123[-5:EST]",false), DateTime{year: None, mon: None, mday:None, hour: Some(23), min: Some(55), sec: Some(21), zone: Some("EST".to_owned()), offset: Some(-5*3600), wday: None, sec_fraction: Some(0.123), ..Default::default()});
    assert_eq!(date_parse("235521.123[+9:JST]",false), DateTime{year: None, mon: None, mday:None, hour: Some(23), min: Some(55), sec: Some(21), zone: Some("JST".to_owned()), offset: Some(9*3600), wday: None, sec_fraction: Some(0.123), ..Default::default()});
}

#[test]
pub fn test_rfc_2822() {

    //# rfc 2822
    assert_eq!(date_parse("Sun, 22 Aug 1999 00:45:29 -0400",false), DateTime{year: Some(1999), mon: Some(8), mday:Some(22), hour: Some(0), min: Some(45), sec: Some(29), zone: Some("-0400".to_owned()), offset: Some(-4*3600), wday: Some(0), ..Default::default()});
    assert_eq!(date_parse("Sun, 22 Aug 1999 00:45:29 -9959",false), DateTime{year: Some(1999), mon: Some(8), mday:Some(22), hour: Some(0), min: Some(45), sec: Some(29), zone: Some("-9959".to_owned()), offset: Some(-(99*3600+59*60)), wday: Some(0), ..Default::default()});
    assert_eq!(date_parse("Sun, 22 Aug 1999 00:45:29 +9959",false), DateTime{year: Some(1999), mon: Some(8), mday:Some(22), hour: Some(0), min: Some(45), sec: Some(29), zone: Some("+9959".to_owned()), offset: Some(99*3600+59*60), wday: Some(0), ..Default::default()});
    assert_eq!(date_parse("Sun, 22 Aug 05 00:45:29 -0400",true), DateTime{year: Some(2005), mon: Some(8), mday:Some(22), hour: Some(0), min: Some(45), sec: Some(29), zone: Some("-0400".to_owned()), offset: Some(-4*3600), wday: Some(0), ..Default::default()});
    assert_eq!(date_parse("Sun, 22 Aug 49 00:45:29 -0400",true), DateTime{year: Some(2049), mon: Some(8), mday:Some(22), hour: Some(0), min: Some(45), sec: Some(29), zone: Some("-0400".to_owned()), offset: Some(-4*3600), wday: Some(0), ..Default::default()});
    assert_eq!(date_parse("Sun, 22 Aug 1999 00:45:29 GMT",false), DateTime{year: Some(1999), mon: Some(8), mday:Some(22), hour: Some(0), min: Some(45), sec: Some(29), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(0), ..Default::default()});
    assert_eq!(date_parse("Sun,\022\r\nAug\r\n1999\r\n00:45:29\r\nGMT",false), DateTime{year: Some(1999), mon: Some(8), mday:Some(22), hour: Some(0), min: Some(45), sec: Some(29), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(0), ..Default::default()});
    assert_eq!(date_parse("Sun, 22 Aug 1999 00:45 GMT",false), DateTime{year: Some(1999), mon: Some(8), mday:Some(22), hour: Some(0), min: Some(45), sec: None, zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(0), ..Default::default()});
    assert_eq!(date_parse("Sun, 22 Aug -1999 00:45 GMT",false), DateTime{year: Some(-1999), mon: Some(8), mday:Some(22), hour: Some(0), min: Some(45), sec: None, zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(0), ..Default::default()});
    assert_eq!(date_parse("Sun, 22 Aug 99 00:45:29 UT",true), DateTime{year: Some(1999), mon: Some(8), mday:Some(22), hour: Some(0), min: Some(45), sec: Some(29), zone: Some("UT".to_owned()), offset: Some(0), wday: Some(0), ..Default::default()});
    assert_eq!(date_parse("Sun, 22 Aug 0099 00:45:29 UT",true), DateTime{year: Some(99), mon: Some(8), mday:Some(22), hour: Some(0), min: Some(45), sec: Some(29), zone: Some("UT".to_owned()), offset: Some(0), wday: Some(0), ..Default::default()});
}

#[test]
pub fn test_rfc_850() {
    //# rfc 850, obsoleted by rfc 1036
    assert_eq!(date_parse("Tuesday, 02-Mar-99 11:20:32 GMT",true), DateTime{year: Some(1999), mon: Some(3), mday:Some(2), hour: Some(11), min: Some(20), sec: Some(32), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(2), ..Default::default()});
}

#[test]
pub fn test_w3c_xforms() {
    //# W3C Working Draft - XForms - 4.8 Time
    assert_eq!(date_parse("2000-01-31 13:20:00-5",false), DateTime{year: Some(2000), mon: Some(1), mday:Some(31), hour: Some(13), min: Some(20), sec: Some(0), zone: Some("-5".to_owned()), offset: Some(-5*3600), wday: None, ..Default::default()});
}

#[test]
pub fn test_tz_offsets_with_separators() {
    //# [-+]\d+.\d+
    assert_eq!(date_parse("2000-01-31 13:20:00-5.5",false), DateTime{year: Some(2000), mon: Some(1), mday:Some(31), hour: Some(13), min: Some(20), sec: Some(0), zone: Some("-5.5".to_owned()), offset: Some(-5*3600-30*60), wday: None, ..Default::default()});
    assert_eq!(date_parse("2000-01-31 13:20:00-5,5",false), DateTime{year: Some(2000), mon: Some(1), mday:Some(31), hour: Some(13), min: Some(20), sec: Some(0), zone: Some("-5,5".to_owned()), offset: Some(-5*3600-30*60), wday: None, ..Default::default()});
    assert_eq!(date_parse("2000-01-31 13:20:00+3.5",false), DateTime{year: Some(2000), mon: Some(1), mday:Some(31), hour: Some(13), min: Some(20), sec: Some(0), zone: Some("+3.5".to_owned()), offset: Some(3*3600+30*60), wday: None, ..Default::default()});
    assert_eq!(date_parse("2000-01-31 13:20:00+3,5",false), DateTime{year: Some(2000), mon: Some(1), mday:Some(31), hour: Some(13), min: Some(20), sec: Some(0), zone: Some("+3,5".to_owned()), offset: Some(3*3600+30*60), wday: None, ..Default::default()});
}

#[test]
pub fn test_mil() {
    //# mil
    assert_eq!(date_parse("2000-01-31 13:20:00 Z",false), DateTime{year: Some(2000), mon: Some(1), mday:Some(31), hour: Some(13), min: Some(20), sec: Some(0), zone: Some("Z".to_owned()), offset: Some(0), wday: None, ..Default::default()});
    assert_eq!(date_parse("2000-01-31 13:20:00 H",false), DateTime{year: Some(2000), mon: Some(1), mday:Some(31), hour: Some(13), min: Some(20), sec: Some(0), zone: Some("H".to_owned()), offset: Some(8*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("2000-01-31 13:20:00 M",false), DateTime{year: Some(2000), mon: Some(1), mday:Some(31), hour: Some(13), min: Some(20), sec: Some(0), zone: Some("M".to_owned()), offset: Some(12*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("2000-01-31 13:20 M",false), DateTime{year: Some(2000), mon: Some(1), mday:Some(31), hour: Some(13), min: Some(20), sec: None, zone: Some("M".to_owned()), offset: Some(12*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("2000-01-31 13:20:00 S",false), DateTime{year: Some(2000), mon: Some(1), mday:Some(31), hour: Some(13), min: Some(20), sec: Some(0), zone: Some("S".to_owned()), offset: Some(-6*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("2000-01-31 13:20:00 A",false), DateTime{year: Some(2000), mon: Some(1), mday:Some(31), hour: Some(13), min: Some(20), sec: Some(0), zone: Some("A".to_owned()), offset: Some(3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("2000-01-31 13:20:00 P",false), DateTime{year: Some(2000), mon: Some(1), mday:Some(31), hour: Some(13), min: Some(20), sec: Some(0), zone: Some("P".to_owned()), offset: Some(-3*3600), wday: None, ..Default::default()});
}

#[test]
pub fn test_dot() {

    //# dot
    assert_eq!(date_parse("1999.5.2",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("1999.05.02",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("-1999.05.02",false), DateTime{year: Some(-1999), mon: Some(5), mday:Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

    assert_eq!(date_parse("0099.5.2",false), DateTime{year: Some(99), mon: Some(5), mday:Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("0099.5.2",true), DateTime{year: Some(99), mon: Some(5), mday:Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

    assert_eq!(date_parse("'99.5.2",false), DateTime{year: Some(99), mon: Some(5), mday:Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("'99.5.2",true), DateTime{year: Some(1999), mon: Some(5), mday:Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}

#[test]
pub fn test_reverse_dot() {

    //# reversed dot
    assert_eq!(date_parse("2.5.1999",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("02.05.1999",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("02.05.-1999",false), DateTime{year: Some(-1999), mon: Some(5), mday:Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

    assert_eq!(date_parse("2.5.0099",false), DateTime{year: Some(99), mon: Some(5), mday:Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("2.5.0099",true), DateTime{year: Some(99), mon: Some(5), mday:Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

    assert_eq!(date_parse("2.5.'99",false), DateTime{year: Some(99), mon: Some(5), mday:Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("2.5.'99",true), DateTime{year: Some(1999), mon: Some(5), mday:Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}

#[test]
pub fn test_vms() {

    //# vms
    assert_eq!(date_parse("08-DEC-1988",false), DateTime{year: Some(1988), mon: Some(12), mday:Some(8), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("31-JAN-1999",false), DateTime{year: Some(1999), mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("31-JAN--1999",false), DateTime{year: Some(-1999), mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

    assert_eq!(date_parse("08-DEC-88",false), DateTime{year: Some(88), mon: Some(12), mday:Some(8), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("08-DEC-88",true), DateTime{year: Some(1988), mon: Some(12), mday:Some(8), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("08-DEC-0088",false), DateTime{year: Some(88), mon: Some(12), mday:Some(8), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("08-DEC-0088",true), DateTime{year: Some(88), mon: Some(12), mday:Some(8), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

}

#[test]
pub fn test_swapped_vms() {
    //# swapped vms
    assert_eq!(date_parse("DEC-08-1988",false), DateTime{year: Some(1988), mon: Some(12), mday:Some(8), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("JAN-31-1999",false), DateTime{year: Some(1999), mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("JAN-31--1999",false), DateTime{year: Some(-1999), mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("JAN-1999",false), DateTime{year: Some(1999), mon: Some(1), mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("JAN--1999",false), DateTime{year: Some(-1999), mon: Some(1), mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}

#[test]
pub fn test_reversed_vms() {

    //# reversed vms
    assert_eq!(date_parse("1988-DEC-08",false), DateTime{year: Some(1988), mon: Some(12), mday:Some(8), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("1999-JAN-31",false), DateTime{year: Some(1999), mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("-1999-JAN-31",false), DateTime{year: Some(-1999), mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

    assert_eq!(date_parse("0088-DEC-08",false), DateTime{year: Some(88), mon: Some(12), mday:Some(8), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("0088-DEC-08",true), DateTime{year: Some(88), mon: Some(12), mday:Some(8), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

    assert_eq!(date_parse("'88/12/8",false), DateTime{year: Some(88), mon: Some(12), mday:Some(8), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("'88/12/8",true), DateTime{year: Some(1988), mon: Some(12), mday:Some(8), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}

#[test]
pub fn test_non_spaced_eu() {

    //# non-spaced eu
    assert_eq!(date_parse("08/dec/1988",false), DateTime{year: Some(1988), mon: Some(12), mday:Some(8), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("31/jan/1999",false), DateTime{year: Some(1999), mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("31/jan/-1999",false), DateTime{year: Some(-1999), mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("08.dec.1988",false), DateTime{year: Some(1988), mon: Some(12), mday:Some(8), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("31.jan.1999",false), DateTime{year: Some(1999), mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("31.jan.-1999",false), DateTime{year: Some(-1999), mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}

#[test]
pub fn test_non_spaced_us() {

    //# non-spaced us
    assert_eq!(date_parse("dec/08/1988",false), DateTime{year: Some(1988), mon: Some(12), mday:Some(8), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("jan/31/1999",false), DateTime{year: Some(1999), mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("jan/31/-1999",false), DateTime{year: Some(-1999), mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("jan/31",false), DateTime{year: None, mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("jan/1988",false), DateTime{year: Some(1988), mon: Some(1), mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("dec.08.1988",false), DateTime{year: Some(1988), mon: Some(12), mday:Some(8), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("jan.31.1999",false), DateTime{year: Some(1999), mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("jan.31.-1999",false), DateTime{year: Some(-1999), mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("jan.31",false), DateTime{year: None, mon: Some(1), mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("jan.1988",false), DateTime{year: Some(1988), mon: Some(1), mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}

#[test]
pub fn test_month_and_day_of_month() {

    //# month and day of month
    assert_eq!(date_parse("Jan 1",false), DateTime{year: None, mon: Some(1), mday:Some(1), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("Jul 11",false), DateTime{year: None, mon: Some(7), mday:Some(11), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("July 11",false), DateTime{year: None, mon: Some(7), mday:Some(11), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("Sept 23",false), DateTime{year: None, mon: Some(9), mday:Some(23), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("Sep. 23",false), DateTime{year: None, mon: Some(9), mday:Some(23), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("Sept. 23",false), DateTime{year: None, mon: Some(9), mday:Some(23), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("September 23",false), DateTime{year: None, mon: Some(9), mday:Some(23), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("October 1st",false), DateTime{year: None, mon: Some(10), mday:Some(1), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("October 23rd",false), DateTime{year: None, mon: Some(10), mday:Some(23), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("October 25th 1999",false), DateTime{year: Some(1999), mon: Some(10), mday:Some(25), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("October 25th -1999",false), DateTime{year: Some(-1999), mon: Some(10), mday:Some(25), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("october 25th 1999",false), DateTime{year: Some(1999), mon: Some(10), mday:Some(25), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("OCTOBER 25th 1999",false), DateTime{year: Some(1999), mon: Some(10), mday:Some(25), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("oCtoBer 25th 1999",false), DateTime{year: Some(1999), mon: Some(10), mday:Some(25), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("aSep 23",false), DateTime{year: None, mon: None, mday:Some(23), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}

#[test]
pub fn test_month_and_year() {

    //# month and year
    assert_eq!(date_parse("Sept 1990",false), DateTime{year: Some(1990), mon: Some(9), mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("Sept '90",false), DateTime{year: Some(90), mon: Some(9), mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("Sept '90",true), DateTime{year: Some(1990), mon: Some(9), mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("1990/09",false), DateTime{year: Some(1990), mon: Some(9), mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("09/1990",false), DateTime{year: Some(1990), mon: Some(9), mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("aSep '90",false), DateTime{year: Some(90), mon: None, mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}

#[test]
pub fn test_year() {

    //# year
    assert_eq!(date_parse("'90",false), DateTime{year: Some(90), mon: None, mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("'90",true), DateTime{year: Some(1990), mon: None, mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}

#[test]
pub fn test_month() {

    //# month
    assert_eq!(date_parse("Jun",false), DateTime{year: None, mon: Some(6), mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("June",false), DateTime{year: None, mon: Some(6), mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("Sep",false), DateTime{year: None, mon: Some(9), mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("Sept",false), DateTime{year: None, mon: Some(9), mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("September",false), DateTime{year: None, mon: Some(9), mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("aSep",false), DateTime{year: None, mon: None, mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}

#[test]
pub fn test_day_of_month() {

    //# day of month
    assert_eq!(date_parse("1st",false), DateTime{year: None, mon: None, mday:Some(1), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("2nd",false), DateTime{year: None, mon: None, mday:Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("3rd",false), DateTime{year: None, mon: None, mday:Some(3), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("4th",false), DateTime{year: None, mon: None, mday:Some(4), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("29th",false), DateTime{year: None, mon: None, mday:Some(29), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("31st",false), DateTime{year: None, mon: None, mday:Some(31), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("1sta",false), DateTime{year: None, mon: None, mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}


#[test]
pub fn test_bc_era() {
    //# era
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT BCE 2000",false), DateTime{year: Some(-1999), mon: Some(8), mday:Some(28), hour: Some(2), min: Some(29), sec: Some(34), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(6), bc: true, ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT B.C.E. 2000",false), DateTime{year: Some(-1999), mon: Some(8), mday:Some(28), hour: Some(2), min: Some(29), sec: Some(34), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(6), bc: true, ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT BC 2000",false), DateTime{year: Some(-1999), mon: Some(8), mday:Some(28), hour: Some(2), min: Some(29), sec: Some(34), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(6), bc: true, ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT B.C. 2000",false), DateTime{year: Some(-1999), mon: Some(8), mday:Some(28), hour: Some(2), min: Some(29), sec: Some(34), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(6), bc: true, ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT 2000 BC",false), DateTime{year: Some(-1999), mon: Some(8), mday:Some(28), hour: Some(2), min: Some(29), sec: Some(34), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(6), bc: true, ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT 2000 BCE",false), DateTime{year: Some(-1999), mon: Some(8), mday:Some(28), hour: Some(2), min: Some(29), sec: Some(34), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(6), bc: true, ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT 2000 B.C.",false), DateTime{year: Some(-1999), mon: Some(8), mday:Some(28), hour: Some(2), min: Some(29), sec: Some(34), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(6), bc: true, ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT 2000 B.C.E.",false), DateTime{year: Some(-1999), mon: Some(8), mday:Some(28), hour: Some(2), min: Some(29), sec: Some(34), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(6), bc: true, ..Default::default()});
}

#[test]
pub fn test_ad_era() {
    //# era
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT CE 2000",false), DateTime{year: Some(2000), mon: Some(8), mday:Some(28), hour: Some(2), min: Some(29), sec: Some(34), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT C.E. 2000",false), DateTime{year: Some(2000), mon: Some(8), mday:Some(28), hour: Some(2), min: Some(29), sec: Some(34), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT AD 2000",false), DateTime{year: Some(2000), mon: Some(8), mday:Some(28), hour: Some(2), min: Some(29), sec: Some(34), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("Sat Aug 28 02:29:34 GMT A.D. 2000",false), DateTime{year: Some(2000), mon: Some(8), mday:Some(28), hour: Some(2), min: Some(29), sec: Some(34), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(6), ..Default::default()});
}

#[test]
pub fn test_collection() {

    //# collection
    assert_eq!(date_parse("Tuesday, May 18, 1999 Published at 13:36 GMT 14:36 UK",false), DateTime{year: Some(1999), mon: Some(5), mday:Some(18), hour: Some(13), min: Some(36), sec: None, zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(2), ..Default::default()});
    assert_eq!(date_parse("July 20, 2000 Web posted at: 3:37 p.m. EDT (1937 GMT)",false), DateTime{year: Some(2000), mon: Some(7), mday:Some(20), hour: Some(15), min: Some(37), sec: None, zone: Some("EDT".to_owned()), offset: Some(-4*3600),wday: None, ..Default::default()});
    assert_eq!(date_parse("12:54 p.m. EDT, September 11, 2006",false), DateTime{year: Some(2006), mon: Some(9), mday:Some(11), hour: Some(12), min: Some(54), sec: None, zone: Some("EDT".to_owned()), offset: Some(-4*3600),  wday: None, ..Default::default()});
    assert_eq!(date_parse("February 04, 2001 at 10:59 AM PST",false), DateTime{year: Some(2001), mon: Some(2), mday:Some(4), hour: Some(10), min: Some(59), sec: None, zone: Some("PST".to_owned()), offset: Some(-8*3600),  wday: None, ..Default::default()});
    assert_eq!(date_parse("Monday May 08, @01:55PM",false), DateTime{year: None, mon: Some(5), mday:Some(8), hour: Some(13), min: Some(55), sec: None, zone: None, offset: None, wday: Some(1), ..Default::default()});
    assert_eq!(date_parse("06.June 2005",false), DateTime{year: Some(2005), mon: Some(6), mday:Some(6), hour: None, min: None, sec: None, zone: None, offset: None,  wday: None, ..Default::default()});
}

#[test]
pub fn test_etc() {
    //# etc.
    assert_eq!(date_parse("8:00 pm lt",false), DateTime{year: None, mon: None, mday:None, hour: Some(20), min: Some(0), sec: None, zone: Some("lt".to_owned()), offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("4:00 AM, Jan. 12, 1990",false), DateTime{year: Some(1990), mon: Some(1), mday:Some(12), hour: Some(4), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("Jan. 12 4:00 AM 1990",false), DateTime{year: Some(1990), mon: Some(1), mday:Some(12), hour: Some(4), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("1990-01-12 04:00:00+00",false), DateTime{year: Some(1990), mon: Some(1), mday:Some(12), hour: Some(4), min: Some(0), sec: Some(0), zone: Some("+00".to_owned()), offset: Some(0), wday: None, ..Default::default()});
    assert_eq!(date_parse("1990-01-11 20:00:00-08",false), DateTime{year: Some(1990), mon: Some(1), mday:Some(11), hour: Some(20), min: Some(0), sec: Some(0), zone: Some("-08".to_owned()), offset: Some(-8*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("1990/01/12 04:00:00",false), DateTime{year: Some(1990), mon: Some(1), mday:Some(12), hour: Some(4), min: Some(0), sec: Some(0), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("Thu Jan 11 20:00:00 PST 1990",false), DateTime{year: Some(1990), mon: Some(1), mday:Some(11), hour: Some(20), min: Some(0), sec: Some(0), zone: Some("PST".to_owned()), offset: Some(-8*3600), wday: Some(4), ..Default::default()});
    assert_eq!(date_parse("Fri Jan 12 04:00:00 GMT 1990",false), DateTime{year: Some(1990), mon: Some(1), mday:Some(12), hour: Some(4), min: Some(0), sec: Some(0), zone: Some("GMT".to_owned()), offset: Some(0), wday: Some(5), ..Default::default()});
    assert_eq!(date_parse("Thu, 11 Jan 1990 20:00:00 -0800",false), DateTime{year: Some(1990), mon: Some(1), mday:Some(11), hour: Some(20), min: Some(0), sec: Some(0), zone: Some("-0800".to_owned()), offset: Some(-8*3600), wday: Some(4), ..Default::default()});
    assert_eq!(date_parse("12-January-1990, 04:00 WET",false), DateTime{year: Some(1990), mon: Some(1), mday:Some(12), hour: Some(4), min: Some(0), sec: None, zone: Some("WET".to_owned()), offset: Some(0), wday: None, ..Default::default()});
    assert_eq!(date_parse("jan 2 3 am +4 5",false), DateTime{year: Some(5), mon: Some(1), mday:Some(2), hour: Some(3), min: None, sec: None, zone: Some("+4".to_owned()), offset: Some(4*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("jan 2 3 am +4 5",true), DateTime{year: Some(2005), mon: Some(1), mday:Some(2), hour: Some(3), min: None, sec: None, zone: Some("+4".to_owned()), offset: Some(4*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("fri1feb3bc4pm+5",false), DateTime{year: Some(-2), mon: Some(2), mday:Some(1), hour: Some(16), min: None, sec: None, zone: Some("+5".to_owned()), offset: Some(5*3600), wday: Some(5), bc: true, ..Default::default()});
    assert_eq!(date_parse("fri1feb3bc4pm+5",true), DateTime{year: Some(-2), mon: Some(2), mday:Some(1), hour: Some(16), min: None, sec: None, zone: Some("+5".to_owned()), offset: Some(5*3600), wday: Some(5), bc: true, ..Default::default()});
    assert_eq!(date_parse("03 feb 1st",false), DateTime{year: Some(3), mon: Some(2), mday:Some(1), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}

#[test]
pub fn test_apostrophe() {
    //# apostrophe
    assert_eq!(date_parse("July 4, '79",true), DateTime{year: Some(1979), mon: Some(7), mday:Some(4), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("4th July '79",true), DateTime{year: Some(1979), mon: Some(7), mday:Some(4), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}

#[test]
pub fn test_day_of_week() {
    //# day of week
    assert_eq!(date_parse("Sunday",false), DateTime{year: None, mon: None, mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: Some(0), ..Default::default()});
    assert_eq!(date_parse("Mon",false), DateTime{year: None, mon: None, mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: Some(1), ..Default::default()});
    assert_eq!(date_parse("Tue",false), DateTime{year: None, mon: None, mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: Some(2), ..Default::default()});
    assert_eq!(date_parse("Wed",false), DateTime{year: None, mon: None, mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: Some(3), ..Default::default()});
    assert_eq!(date_parse("Thurs",false), DateTime{year: None, mon: None, mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: Some(4), ..Default::default()});
    assert_eq!(date_parse("Friday",false), DateTime{year: None, mon: None, mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: Some(5), ..Default::default()});
    assert_eq!(date_parse("Sat.",false), DateTime{year: None, mon: None, mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("sat.",false), DateTime{year: None, mon: None, mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("SAT.",false), DateTime{year: None, mon: None, mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: Some(6), ..Default::default()});
    assert_eq!(date_parse("sAt.",false), DateTime{year: None, mon: None, mday:None, hour: None, min: None, sec: None, zone: None, offset: None, wday: Some(6), ..Default::default()});
}

#[test]
pub fn test_time() {
    //# time
    assert_eq!(date_parse("09:55",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: Some(55), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("09:55:30",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: Some(55), sec: Some(30), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("09:55:30am",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: Some(55), sec: Some(30), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("09:55:30pm",false), DateTime{year: None, mon: None, mday:None, hour: Some(21), min: Some(55), sec: Some(30), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("09:55:30a.m.",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: Some(55), sec: Some(30), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("09:55:30p.m.",false), DateTime{year: None, mon: None, mday:None, hour: Some(21), min: Some(55), sec: Some(30), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("09:55:30pm GMT",false), DateTime{year: None, mon: None, mday:None, hour: Some(21), min: Some(55), sec: Some(30), zone: Some("GMT".to_owned()), offset: Some(0), wday: None, ..Default::default()});
    assert_eq!(date_parse("09:55:30p.m. GMT",false), DateTime{year: None, mon: None, mday:None, hour: Some(21), min: Some(55), sec: Some(30), zone: Some("GMT".to_owned()), offset: Some(0), wday: None, ..Default::default()});
    assert_eq!(date_parse("09:55+0900",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: Some(55), sec: None, zone: Some("+0900".to_owned()), offset: Some(9*3600), wday: None, ..Default::default()});
    assert_eq!(date_parse("09 AM",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("09am",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("09 A.M.",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("09 PM",false), DateTime{year: None, mon: None, mday:None, hour: Some(21), min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("09pm",false), DateTime{year: None, mon: None, mday:None, hour: Some(21), min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("09 P.M.",false), DateTime{year: None, mon: None, mday:None, hour: Some(21), min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

    assert_eq!(date_parse("9h22m23s",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: Some(22), sec: Some(23), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("9h 22m 23s",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: Some(22), sec: Some(23), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("9h22m",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: Some(22), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("9h 22m",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: Some(22), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("9h",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("9h 22m 23s am",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: Some(22), sec: Some(23), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("9h 22m 23s pm",false), DateTime{year: None, mon: None, mday:None, hour: Some(21), min: Some(22), sec: Some(23), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("9h 22m am",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: Some(22), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("9h 22m pm",false), DateTime{year: None, mon: None, mday:None, hour: Some(21), min: Some(22), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("9h am",false), DateTime{year: None, mon: None, mday:None, hour: Some(9), min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("9h pm",false), DateTime{year: None, mon: None, mday:None, hour: Some(21), min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

    assert_eq!(date_parse("00:00",false), DateTime{year: None, mon: None, mday:None, hour: Some(0), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("01:00",false), DateTime{year: None, mon: None, mday:None, hour: Some(1), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("11:00",false), DateTime{year: None, mon: None, mday:None, hour: Some(11), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("12:00",false), DateTime{year: None, mon: None, mday:None, hour: Some(12), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("13:00",false), DateTime{year: None, mon: None, mday:None, hour: Some(13), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("23:00",false), DateTime{year: None, mon: None, mday:None, hour: Some(23), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("24:00",false), DateTime{year: None, mon: None, mday:None, hour: Some(24), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});

    assert_eq!(date_parse("00:00 AM",false), DateTime{year: None, mon: None, mday:None, hour: Some(0), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("12:00 AM",false), DateTime{year: None, mon: None, mday:None, hour: Some(0), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("01:00 AM",false), DateTime{year: None, mon: None, mday:None, hour: Some(1), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("11:00 AM",false), DateTime{year: None, mon: None, mday:None, hour: Some(11), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("00:00 PM",false), DateTime{year: None, mon: None, mday:None, hour: Some(12), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("12:00 PM",false), DateTime{year: None, mon: None, mday:None, hour: Some(12), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("01:00 PM",false), DateTime{year: None, mon: None, mday:None, hour: Some(13), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("11:00 PM",false), DateTime{year: None, mon: None, mday:None, hour: Some(23), min: Some(0), sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}

#[test]
pub fn test_other() {

    //# pick up the rest
    assert_eq!(date_parse("2000-01-02 1",false), DateTime{year: Some(2000), mon: Some(1), mday:Some(2), hour: Some(1), min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("2000-01-02 23",false), DateTime{year: Some(2000), mon: Some(1), mday:Some(2), hour: Some(23), min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("2000-01-02 24",false), DateTime{year: Some(2000), mon: Some(1), mday:Some(2), hour: Some(24), min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("1 03:04:05",false), DateTime{year: None, mon: None, mday:Some(1), hour: Some(3), min: Some(4), sec: Some(5), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("02 03:04:05",false), DateTime{year: None, mon: None, mday:Some(2), hour: Some(3), min: Some(4), sec: Some(5), zone: None, offset: None, wday: None, ..Default::default()});
    assert_eq!(date_parse("31 03:04:05",false), DateTime{year: None, mon: None, mday:Some(31), hour: Some(3), min: Some(4), sec: Some(5), zone: None, offset: None, wday: None, ..Default::default()});

    // # null, space
    assert_eq!(date_parse("",false), DateTime{..Default::default()});
    assert_eq!(date_parse(" ",false), DateTime{..Default::default()});
    assert_eq!(date_parse("          ",false), DateTime{..Default::default()});
    assert_eq!(date_parse("\t",false), DateTime{..Default::default()});
    assert_eq!(date_parse("\n",false), DateTime{..Default::default()});
    assert_eq!(date_parse("\r",false), DateTime{..Default::default()});
    assert_eq!(date_parse("\t\n\0\r ",false), DateTime{..Default::default()});
    assert_eq!(date_parse("1999-05-23\t\n\0\r 21:34:56",false), DateTime{year: Some(1999), mon: Some(5), mday: Some(23), hour: Some(21), min: Some(34), sec: Some(56), zone: None, offset: None, wday: None, ..Default::default()});
}


#[test]
pub fn test_parse_slash_exp() {
assert_eq!(date_parse("2/5/1999",false),DateTime{year: Some(1999), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
assert_eq!(date_parse("02/05/1999",false),DateTime{year: Some(1999), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
assert_eq!(date_parse("02/05/-1999",false),DateTime{year: Some(-1999), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
assert_eq!(date_parse("05/02",false),DateTime{year: None, mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
assert_eq!(date_parse(" 5/ 2",false),DateTime{year: None, mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

assert_eq!(date_parse("2/5/'99",true),DateTime{year: Some(1999), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
assert_eq!(date_parse("2/5/0099",false),DateTime{year: Some(99), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
assert_eq!(date_parse("2/5/0099",true),DateTime{year: Some(99), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

assert_eq!(date_parse("2/5 1999",false),DateTime{year: Some(1999), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
assert_eq!(date_parse("2/5-1999",false),DateTime{year: Some(1999), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
assert_eq!(date_parse("2/5--1999",false),DateTime{year: Some(-1999), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

//# big
assert_eq!(date_parse("99/5/2",false),DateTime{year: Some(99), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
assert_eq!(date_parse("99/5/2",true),DateTime{year: Some(1999), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

assert_eq!(date_parse("1999/5/2",false),DateTime{year: Some(1999), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
assert_eq!(date_parse("1999/05/02",false),DateTime{year: Some(1999), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
assert_eq!(date_parse("-1999/05/02",false),DateTime{year: Some(-1999), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

assert_eq!(date_parse("0099/5/2",false),DateTime{year: Some(99), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
assert_eq!(date_parse("0099/5/2",true),DateTime{year: Some(99), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});

assert_eq!(date_parse("'99/5/2",false),DateTime{year: Some(99), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
assert_eq!(date_parse("'99/5/2",true),DateTime{year: Some(1999), mon: Some(5), mday: Some(2), hour: None, min: None, sec: None, zone: None, offset: None, wday: None, ..Default::default()});
}


fn get_time_values(dt: DateTime) -> (Option<i64>, Option<i64>, Option<i64>, Option<f64>) {
    (dt.hour, dt.min, dt.sec, dt.sec_fraction)
}
fn get_date_values(dt: DateTime) -> (Option<i64>, Option<i64>, Option<i64>) {
    (dt.year, dt.mon, dt.mday)
}
fn get_cw_values(dt: DateTime) -> (Option<i64>, Option<i64>, Option<i64>) {
    (dt.cwyear, dt.cweek, dt.cwday)
}
fn get_yday_values(dt: DateTime) -> (Option<i64>, Option<i64>) {
    (dt.year, dt.yday)
}

#[test]
pub fn test_parse_2(){
    let mut h = date_parse("22:45:59.5", true);
    assert_eq!((Some(22), Some(45), Some(59), Some(0.5)), get_time_values(h));
    h = date_parse("22:45:59.05", true);
    assert_eq!((Some(22), Some(45), Some(59), Some(0.05)), get_time_values(h));
    h = date_parse("22:45:59.005", true);
    assert_eq!((Some(22), Some(45), Some(59), Some(0.005)), get_time_values(h));
    h = date_parse("22:45:59.0123", true);
    assert_eq!((Some(22), Some(45), Some(59), Some(0.0123)), get_time_values(h));

    h = date_parse("224559.5", false);
    assert_eq!((Some(22), Some(45), Some(59), Some(0.5)), get_time_values(h));
    h = date_parse("224559.05", true);
    assert_eq!((Some(22), Some(45), Some(59), Some(0.05)), get_time_values(h));
    h = date_parse("224559.005", true);
    assert_eq!((Some(22), Some(45), Some(59), Some(0.005)), get_time_values(h));
    h = date_parse("224559.0123", true);
    assert_eq!((Some(22), Some(45), Some(59), Some(0.0123)), get_time_values(h));


    h = date_parse("2006-w15-5", true);
    assert_eq!((Some(2006), Some(15), Some(5)), get_cw_values(h));
    h = date_parse("2006w155", true);
    assert_eq!((Some(2006), Some(15), Some(5)), get_cw_values(h));
    h = date_parse("06w155", false);
    assert_eq!((Some(6), Some(15), Some(5)), get_cw_values(h));
    h = date_parse("06w155", true);
    assert_eq!((Some(2006), Some(15), Some(5)), get_cw_values(h));

    h = date_parse("2006-w15", true);
    assert_eq!((Some(2006), Some(15), None), get_cw_values(h));
    h = date_parse("2006w15", true);
    assert_eq!((Some(2006), Some(15), None), get_cw_values(h));

    h = date_parse("-w15-5", true);
    assert_eq!((None, Some(15), Some(5)), get_cw_values(h));
    h = date_parse("-w155", true);
    assert_eq!((None, Some(15), Some(5)), get_cw_values(h));

    h = date_parse("-w15", true);
    assert_eq!((None, Some(15), None), get_cw_values(h));
    h = date_parse("-w15", true);
    assert_eq!((None, Some(15), None), get_cw_values(h));

    h = date_parse("-w-5", true);
    assert_eq!((None, None, Some(5)), get_cw_values(h));

    h = date_parse("--11-29", true);
    assert_eq!((None, Some(11), Some(29)), get_date_values(h));
    h = date_parse("--1129", true);
    assert_eq!((None, Some(11), Some(29)), get_date_values(h));
    h = date_parse("--11", true);
    assert_eq!((None, Some(11), None), get_date_values(h));
    h = date_parse("---29", true);
    assert_eq!((None, None, Some(29)), get_date_values(h));
    h = date_parse("-333", true);
    assert_eq!((None, Some(333)), get_yday_values(h));

    h = date_parse("2006-333", true);
    assert_eq!((Some(2006), Some(333)), get_yday_values(h));
    h = date_parse("2006333", true);
    assert_eq!((Some(2006), Some(333)), get_yday_values(h));
    h = date_parse("06333", false);
    assert_eq!((Some(6), Some(333)), get_yday_values(h));
    h = date_parse("06333", true);
    assert_eq!((Some(2006), Some(333)), get_yday_values(h));
    h = date_parse("333", true);
    assert_eq!((None, Some(333)), get_yday_values(h));

    h = date_parse("", true);
    assert_eq!(DateTime::default(), h);
}
