use aoc2023::*;

pub fn main() -> Result<()> {
    let input: Vec<Mountains> = read_input_as_chunks(13)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Debug)]
struct Mountains {
    #[allow(unused)]
    grid: Vec<Vec<Cell>>,
    perfect_hmirror: Option<usize>,
    perfect_vmirror: Option<usize>,
    smudged_hmirror: Option<usize>,
    smudged_vmirror: Option<usize>,
}

impl FromStr for Mountains {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let grid = s
            .trim()
            .lines()
            .map(|s| s.trim().chars().map(Cell::from_char).collect_vec())
            .collect_vec();

        let perfect_hmirror = Self::find_hmirror(&grid, 0);
        let smudged_hmirror = Self::find_hmirror(&grid, 1);

        // For vertical mirror, let's just transpose the grid and ask the same question
        let mut tgrid = vec![Vec::new(); grid[0].len()];
        for row in &grid {
            for (tr, c) in row.iter().enumerate() {
                tgrid[tr].push(*c);
            }
        }
        let perfect_vmirror = Self::find_hmirror(&tgrid, 0);
        let smudged_vmirror = Self::find_hmirror(&tgrid, 1);

        Ok(Self {
            grid,
            perfect_hmirror,
            perfect_vmirror,
            smudged_hmirror,
            smudged_vmirror,
        })
    }
}

impl Mountains {
    fn find_hmirror(grid: &[Vec<Cell>], smudge_goal: usize) -> Option<usize> {
        // A horizontal mirror involves checking if rows are equal...
        let mut hmirror = None;
        'hmirror: for hpos in 1..grid.len() {
            // A mirror at hpos as rows 0..hpos before it and hpos.. after it
            // The mirror is valid if all possible rows before / after match or are missing
            let mut smudges = 0;
            for mrow in 0..hpos {
                let dist = hpos - mrow - 1;
                let rrow = hpos + dist;
                if let Some(lower) = grid.get(rrow) {
                    smudges += grid[mrow]
                        .iter()
                        .copied()
                        .zip(lower.iter().copied())
                        .filter(|&(a, b)| a != b)
                        .count();
                    if smudges > smudge_goal {
                        continue 'hmirror;
                    }
                }
            }
            if smudges == smudge_goal {
                hmirror = Some(hpos);
                break;
            }
        }
        hmirror
    }

    fn mirror_score(&self) -> u64 {
        let lcols = self.perfect_vmirror.unwrap_or(0) as u64;
        let arows = self.perfect_hmirror.unwrap_or(0) as u64;
        lcols + (arows * 100)
    }

    fn smudged_mirror_score(&self) -> u64 {
        let lcols = self.smudged_vmirror.unwrap_or(0) as u64;
        let arows = self.smudged_hmirror.unwrap_or(0) as u64;
        lcols + (arows * 100)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Ash,
    Rocks,
}

impl Cell {
    fn from_char(ch: char) -> Self {
        match ch {
            '.' => Self::Ash,
            '#' => Self::Rocks,
            _ => unreachable!(),
        }
    }
}

fn part1(input: &[Mountains]) -> u64 {
    input.iter().map(Mountains::mirror_score).sum()
}

fn part2(input: &[Mountains]) -> u64 {
    input.iter().map(Mountains::smudged_mirror_score).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn testcase1() {
        let input: Vec<Mountains> = input_as_chunks(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 405);
    }

    #[test]
    fn testcase2() {
        let input: Vec<Mountains> = input_as_chunks(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 400);
    }
}
