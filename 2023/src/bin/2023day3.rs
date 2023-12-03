use aoc2023::*;

pub fn main() -> Result<()> {
    let input = read_input(3)?;
    let input = Schematic::from_str(&input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Default, Debug)]
struct Schematic {
    rows: Vec<String>,
    symbols: HashSet<(usize, usize)>,
    digit_runs: HashSet<(usize, usize, usize)>,
}

impl FromStr for Schematic {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ret = Self::default();

        let width = s.lines().next().unwrap().len() + 2;
        ret.rows.push(".".repeat(width));
        for line in s.lines() {
            ret.rows.push(format!(".{}.", line));
        }
        ret.rows.push(".".repeat(width));

        for (rown, row) in ret.rows.iter().enumerate() {
            let mut digits: Option<(usize, usize, usize)> = None;
            for (col, ch) in row.chars().enumerate() {
                match ch {
                    '0'..='9' => {
                        digits = match digits {
                            None => Some((rown, col, col)),
                            Some((srow, scol, ecol)) => Some((srow, scol, ecol + 1)),
                        };
                    }
                    '.' => {
                        if let Some(dpos) = digits.take() {
                            ret.digit_runs.insert(dpos);
                        }
                    }
                    _ => {
                        if let Some(dpos) = digits.take() {
                            ret.digit_runs.insert(dpos);
                        }
                        ret.symbols.insert((rown, col));
                    }
                }
            }
        }

        Ok(ret)
    }
}

impl Schematic {
    fn _digits_overlapping(
        &self,
        dset: &mut HashSet<(usize, usize, usize)>,
        row: usize,
        col: usize,
    ) {
        for &(rrow, scol, ecol) in &self.digit_runs {
            if rrow == row && col >= scol && col <= ecol {
                dset.insert((rrow, scol, ecol));
            }
        }
    }

    fn digits_overlapping(&self, row: usize, col: usize) -> HashSet<(usize, usize, usize)> {
        let mut dset = HashSet::new();
        for drow in (row - 1)..=(row + 1) {
            for dcol in (col - 1)..=(col + 1) {
                self._digits_overlapping(&mut dset, drow, dcol);
            }
        }

        dset
    }

    fn value_of(&self, dpos: (usize, usize, usize)) -> u64 {
        let mut ret = 0;
        for ch in self.rows[dpos.0]
            .chars()
            .skip(dpos.1)
            .take((dpos.2 - dpos.1) + 1)
        {
            ret *= 10;
            ret += ((ch as u8) - b'0') as u64;
        }
        ret
    }
}

fn part1(input: &Schematic) -> u64 {
    input
        .symbols
        .iter()
        .flat_map(|&(row, col)| input.digits_overlapping(row, col))
        .map(|dpos| input.value_of(dpos))
        .sum()
}

fn part2(input: &Schematic) -> u64 {
    input
        .symbols
        .iter()
        .filter_map(|&(row, col)| {
            let ch = input.rows[row].chars().nth(col).unwrap();
            if ch == '*' {
                let dsets = input.digits_overlapping(row, col);
                if dsets.len() == 2 {
                    Some(
                        dsets
                            .into_iter()
                            .map(|dpos| input.value_of(dpos))
                            .product::<u64>(),
                    )
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn testcase1() {
        let input: Schematic = Schematic::from_str(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 4361);
    }

    #[test]
    fn testcase2() {
        let input: Schematic = Schematic::from_str(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 467835);
    }
}
