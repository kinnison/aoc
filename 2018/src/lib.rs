use std::fs::read_to_string;

pub type Result<T> = std::result::Result<T, Box<std::error::Error>>;

pub use lazy_static::lazy_static;
pub use regex::Regex;
pub use std::collections::HashMap;
pub use std::collections::HashSet;

pub use permutohedron::Heap;

pub use serde_json::Value;

pub fn read_input(day: usize) -> Result<String> {
    Ok(read_to_string(format!("inputs/day{}", day))?)
}
