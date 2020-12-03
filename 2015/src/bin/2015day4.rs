use aoc2015::*;
use std::io::Result;

fn run(input: &str, mstr: &str) -> usize {
    for v in 0.. {
        let target = format!("{}{}", input, v);
        let digest = format!("{:x}", md5::compute(target));
        if digest[0..mstr.len()] == *mstr {
            return v;
        }
    }
    unreachable!()
}

fn main() -> Result<()> {
    let input = read_input(4)?.trim().to_owned();
    println!("Part 1: {}", run(&input, "00000"));
    println!("Part 2: {}", run(&input, "000000"));
    Ok(())
}
