use std::fs::read_to_string;
pub use std::io::Result;

pub use lazy_static::lazy_static;
pub use regex::Regex;
pub use std::collections::HashMap;
pub use std::collections::HashSet;

pub use permutohedron::heap_recursive;
pub use permutohedron::Heap;

pub use serde_json::Value;

pub use itertools::Itertools;

pub use either::{Either, Left, Right};

pub fn read_input(day: usize) -> Result<String> {
    read_to_string(format!("inputs/day{}", day))
}
