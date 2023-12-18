//#![allow(clippy::needless_question_mark)]
use std::{
    fs::read_to_string,
    ops::{Div, Mul},
};

pub type StdResult<T, E> = std::result::Result<T, E>;
pub type GenError = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, GenError>;

pub use gcd::Gcd;
pub use lazy_static::lazy_static;
pub use regex::{Regex, RegexBuilder};
pub use std::cmp::{max, min, Ordering};
pub use std::collections::hash_map::Entry;
pub use std::collections::HashMap;
pub use std::collections::HashSet;
pub use std::convert::{Infallible, TryFrom, TryInto};
pub use std::fmt;
pub use std::iter::{successors, Peekable};
pub use std::ops::{Deref, DerefMut};
pub use std::str::FromStr;

pub use permutohedron::Heap;

pub use serde::Deserialize;
pub use serde_json::Value;

pub use chrono::prelude::*;

pub use parsebyregex::ParseByRegex;
pub use parsebyregex_derive::ParseByRegex;

pub use std::collections::VecDeque;

pub use itertools::*;

pub use modinverse::*;

pub use memoize::memoize;

pub fn read_input(day: usize) -> Result<String> {
    color_backtrace::install();
    Ok(read_to_string(format!("inputs/day{}", day))?)
}

pub fn read_input_as_vec<T: ParseByRegex>(day: usize) -> Result<Vec<T>> {
    let plain = read_input(day)?;
    input_as_vec(plain)
}

pub fn input_as_vec<T: ParseByRegex, S: AsRef<str>>(plain: S) -> Result<Vec<T>> {
    plain
        .as_ref()
        .trim()
        .lines()
        .map(ParseByRegex::parse_by_regex)
        .collect()
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
    if let Some(first) = first {
        Ok((mapped?, first.trim().to_owned()))
    } else {
        Err("No lines at all?".into())
    }
}

pub fn read_input_as_vec_and_first<T: ParseByRegex>(day: usize) -> Result<(Vec<T>, String)> {
    let plain = read_input(day)?;
    input_as_vec_and_first(plain)
}

pub fn line_as_list<T: ParseByRegex, S: AsRef<str>>(line: S) -> Result<Vec<T>> {
    line.as_ref()
        .trim()
        .split(',')
        .map(ParseByRegex::parse_by_regex)
        .collect()
}

pub fn input_as_lists<T: ParseByRegex, S: AsRef<str>>(input: S) -> Result<Vec<Vec<T>>> {
    input.as_ref().trim().lines().map(line_as_list).collect()
}

pub fn read_input_as_lists<T: ParseByRegex>(day: usize) -> Result<Vec<Vec<T>>> {
    let plain = read_input(day)?;
    input_as_lists(plain)
}

pub fn input_by_split_pat<T: ParseByRegex, S: AsRef<str>>(input: S, pat: &str) -> Result<Vec<T>> {
    input
        .as_ref()
        .trim()
        .split(pat)
        .map(ParseByRegex::parse_by_regex)
        .collect()
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

pub fn input_as_groups<T>(input: &str) -> Result<Vec<Vec<T>>>
where
    T: ParseByRegex,
{
    input
        .trim()
        .lines()
        .group_by(|&s| s.is_empty())
        .into_iter()
        .filter(|i| !i.0)
        .map(|(_, lines)| lines.map(T::parse_by_regex).collect::<Result<Vec<_>>>())
        .collect()
}

pub fn read_input_as_groups<T>(day: usize) -> Result<Vec<Vec<T>>>
where
    T: ParseByRegex,
{
    let plain = read_input(day)?;
    input_as_groups(&plain)
}

pub fn input_as_chunks<T>(input: &str) -> Result<Vec<T>>
where
    T: ParseByRegex,
{
    input.trim().split("\n\n").map(T::parse_by_regex).collect()
}

pub fn read_input_as_chunks<T>(day: usize) -> Result<Vec<T>>
where
    T: ParseByRegex,
{
    let plain = read_input(day)?;
    input_as_chunks(&plain)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SpacedString(Vec<String>);

impl FromStr for SpacedString {
    type Err = Infallible;

    fn from_str(value: &str) -> StdResult<Self, Self::Err> {
        let v = input_by_split_pat(value, " ").unwrap();
        Ok(Self(v))
    }
}

impl Deref for SpacedString {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SpacedString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CommaSpacedString(Vec<String>);

impl FromStr for CommaSpacedString {
    type Err = Infallible;

    fn from_str(value: &str) -> StdResult<Self, Self::Err> {
        let v = input_by_split_pat(value, ", ").unwrap();
        Ok(Self(v))
    }
}

impl Deref for CommaSpacedString {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CommaSpacedString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// 2020 specific stuff

#[derive(ParseByRegex, Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Facing {
    #[regex = "[NnUu]"]
    North,
    #[regex = "[EeRr]"]
    East,
    #[regex = "[SsDd]"]
    South,
    #[regex = "[WwLl]"]
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

    pub fn row_col_offset(self) -> (i32, i32) {
        match self {
            Facing::North => (-1, 0),
            Facing::East => (0, 1),
            Facing::South => (1, 0),
            Facing::West => (0, -1),
        }
    }

    pub fn do_row_col_move(self, row: i32, col: i32) -> (i32, i32) {
        let ofs = self.row_col_offset();
        (row + ofs.0, col + ofs.1)
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

    #[allow(clippy::comparison_chain)]
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

    #[allow(clippy::comparison_chain)]
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

// Stuff from 2021

pub fn triangle(n: i32) -> i32 {
    (n * (n - 1)) / 2
}

pub fn hex_byte_to_value(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => panic!("Invalue hex digit {}", b as char),
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
