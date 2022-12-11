use aoc2022::*;

#[path = "2022day1.rs"]
mod day01;
#[path = "2022day2.rs"]
mod day02;
#[path = "2022day3.rs"]
mod day03;
#[path = "2022day4.rs"]
mod day04;
#[path = "2022day5.rs"]
mod day05;
#[path = "2022day6.rs"]
mod day06;
#[path = "2022day7.rs"]
mod day07;
#[path = "2022day8.rs"]
mod day08;
#[path = "2022day9.rs"]
mod day09;
#[path = "2022day10.rs"]
mod day10;
#[path = "2022day11.rs"]
mod day11;

fn main() -> Result<()> {
    day01::main()?;
    day02::main()?;
    day03::main()?;
    day04::main()?;
    day05::main()?;
    day06::main()?;
    day07::main()?;
    day08::main()?;
    day09::main()?;
    day10::main()?;
    day11::main()?;

    Ok(())
}
