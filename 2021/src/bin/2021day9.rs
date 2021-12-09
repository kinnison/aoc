use aoc2021::*;

struct LavaTubes {
    map: Vec<u8>,
    width: usize,
    height: usize,
}

impl FromStr for LavaTubes {
    type Err = GenError;

    fn from_str(input: &str) -> Result<LavaTubes> {
        let input = input.trim();
        let mut map = Vec::new();
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        map.extend(std::iter::repeat(255).take(width + 2));
        for line in input.lines() {
            map.push(255);
            for b in line.bytes() {
                map.push(b - b'0');
            }
            map.push(255);
        }
        map.extend(std::iter::repeat(255).take(width + 2));

        Ok(Self { map, width, height })
    }
}
impl LavaTubes {
    fn offset(&self, row: usize, col: usize) -> usize {
        (row * (self.width + 2)) + col
    }

    fn get(&self, row: usize, col: usize) -> u8 {
        self.map[self.offset(row, col)]
    }

    fn risk_value(&self, row: usize, col: usize) -> Option<usize> {
        let val = self.get(row, col);
        if val < self.get(row - 1, col)
            && val < self.get(row + 1, col)
            && val < self.get(row, col - 1)
            && val < self.get(row, col + 1)
        {
            Some(usize::from(val) + 1)
        } else {
            None
        }
    }

    fn low_points(&self) -> impl Iterator<Item = (usize, usize)> {
        (1..=self.height)
            .flat_map(|row| {
                (1..=self.width)
                    .filter_map(move |col| self.risk_value(row, col).map(|_| (row, col)))
            })
            .collect_vec()
            .into_iter()
    }

    #[allow(dead_code)]
    fn show(&self) {
        for row in 1..=self.height {
            for col in 1..=self.width {
                print!("{}", char::from(self.get(row, col) + b'0'));
            }
            println!();
        }
    }

    fn basin_size(&self, row: usize, col: usize) -> usize {
        let mut in_basin = HashSet::new();
        let mut to_process = HashSet::new();
        to_process.insert((row, col));
        while !to_process.is_empty() {
            let (row, col) = to_process.iter().copied().next().unwrap();
            in_basin.insert((row, col));
            to_process.remove(&(row, col));
            let curval = self.get(row, col);
            let nval = self.get(row - 1, col);
            if nval < 9 && nval > curval && !in_basin.contains(&(row - 1, col)) {
                to_process.insert((row - 1, col));
            }
            let nval = self.get(row + 1, col);
            if nval < 9 && nval > curval && !in_basin.contains(&(row + 1, col)) {
                to_process.insert((row + 1, col));
            }
            let nval = self.get(row, col - 1);
            if nval < 9 && nval > curval && !in_basin.contains(&(row, col - 1)) {
                to_process.insert((row, col - 1));
            }
            let nval = self.get(row, col + 1);
            if nval < 9 && nval > curval && !in_basin.contains(&(row, col + 1)) {
                to_process.insert((row, col + 1));
            }
        }
        in_basin.len()
    }
}

fn part1(input: &LavaTubes) -> usize {
    (1..=input.height)
        .flat_map(|row| (1..=input.width).filter_map(move |col| input.risk_value(row, col)))
        .sum()
}

fn part2(input: &LavaTubes) -> usize {
    // The implication from the question is that each low point has its own isolated basin,
    // so treat the inputs as such
    // Step 1, compute the sizes of all the basins
    let mut sizes = input
        .low_points()
        .map(|(row, col)| input.basin_size(row, col))
        .collect_vec();
    // We want the three largest, so sort the vec backwards
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes.into_iter().take(3).product()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn testcase1() {
        let input = LavaTubes::from_str(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn testcase2() {
        let input = LavaTubes::from_str(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 1134);
    }
}

fn main() -> Result<()> {
    let input = read_input(9)?;
    let input = LavaTubes::from_str(&input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
