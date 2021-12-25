use std::hint::unreachable_unchecked;

use aoc2021::*;

struct SeaFloor {
    width: usize,
    height: usize,
    cukes: Vec<u8>,
}

impl FromStr for SeaFloor {
    type Err = ();

    fn from_str(input: &str) -> StdResult<Self, Self::Err> {
        let input = input.trim();
        let height = input.lines().count();
        let width = input.lines().next().unwrap().trim().len();
        let mut cukes = Vec::with_capacity(width * height);

        for ch in input.chars() {
            match ch {
                '.' => cukes.push(0),
                '>' => cukes.push(1),
                'v' => cukes.push(2),
                _ => (),
            }
        }

        assert_eq!(cukes.len(), width * height);

        Ok(Self {
            width,
            height,
            cukes,
        })
    }
}

//fn print_cukes(width: usize, height: usize, cukes: &[u8]) {
//    let mut pos = 0;
//    for row in 0..height {
//        for col in 0..width {
//            match cukes[pos]
//        }
//    }
//}

fn facing_east(width: usize, _height: usize, row: usize, col: usize) -> usize {
    if col == width - 1 {
        row * width
    } else {
        (row * width) + col + 1
    }
}

fn facing_south(width: usize, height: usize, row: usize, col: usize) -> usize {
    if row == height - 1 {
        col
    } else {
        ((row + 1) * width) + col
    }
}

#[cfg(debug_assertions)]
fn print_cukes(label: &str, width: usize, height: usize, cukes: &[u8]) {
    println!("{}", label);
    let mut pos = 0;
    for _ in 0..height {
        for _ in 0..width {
            print!(
                "{}",
                match cukes[pos] {
                    0 => '.',
                    1 => '>',
                    2 => 'v',
                    _ => unsafe { unreachable_unchecked() },
                }
            );
            pos += 1;
        }
        println!();
    }
    println!();
}

fn part1(input: &SeaFloor) -> usize {
    let width = input.width;
    let height = input.height;
    let mut step1 = input.cukes.clone();
    let mut step2 = input.cukes.clone();
    let mut step3 = input.cukes.clone();
    let mut turns = 0;
    let mut moved = true;
    #[cfg(debug_assertions)]
    print_cukes("Starting positions", width, height, &step1);
    while moved {
        step2[..].fill(0);
        // Move the east-facing cukes from step1 to step2
        let mut pos = 0;
        for row in 0..height {
            for col in 0..width {
                let facing = facing_east(width, height, row, col);
                match step1[pos] {
                    0 => (),
                    1 => {
                        if step1[facing] == 0 {
                            step2[facing] = 1;
                        } else {
                            step2[pos] = 1;
                        }
                    }
                    2 => step2[pos] = 2,
                    _ => unsafe { unreachable_unchecked() },
                }
                pos += 1;
            }
        }
        step3[..].fill(0);
        pos = 0;
        // move the south facing cukes from step2 back to step 1
        for row in 0..height {
            for col in 0..width {
                let facing = facing_south(width, height, row, col);
                match step2[pos] {
                    0 => (),
                    1 => step3[pos] = 1,
                    2 => {
                        if step2[facing] == 0 {
                            step3[facing] = 2;
                        } else {
                            step3[pos] = 2
                        }
                    }
                    _ => unsafe { unreachable_unchecked() },
                }
                pos += 1;
            }
        }
        moved = step1[..] != step3[..];
        std::mem::swap(&mut step1, &mut step3);
        turns += 1;

        #[cfg(debug_assertions)]
        print_cukes(&format!("After {} turns", turns), width, height, &step1);
    }

    turns
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#;

    #[test]
    fn testcase1() {
        let input = SeaFloor::from_str(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 58);
    }
}

fn main() -> Result<()> {
    let input = read_input(25)?;
    let input = SeaFloor::from_str(&input).unwrap();
    println!("Loaded sea floor {}x{}", input.width, input.height);
    println!("Part 1: {}", part1(&input));
    Ok(())
}
