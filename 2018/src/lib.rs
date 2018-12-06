use std::fs::read_to_string;

pub type Result<T> = std::result::Result<T, Box<std::error::Error>>;

pub use lazy_static::lazy_static;
pub use regex::Regex;
pub use std::collections::HashMap;
pub use std::collections::HashSet;
pub use std::cmp::{min,max};

pub use permutohedron::Heap;

pub use serde_json::Value;

pub use chrono::prelude::*;

pub use parsebyregex::ParseByRegex;
pub use parsebyregex_derive::ParseByRegex;

pub fn read_input(day: usize) -> Result<String> {
    Ok(read_to_string(format!("inputs/day{}", day))?)
}

pub fn read_input_as_vec<T: ParseByRegex>(day: usize) -> Result<Vec<T>> {
    let plain = read_input(day)?;
    Ok(input_as_vec(plain)?)
}

pub fn input_as_vec<T: ParseByRegex, S: AsRef<str>>(plain: S) -> Result<Vec<T>> {
    let mapped: Result<Vec<T>> = plain
        .as_ref()
        .trim()
        .lines()
        .map(ParseByRegex::parse_by_regex)
        .collect();
    Ok(mapped?)
}
