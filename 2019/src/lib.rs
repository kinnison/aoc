use std::fs::read_to_string;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use lazy_static::lazy_static;
pub use regex::Regex;
pub use std::cmp::{max, min};
pub use std::collections::HashMap;
pub use std::collections::HashSet;
pub use std::convert::{TryFrom, TryInto};
pub use std::iter::successors;

pub use permutohedron::Heap;

pub use serde_json::Value;

pub use chrono::prelude::*;

pub use parsebyregex::ParseByRegex;
pub use parsebyregex_derive::ParseByRegex;

pub use std::collections::VecDeque;

pub use twoway;

pub use itertools::*;

pub fn read_input(day: usize) -> Result<String> {
    color_backtrace::install();
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
        Err("No lines at all?".into())
    } else {
        Ok((
            mapped?,
            first.expect("Something went wrong").trim().to_owned(),
        ))
    }
}

pub fn read_input_as_vec_and_first<T: ParseByRegex>(day: usize) -> Result<(Vec<T>, String)> {
    let plain = read_input(day)?;
    Ok(input_as_vec_and_first(plain)?)
}

pub fn line_as_list<T: ParseByRegex, S: AsRef<str>>(line: S) -> Result<Vec<T>> {
    let mapped: Result<Vec<T>> = line
        .as_ref()
        .trim()
        .split(',')
        .map(ParseByRegex::parse_by_regex)
        .collect();
    Ok(mapped?)
}

pub fn input_as_lists<T: ParseByRegex, S: AsRef<str>>(input: S) -> Result<Vec<Vec<T>>> {
    let mapped: Result<Vec<Vec<T>>> = input.as_ref().trim().lines().map(line_as_list).collect();
    Ok(mapped?)
}

pub fn read_input_as_lists<T: ParseByRegex>(day: usize) -> Result<Vec<Vec<T>>> {
    let plain = read_input(day)?;
    Ok(input_as_lists(plain)?)
}

// 2019 specific stuff

pub mod intcode;

pub use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    pub fn rotate_left(self) -> Self {
        use Facing::*;
        match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }

    pub fn rotate_right(self) -> Self {
        use Facing::*;
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }

    pub fn move_by<N>(self, pos: (N, N)) -> (N, N)
    where
        N: num_traits::Num,
    {
        use Facing::*;
        match self {
            Up => (pos.0, pos.1 - N::one()),
            Down => (pos.0, pos.1 + N::one()),
            Left => (pos.0 - N::one(), pos.1),
            Right => (pos.0 + N::one(), pos.1),
        }
    }
}

// Stolen from rosetta code

pub fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn surrounds(pos: (i32, i32)) -> [(i32, i32); 4] {
    [
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
    ]
}
