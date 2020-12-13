use aoc2020::*;

#[derive(Debug, ParseByRegex)]
pub enum Operation {
    #[regex = "[Nn]"]
    North,
    #[regex = "[Ee]"]
    East,
    #[regex = "[Ss]"]
    South,
    #[regex = "[Ww]"]
    West,
    #[regex = "[Ff]"]
    Forward,
    #[regex = "[Rr]"]
    Right,
    #[regex = "[Ll]"]
    Left,
}
#[derive(Debug, ParseByRegex)]
#[regex = r"(?P<op>.)(?P<val>\d+)"]
pub struct Instruction {
    op: Operation,
    val: i32,
}

fn part1(input: &[Instruction]) -> i32 {
    let mut facing = Facing::East;
    let mut pos = XYPosition::default();
    for instr in input {
        match instr.op {
            Operation::North => pos = pos.moved(Facing::North, instr.val),
            Operation::East => pos = pos.moved(Facing::East, instr.val),
            Operation::South => pos = pos.moved(Facing::South, instr.val),
            Operation::West => pos = pos.moved(Facing::West, instr.val),
            Operation::Forward => pos = pos.moved(facing, instr.val),
            Operation::Right => facing = facing.turn_right_deg(instr.val),
            Operation::Left => facing = facing.turn_left_deg(instr.val),
        }
    }
    pos.origin_manhattan()
}

fn part2(input: &[Instruction]) -> i32 {
    let mut waypoint = XYPosition { x: 10, y: 1 };
    let mut shippos = XYPosition::default();
    for instr in input {
        match instr.op {
            Operation::North => waypoint = waypoint.moved(Facing::North, instr.val),
            Operation::East => waypoint = waypoint.moved(Facing::East, instr.val),
            Operation::West => waypoint = waypoint.moved(Facing::West, instr.val),
            Operation::South => waypoint = waypoint.moved(Facing::South, instr.val),
            Operation::Right => waypoint = waypoint.rotate_right(instr.val),
            Operation::Left => waypoint = waypoint.rotate_left(instr.val),
            Operation::Forward => {
                shippos = XYPosition {
                    x: shippos.x + (waypoint.x * instr.val),
                    y: shippos.y + (waypoint.y * instr.val),
                }
            }
        }
    }
    shippos.origin_manhattan()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"F10
N3
F7
R90
F11"#;

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 25);
    }

    #[test]
    fn testcase2() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 286);
    }
}

fn main() -> Result<()> {
    let input = read_input_as_vec(12)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
