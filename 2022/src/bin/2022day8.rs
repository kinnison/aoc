use aoc2022::*;

struct Forest {
    width: usize,
    height: usize,
    rows: Vec<Vec<u8>>,
}

impl<T> From<T> for Forest
where
    T: AsRef<str>,
{
    fn from(input: T) -> Self {
        let rows = input
            .as_ref()
            .trim()
            .lines()
            .map(|l| l.trim().bytes().map(|b| b - b'0').collect_vec())
            .collect_vec();
        let width = rows[0].len();
        let height = rows.len();
        Self {
            width,
            height,
            rows,
        }
    }
}

impl Forest {
    fn is_highest(mut iter: impl Iterator<Item = u8>) -> bool {
        let cur = iter.next().unwrap();
        let max = iter.max().unwrap();
        cur > max
    }

    fn sees(mut iter: impl Iterator<Item = u8>) -> usize {
        let house = iter.next().unwrap();
        let mut count = 0;
        for tree in iter {
            count += 1;
            if tree >= house {
                break;
            }
        }
        count
    }
}
fn part1(input: &Forest) -> usize {
    let mut total = input.width * 2 + input.height * 2 - 4;

    for row in 1..(input.height - 1) {
        for col in 1..(input.width - 1) {
            #[allow(unused_parens)]
            if (
                // left to right
                Forest::is_highest(input.rows[row].iter().copied().skip(col)) ||
                // right to left
                Forest::is_highest(input.rows[row].iter().copied().rev().skip(input.width - col - 1)) ||
                // top to bottom
                Forest::is_highest(input.rows.iter().map(|v|v[col]).skip(row)) ||
                // bottom to top
                Forest::is_highest(input.rows.iter().rev().map(|v|v[col]).skip(input.height-row-1))
            ) {
                total += 1;
            }
        }
    }

    total
}

fn part2(input: &Forest) -> usize {
    let mut best_score = 0;

    for row in 1..(input.height - 1) {
        for col in 1..(input.width - 1) {
            println!("try row={row} col={col}");
            #[allow(unused_parens)]
            let score = (
                // left to right
                Forest::sees(input.rows[row].iter().copied().skip(col)) *
                // right to left
                Forest::sees(input.rows[row].iter().copied().rev().skip(input.width - col - 1)) *
                // top to bottom
                Forest::sees(input.rows.iter().map(|v|v[col]).skip(row)) *
                // bottom to top
                Forest::sees(input.rows.iter().rev().map(|v|v[col]).skip(input.height-row-1))
            );
            println!("row={row} col={col} score={score}");
            if score > best_score {
                best_score = score;
            }
        }
    }

    best_score
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
30373
25512
65332
33549
35390"#;

    #[test]
    fn testcase1() {
        let input = Forest::from(TEST_INPUT);
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn testcase2() {
        let input = Forest::from(TEST_INPUT);
        assert_eq!(part2(&input), 8);
    }
}

fn main() -> Result<()> {
    let input: String = read_input(8)?;
    let input = input.into();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
