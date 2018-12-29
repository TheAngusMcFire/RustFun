#![allow(non_snake_case)]

extern crate regex;

use regex::Regex;
mod stuff;


fn main() 
{
    stuff::test();

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));
}
