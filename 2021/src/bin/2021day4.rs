use aoc2021::*;

#[derive(Debug)]
struct BingoCard {
    nums: Vec<Vec<usize>>,
}

#[derive(Debug)]
struct BingoGame {
    seq: Vec<usize>,
    cards: Vec<BingoCard>,
}

impl FromStr for BingoGame {
    type Err = GenError;

    fn from_str(input: &str) -> Result<Self> {
        let mut lines = input.trim().lines();

        let seq = input_by_split_pat(lines.next().unwrap(), ",")?;
        let mut cards = vec![];
        let mut card = BingoCard { nums: vec![] };
        while let Some(line) = lines.next() {
            assert_eq!(line, "");
            for _ in 0..5 {
                let mut line = lines.next().unwrap().trim().to_string();
                while line.contains("  ") {
                    line = line.replace("  ", " ");
                }
                let line = input_by_split_pat(line, " ")?;
                card.nums.push(line);
            }
            cards.push(std::mem::replace(&mut card, BingoCard { nums: vec![] }));
        }

        Ok(Self { seq, cards })
    }
}

struct BingoCardWins<'a> {
    card: &'a BingoCard,
    pos: usize,
}

impl<'a> Iterator for BingoCardWins<'a> {
    type Item = [usize; 5];

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == 10 {
            None
        } else if self.pos < 5 {
            // Rows
            let mut out = [0; 5];
            out[..5].clone_from_slice(&self.card.nums[self.pos][..5]);
            self.pos += 1;
            Some(out)
        } else {
            // Cols
            let mut out = [0; 5];
            (0..5).for_each(|i| {
                out[i] = self.card.nums[i][self.pos - 5];
            });
            self.pos += 1;
            Some(out)
        }
    }
}

impl BingoCard {
    // Outcomes:
    // None -> didn't win
    // Some((when, score)) -> did win at some point
    fn play(&self, seq: &[usize]) -> Option<(usize, usize)> {
        let mut called = HashSet::new();
        for (idx, val) in seq.iter().copied().enumerate() {
            called.insert(val);
            if idx < 5 {
                continue;
            }
            if let Some(remaining) = self.score(&called) {
                return Some((idx, remaining * val));
            }
        }

        None
    }
    fn score(&self, called: &HashSet<usize>) -> Option<usize> {
        let mut winning = false;
        'row: for row in self.wins() {
            for val in row {
                if !called.contains(&val) {
                    continue 'row;
                }
            }
            winning = true;
            break;
        }
        if winning {
            // We're winning, so compute unmarked total
            Some(
                self.nums
                    .iter()
                    .map(|r| {
                        r.iter()
                            .copied()
                            .filter(|n| !called.contains(n))
                            .sum::<usize>()
                    })
                    .sum(),
            )
        } else {
            None
        }
    }

    fn wins(&self) -> BingoCardWins<'_> {
        BingoCardWins { card: self, pos: 0 }
    }
}

fn part1(input: &BingoGame) -> usize {
    let mut wins: Vec<_> = input
        .cards
        .iter()
        .filter_map(|card| card.play(&input.seq))
        .collect();
    wins.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    // first winner should be first to win, so score that
    wins[0].1
}

fn part2(input: &BingoGame) -> usize {
    let mut wins: Vec<_> = input
        .cards
        .iter()
        .filter_map(|card| card.play(&input.seq))
        .collect();
    wins.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    // last winner should be first in wins, so score that
    wins[0].1
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    #[test]
    fn testcase1() {
        let input = BingoGame::from_str(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 4512);
    }

    #[test]
    fn testcase2() {
        let input = BingoGame::from_str(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 1924);
    }
}

fn main() -> Result<()> {
    let input = read_input(4)?;
    let input = BingoGame::from_str(&input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
