use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use aoc2020::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Decks {
    p1: VecDeque<u8>,
    p2: VecDeque<u8>,
}

impl Decks {
    fn fpr(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.p1.hash(&mut hasher);
        self.p2.hash(&mut hasher);
        hasher.finish()
    }
}

impl FromStr for Decks {
    type Err = GenError;

    fn from_str(value: &str) -> Result<Self> {
        // Rough format:
        // Player1: nums \n\n Player2: nums
        let value = value.trim();
        let gap = value.find("\n\n").ok_or("no gap?")?;
        let (p1, p2) = value.split_at(gap);
        let p1 = p1.trim().lines().skip(1);
        let p2 = p2.trim().lines().skip(1);

        let p1: StdResult<_, _> = p1.map(|s| s.parse()).collect();
        let p2: StdResult<_, _> = p2.map(|s| s.parse()).collect();

        let p1 = p1?;
        let p2 = p2?;

        Ok(Self { p1, p2 })
    }
}

fn part1(input: &Decks) -> usize {
    let mut game = input.clone();
    while !(game.p1.is_empty() || game.p2.is_empty()) {
        let p1card = game.p1.pop_front().unwrap();
        let p2card = game.p2.pop_front().unwrap();
        if p1card > p2card {
            game.p1.push_back(p1card);
            game.p1.push_back(p2card);
        } else {
            game.p2.push_back(p2card);
            game.p2.push_back(p1card);
        }
    }
    // Game over, give winning player their score
    let winner = if game.p1.is_empty() {
        &game.p2
    } else {
        &game.p1
    };
    winner
        .iter()
        .rev()
        .copied()
        .enumerate()
        .map(|(i, c)| (i + 1) * (c as usize))
        .sum()
}

fn recursive_game(mut game: Decks) -> (u8, Decks) {
    // During a recursive game, we can't repeat arrangements
    let mut seen = HashSet::new();
    while !(game.p1.is_empty() || game.p2.is_empty()) {
        let fpr = game.fpr();
        if seen.contains(&fpr) {
            // We've seen this arrangement before, p1 wins by default
            return (1, game);
        }
        seen.insert(fpr);
        // Okay, unique arrangement, play a round...
        let p1card = game.p1.pop_front().unwrap() as usize;
        let p2card = game.p2.pop_front().unwrap() as usize;
        let winner = if (game.p1.len() >= p1card) && (game.p2.len() >= p2card) {
            // We can recurse, so let's do that
            let mut subgame = game.clone();
            while subgame.p1.len() > p1card {
                subgame.p1.pop_back();
            }
            while subgame.p2.len() > p2card {
                subgame.p2.pop_back();
            }
            recursive_game(subgame).0
        } else {
            // We can't recurse, normal round
            if p1card > p2card {
                1
            } else {
                2
            }
        };
        let p1card = p1card as u8;
        let p2card = p2card as u8;
        if winner == 1 {
            game.p1.push_back(p1card);
            game.p1.push_back(p2card);
        } else {
            game.p2.push_back(p2card);
            game.p2.push_back(p1card);
        }
    }
    // Since we reached here, the winner is simply whoever has cards.
    let winner = if game.p1.is_empty() { 2 } else { 1 };
    (winner, game)
}

fn part2(input: &Decks) -> usize {
    // Recursive gaming...
    let (winner, game) = recursive_game(input.clone());
    let winner = if winner == 1 { &game.p1 } else { &game.p2 };
    winner
        .iter()
        .rev()
        .copied()
        .enumerate()
        .map(|(i, c)| (i + 1) * (c as usize))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;

    #[test]
    fn testcase1() {
        let decks = Decks::from_str(TEST_INPUT).unwrap();
        assert_eq!(part1(&decks), 306);
    }

    #[test]
    fn testcase2() {
        let decks = Decks::from_str(TEST_INPUT).unwrap();
        assert_eq!(part2(&decks), 291);
    }
}

fn main() -> Result<()> {
    let input: String = read_input(22)?;
    let input = Decks::from_str(&input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
