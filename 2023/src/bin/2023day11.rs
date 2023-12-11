use aoc2023::*;

pub fn main() -> Result<()> {
    let input = read_input(11)?;
    let input = parse_map(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Debug)]
struct Map {
    grid: Vec<String>,
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<bool>,
    empty_cols: Vec<bool>,
}

fn parse_map(input: &str) -> Map {
    let mut grid = Vec::new();
    let mut galaxies = Vec::new();
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();

    for (ri, row) in input.trim().lines().map(str::trim).enumerate() {
        grid.push(row.to_string());
        for (ci, val) in row.bytes().enumerate() {
            if b'#' == val {
                galaxies.push((ri, ci))
            }
        }
    }

    for row in 0..grid.len() {
        empty_rows.push(galaxies.iter().all(|&(ri, _ci)| ri != row));
    }

    for col in 0..grid[0].len() {
        empty_cols.push(galaxies.iter().all(|&(_ri, ci)| ci != col));
    }

    Map {
        grid,
        galaxies,
        empty_rows,
        empty_cols,
    }
}

fn sum_distances(input: &Map, expansion: usize) -> usize {
    let mut total = 0;
    for g1 in 0..(input.galaxies.len() - 1) {
        for g2 in (g1 + 1)..input.galaxies.len() {
            let (mut row1, mut col1) = input.galaxies[g1];
            let (mut row2, mut col2) = input.galaxies[g2];
            if row1 > row2 {
                std::mem::swap(&mut row1, &mut row2);
            }
            if col1 > col2 {
                std::mem::swap(&mut col1, &mut col2);
            }
            let expanded_rows = (row1..=row2).filter(|&row| input.empty_rows[row]).count();
            let expanded_cols = (col1..=col2).filter(|&col| input.empty_cols[col]).count();
            total += (row2 - row1) + (col2 - col1) + ((expanded_rows + expanded_cols) * expansion);
        }
    }
    total
}

fn part1(input: &Map) -> usize {
    sum_distances(input, 1)
}

fn part2(input: &Map) -> usize {
    sum_distances(input, 999_999)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn testcase1_1() {
        let input = parse_map(TEST_INPUT);
        eprintln!("{input:?}");
        assert_eq!(part1(&input), 374);
    }

    #[test]
    fn testcase2() {
        let input = parse_map(TEST_INPUT);
        assert_eq!(sum_distances(&input, 9), 1030);
    }
}
