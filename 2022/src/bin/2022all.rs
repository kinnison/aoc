use aoc2022::*;

#[path = "2022day1.rs"]
mod day1;
#[path = "2022day10.rs"]
mod day10;
#[path = "2022day2.rs"]
mod day2;
#[path = "2022day3.rs"]
mod day3;
#[path = "2022day4.rs"]
mod day4;
#[path = "2022day5.rs"]
mod day5;
#[path = "2022day6.rs"]
mod day6;
#[path = "2022day7.rs"]
mod day7;
#[path = "2022day8.rs"]
mod day8;
#[path = "2022day9.rs"]
mod day9;

fn main() -> Result<()> {
    day1::main()?;
    day2::main()?;
    day3::main()?;
    day4::main()?;
    day5::main()?;
    day6::main()?;
    day7::main()?;
    day8::main()?;
    day9::main()?;
    day10::main()?;

    Ok(())
}
