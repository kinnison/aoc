use aoc2022::*;

#[derive(ParseByRegex, Debug, Clone, Copy)]
enum HandShape {
    #[regex = "[AX]"]
    Rock,
    #[regex = "[BY]"]
    Paper,
    #[regex = "[CZ]"]
    Scissors,
}

impl HandShape {
    fn score(self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

use HandShape::*;

#[derive(ParseByRegex, Debug, Clone, Copy)]
#[regex = "(?P<opponent>.) (?P<me>.)"]
struct Round {
    opponent: HandShape,
    me: HandShape,
}

impl Round {
    fn score(self) -> usize {
        self.me.score()
            + match (self.opponent, self.me) {
                (Rock, Rock) => 3,
                (Rock, Paper) => 6,
                (Rock, Scissors) => 0,
                (Paper, Rock) => 0,
                (Paper, Paper) => 3,
                (Paper, Scissors) => 6,
                (Scissors, Rock) => 6,
                (Scissors, Paper) => 0,
                (Scissors, Scissors) => 3,
            }
    }

    fn part2_xlate(self) -> Self {
        // this is horrid and an example of me being premature in translating
        // the input data into rock/paper/scissors
        // XYZ was parsed as Rock/Paper/Scissors
        // Here it's Lose/Draw/Win, so we have to remap so we can score things
        Self {
            opponent: self.opponent,
            me: match (self.opponent, self.me) {
                (Rock, Rock) => Scissors,
                (Rock, Paper) => Rock,
                (Rock, Scissors) => Paper,
                (Paper, Rock) => Rock,
                (Paper, Paper) => Paper,
                (Paper, Scissors) => Scissors,
                (Scissors, Rock) => Paper,
                (Scissors, Paper) => Scissors,
                (Scissors, Scissors) => Rock,
            },
        }
    }
}

fn part1(input: &[Round]) -> usize {
    input.iter().copied().map(Round::score).sum()
}

fn part2(input: &[Round]) -> usize {
    input
        .iter()
        .copied()
        .map(Round::part2_xlate)
        .map(Round::score)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
A Y
B X
C Z

"#;

    #[test]
    fn testcase1() {
        let input: Vec<Round> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(dbg!(&input)), 15);
    }

    #[test]
    fn testcase2() {
        let input: Vec<Round> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 12);
    }
}

fn main() -> Result<()> {
    let input: Vec<Round> = read_input_as_vec(2)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
