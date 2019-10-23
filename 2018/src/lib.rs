use std::fs::read_to_string;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use lazy_static::lazy_static;
pub use regex::Regex;
pub use std::cmp::{max, min};
pub use std::collections::HashMap;
pub use std::collections::HashSet;

pub use permutohedron::Heap;

pub use serde_json::Value;

pub use chrono::prelude::*;

pub use parsebyregex::ParseByRegex;
pub use parsebyregex_derive::ParseByRegex;

pub use std::collections::VecDeque;

pub use twoway;

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

pub fn input_as_vec_and_first<T: ParseByRegex, S: AsRef<str>>(
    plain: S,
) -> Result<(Vec<T>, String)> {
    let mut lines = plain.as_ref().trim().lines();
    let first = lines.next();
    let mapped: Result<Vec<T>> = lines
        .filter(|s| !s.trim().is_empty()) // In case there are any blanks after the first line
        .map(ParseByRegex::parse_by_regex)
        .collect();
    if first.is_none() {
        Err("No lines at all?")?
    }
    Ok((
        mapped?,
        first.expect("Something went wrong").trim().to_owned(),
    ))
}

pub fn read_input_as_vec_and_first<T: ParseByRegex>(day: usize) -> Result<(Vec<T>, String)> {
    let plain = read_input(day)?;
    Ok(input_as_vec_and_first(plain)?)
}
