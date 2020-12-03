use aoc2020::*;

struct MapSegment {
    height: usize,
    width: usize,
    rows: Vec<Vec<bool>>,
}

impl MapSegment {
    fn new(input: &str) -> Self {
        let mut ret = MapSegment {
            height: 0,
            width: 0,
            rows: Vec::new(),
        };
        for l in input.lines() {
            let l = l.trim();
            ret.rows.push(l.chars().map(|c| c == '#').collect());
        }
        ret.height = ret.rows.len();
        ret.width = ret.rows[0].len();
        ret
    }

    fn is_tree(&self, row: usize, col: usize) -> bool {
        let col = col % self.width;
        self.rows[row][col]
    }

    fn off_bottom(&self, row: usize) -> bool {
        row >= self.height
    }

    fn test_slope(&self, right: usize, down: usize) -> usize {
        let mut row = 0;
        let mut col = 0;
        let mut trees = 0;
        while !self.off_bottom(row) {
            if self.is_tree(row, col) {
                trees += 1;
            }
            row += down;
            col += right;
        }

        trees
    }
}

fn part1(input: &MapSegment) -> usize {
    // We're traversing right 3 down 1 until we finish
    input.test_slope(3, 1)
}

fn part2(input: &MapSegment) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .copied()
        .map(|(right, down)| input.test_slope(right, down))
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

    #[test]
    fn testcase1() {
        let map = MapSegment::new(&TEST_INPUT);
        assert_eq!(part1(&map), 7)
    }

    #[test]
    fn testcase2() {
        let map = MapSegment::new(&TEST_INPUT);
        assert_eq!(part2(&map), 336)
    }
}

fn main() -> Result<()> {
    let input: String = read_input(3)?;
    let input = MapSegment::new(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
