#[macro_use]
extern crate lazy_static;

pub mod date_parser;
use crate::date_parser::date_parse;

fn main() {
    let x = date_parse("10 sept 2022 01:02:03.222333 p.m. +05:30 Mon", true);
    println!("Hello, world! {:?}", x);

}
