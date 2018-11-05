use std::fs::read_to_string;
use std::io::Result;

pub fn read_input(day: usize) -> Result<String> {
    read_to_string(format!("inputs/day{}", day))
}
