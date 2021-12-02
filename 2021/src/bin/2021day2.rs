use aoc2021::*;

#[derive(ParseByRegex, Debug)]
enum Command {
    #[regex = "forward (.+)"]
    Forward(i32),
    #[regex = "down (.+)"]
    Down(i32),
    #[regex = "up (.+)"]
    Up(i32),
}

fn part1(input: &[Command]) -> i32 {
    // horiz, vert
    input
        .iter()
        .fold([0, 0], |acc, v| match v {
            Command::Forward(n) => [acc[0] + n, acc[1]],
            Command::Down(n) => [acc[0], acc[1] + n],
            Command::Up(n) => [acc[0], acc[1] - n],
        })
        .iter()
        .product()
}

fn part2(input: &[Command]) -> i32 {
    // horiz, vert, aim
    input.iter().fold([0, 0, 0], |acc, v| match v {
        Command::Down(n) => [acc[0], acc[1], acc[2] + n],
        Command::Up(n) => [acc[0], acc[1], acc[2] - n],
        Command::Forward(n) => [acc[0] + n, acc[1] + (acc[2] * n), acc[2]],
    })[..=1]
        .iter()
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

    #[test]
    fn testcase1() {
        let input: Vec<Command> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 150);
    }

    #[test]
    fn testcase2() {
        let input: Vec<Command> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 900);
    }
}

fn main() -> Result<()> {
    let input: Vec<Command> = read_input_as_vec(2)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
