use aoc2022::*;

#[derive(ParseByRegex, Clone, Copy, Debug)]
enum InputStep {
    #[regex = r"addx (\-?\d+)"]
    Addx(i32),
    #[regex = "noop"]
    Noop,
}

fn run_machine(input: &[InputStep]) -> Vec<i32> {
    let mut ret = vec![1]; // cycle zero x is 1
    let mut x = 1;
    for step in input {
        match step {
            InputStep::Addx(dx) => {
                // Addx takes two cycles, during the first cycle, X is unchanged
                ret.push(x);
                // During the second cycle, X is unchanged
                ret.push(x);
                // After the second cycle, X updates
                x += *dx;
            }
            InputStep::Noop => {
                // Noop takes one cycle and does not change X
                // so *during* this cycle, X is unchanged
                ret.push(x);
            }
        }
    }
    ret
}

fn strength(cyclemap: &[i32], cycle: usize) -> i32 {
    cyclemap[cycle] * (cycle as i32)
}

fn part1(input: &[InputStep]) -> i32 {
    let cycle_map = run_machine(input);

    strength(&cycle_map, 20)
        + strength(&cycle_map, 60)
        + strength(&cycle_map, 100)
        + strength(&cycle_map, 140)
        + strength(&cycle_map, 180)
        + strength(&cycle_map, 220)
}

fn within(spos: i32, xpos: usize) -> bool {
    let xpos = xpos as i32;
    xpos >= spos - 1 && xpos <= spos + 1
}

fn part2(input: &[InputStep]) -> String {
    let cycle_map = run_machine(input);
    let mut ret = String::new();
    for (cycle, spos) in cycle_map.iter().copied().enumerate().skip(1) {
        let xpos = (cycle - 1) % 40;
        if xpos == 0 {
            ret.push('\n');
        }
        if within(spos, xpos) {
            ret.push('#');
        } else {
            ret.push('.');
        }
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;

    #[test]
    fn testcase1() {
        let input: Vec<InputStep> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(dbg!(&input)), 13140);
    }

    static TEST_OUTPUT: &str = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#;

    #[test]
    fn testcase2() {
        let input: Vec<InputStep> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), TEST_OUTPUT);
    }
}

pub fn main() -> Result<()> {
    let input: Vec<InputStep> = read_input_as_vec(10)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
