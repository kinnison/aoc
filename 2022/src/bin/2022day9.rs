use aoc2022::*;

#[derive(ParseByRegex, Debug, Clone, Copy)]
enum Direction {
    #[regex = "[Uu]"]
    Up,
    #[regex = "[Dd]"]
    Down,
    #[regex = "[Ll]"]
    Left,
    #[regex = "[Rr]"]
    Right,
}

impl Direction {
    fn domove(self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        }
    }
}

#[derive(ParseByRegex, Debug, Clone, Copy)]
#[regex = r"(?P<dir>.) (?P<count>\d+)"]
struct InputStep {
    dir: Direction,
    count: usize,
}

fn track_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if head.0.abs_diff(tail.0) < 2 && head.1.abs_diff(tail.1) < 2 {
        // within 1 in each direction, so tail does not move
        return tail;
    }
    if head.0 == tail.0 {
        // if in-line vertically move up/down to match
        if head.1 < tail.1 {
            (tail.0, tail.1 - 1)
        } else {
            (tail.0, tail.1 + 1)
        }
    } else if head.1 == tail.1 {
        // if in-line horizontally move left/right to match
        if head.0 < tail.0 {
            (tail.0 - 1, tail.1)
        } else {
            (tail.0 + 1, tail.1)
        }
    } else {
        // We need to move diagonally
        (
            if head.0 < tail.0 {
                tail.0 - 1
            } else {
                tail.0 + 1
            },
            if head.1 < tail.1 {
                tail.1 - 1
            } else {
                tail.1 + 1
            },
        )
    }
}

fn part1(input: &[InputStep]) -> usize {
    let mut tailpositions = HashSet::new();
    let mut head = (0i32, 0i32);
    let mut tail = (0i32, 0i32);
    tailpositions.insert(tail);

    for step in input {
        for _ in 0..step.count {
            head = step.dir.domove(head);
            tail = track_tail(head, tail);
            tailpositions.insert(tail);
        }
    }

    tailpositions.len()
}

fn part2(input: &[InputStep]) -> usize {
    let mut tailpositions = HashSet::new();

    let mut knots = vec![(0i32, 0i32); 10];
    tailpositions.insert(knots[9]);

    for step in input {
        for _ in 0..step.count {
            knots[0] = step.dir.domove(knots[0]);
            for n in 1..=9 {
                knots[n] = track_tail(knots[n - 1], knots[n]);
            }
            tailpositions.insert(knots[9]);
        }
    }

    tailpositions.len()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    static TEST_INPUT2: &str = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    #[test]
    fn testcase1() {
        let input: Vec<InputStep> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(dbg!(&input)), 13);
    }

    #[test]
    fn testcase2() {
        let input: Vec<InputStep> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 1);
        let input: Vec<InputStep> = input_as_vec(TEST_INPUT2).unwrap();
        assert_eq!(part2(&input), 36);
    }
}

pub fn main() -> Result<()> {
    let input: Vec<InputStep> = read_input_as_vec(9)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
