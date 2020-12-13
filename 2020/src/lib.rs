use std::{
    fs::read_to_string,
    ops::{Div, Mul},
};

pub type StdResult<T, E> = std::result::Result<T, E>;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use gcd::Gcd;
pub use lazy_static::lazy_static;
pub use regex::Regex;
pub use std::cmp::{max, min, Ordering};
pub use std::collections::hash_map::Entry;
pub use std::collections::HashMap;
pub use std::collections::HashSet;
pub use std::convert::{TryFrom, TryInto};
pub use std::fmt;
pub use std::iter::successors;
pub use std::str::FromStr;

pub use permutohedron::Heap;

pub use serde_json::Value;

pub use chrono::prelude::*;

pub use parsebyregex::ParseByRegex;
pub use parsebyregex_derive::ParseByRegex;

pub use std::collections::VecDeque;

pub use twoway;

pub use itertools::*;

pub use modinverse::*;

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

pub fn input_by_split_pat<T: ParseByRegex, S: AsRef<str>>(input: S, pat: &str) -> Result<Vec<T>> {
    let mapped: Result<Vec<T>> = input
        .as_ref()
        .trim()
        .split(pat)
        .map(ParseByRegex::parse_by_regex)
        .collect();
    Ok(mapped?)
}

pub fn read_input_as_vec_split<T: ParseByRegex>(day: usize, pat: &str) -> Result<Vec<T>> {
    let plain = read_input(day)?;
    input_by_split_pat(plain, pat)
}

pub fn input_as_first_and_vec_by_pat<T: ParseByRegex, S: AsRef<str>>(
    input: S,
    pat: &str,
) -> Result<(String, Vec<T>)> {
    let input = input.as_ref().trim();
    let mut bits = input.splitn(2, '\n');
    let first = bits.next().ok_or("No lines")?;
    let rest = bits.next().ok_or("Only one line?")?;
    let first = first.to_string();
    let rest = input_by_split_pat(rest, pat)?;
    Ok((first, rest))
}

pub fn read_input_as_first_and_vec_by_pat<T: ParseByRegex>(
    day: usize,
    pat: &str,
) -> Result<(String, Vec<T>)> {
    let plain = read_input(day)?;
    input_as_first_and_vec_by_pat(plain, pat)
}

// 2020 specific stuff

#[derive(ParseByRegex, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Facing {
    #[regex = "[Nn]"]
    North,
    #[regex = "[Ee]"]
    East,
    #[regex = "[Ss]"]
    South,
    #[regex = "[Ww]"]
    West,
}

impl Facing {
    fn turn_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn turn_left_deg(mut self, mut deg: i32) -> Self {
        if deg < 0 {
            self.turn_right_deg(-deg)
        } else {
            while deg > 0 {
                self = self.turn_left();
                deg -= 90;
            }
            self
        }
    }

    pub fn turn_right_deg(mut self, mut deg: i32) -> Self {
        if deg < 0 {
            self.turn_left_deg(-deg)
        } else {
            while deg > 0 {
                self = self.turn_right();
                deg -= 90;
            }
            self
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct XYPosition {
    pub x: i32,
    pub y: i32,
}

impl XYPosition {
    pub fn moved(self, dir: Facing, amount: i32) -> Self {
        let delts = match dir {
            Facing::North => (0, 1),
            Facing::East => (1, 0),
            Facing::South => (0, -1),
            Facing::West => (-1, 0),
        };
        Self {
            x: self.x + (delts.0 * amount),
            y: self.y + (delts.1 * amount),
        }
    }

    pub fn origin_manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    #[allow(clippy::clippy::comparison_chain)]
    pub fn rotate_left(self, deg: i32) -> Self {
        if deg == 0 {
            self
        } else if deg < 0 {
            self.rotate_right(-deg)
        } else {
            (XYPosition {
                x: -self.y,
                y: self.x,
            })
            .rotate_left(deg - 90)
        }
    }

    #[allow(clippy::clippy::comparison_chain)]
    pub fn rotate_right(self, deg: i32) -> Self {
        if deg == 0 {
            self
        } else if deg < 0 {
            self.rotate_left(-deg)
        } else {
            (XYPosition {
                x: self.y,
                y: -self.x,
            })
            .rotate_right(deg - 90)
        }
    }
}

pub trait Lcm {
    type Output;
    fn lcm(self, other: Self) -> Self::Output;
}

impl<T> Lcm for T
where
    T: Copy + Gcd + Mul<T>,
    <T as Mul<T>>::Output: Div<T>,
{
    type Output = <<T as Mul>::Output as Div<T>>::Output;

    fn lcm(self, other: Self) -> Self::Output {
        (self * other) / self.gcd(other)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn posrot() {
        let pos = XYPosition { x: 10, y: 1 };
        assert_eq!(pos, pos.rotate_left(360));
        assert_eq!(pos.rotate_left(180), pos.rotate_right(180));
    }
}
