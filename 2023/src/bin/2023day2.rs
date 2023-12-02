use aoc2023::*;

#[derive(Debug, ParseByRegex)]
enum RoundPart {
    #[regex = r"(\d+) blue"]
    Blue(u64),
    #[regex = r"(\d+) green"]
    Green(u64),
    #[regex = r"(\d+) red"]
    Red(u64),
}

#[derive(Debug)]
struct Round {
    red: u64,
    green: u64,
    blue: u64,
}

impl FromStr for Round {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<RoundPart> = input_by_split_pat(s, ", ").unwrap();
        let mut ret = Round {
            red: 0,
            green: 0,
            blue: 0,
        };

        for part in parts {
            match part {
                RoundPart::Blue(n) => ret.blue += n,
                RoundPart::Green(n) => ret.green += n,
                RoundPart::Red(n) => ret.red += n,
            }
        }
        Ok(ret)
    }
}

#[derive(Debug)]
struct Rounds {
    rounds: Vec<Round>,
}

impl FromStr for Rounds {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self {
            rounds: input_by_split_pat(s, "; ").unwrap(),
        })
    }
}

#[derive(Debug, ParseByRegex)]
#[regex = r"Game (?P<gameid>\d+): (?P<rounds>.+)"]
struct Game {
    gameid: u64,
    rounds: Rounds,
}

pub fn main() -> Result<()> {
    let input: Vec<Game> = read_input_as_vec(2)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

impl Game {
    fn possible(&self, red: u64, green: u64, blue: u64) -> bool {
        for round in &self.rounds.rounds {
            if round.red > red || round.green > green || round.blue > blue {
                return false;
            }
        }
        true
    }

    fn power(&self) -> u64 {
        // Power of a game is the minimum set of cubes multiplied together
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for round in &self.rounds.rounds {
            min_red = min_red.max(round.red);
            min_green = min_green.max(round.green);
            min_blue = min_blue.max(round.blue);
        }

        min_red * min_green * min_blue
    }
}

fn part1(input: &[Game]) -> u64 {
    // With a limit of 12 red, 13 green, 14 blue
    // Sum the game IDs which are possible

    input
        .iter()
        .filter(|game| game.possible(12, 13, 14))
        .map(|game| game.gameid)
        .sum()
}

fn part2(input: &[Game]) -> u64 {
    input.iter().map(|game| game.power()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn testcase1() {
        let input: Vec<Game> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn testcase2() {
        let input: Vec<Game> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 2286);
    }
}
