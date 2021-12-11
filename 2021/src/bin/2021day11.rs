#![allow(clippy::manual_range_contains)]
use aoc2021::*;

#[derive(Clone)]
struct Octopodes {
    energy: [u8; 100],
}

impl FromStr for Octopodes {
    type Err = GenError;

    fn from_str(input: &str) -> Result<Self> {
        let input = input.trim();
        let mut energy = [0; 100];
        let mut idx = 0;
        for b in input.bytes() {
            match b {
                b if b >= b'0' && b <= b'9' => {
                    energy[idx] = b - b'0';
                    idx += 1;
                }
                _ => {}
            }
        }
        Ok(Self { energy })
    }
}

impl Octopodes {
    fn index(row: i32, col: i32) -> Option<usize> {
        if row < 0 || row > 9 || col < 0 || col > 9 {
            None
        } else {
            Some(((row as usize) * 10) + (col as usize))
        }
    }

    fn index_unchecked(row: i32, col: i32) -> usize {
        ((row as usize) * 10) + (col as usize)
    }

    fn around(row: i32, col: i32) -> impl Iterator<Item = usize> {
        (-1..=1).flat_map(move |row_ofs| {
            (-1..=1).flat_map(move |col_ofs| Octopodes::index(row + row_ofs, col + col_ofs))
        })
    }

    fn step(&mut self) -> usize {
        let mut flashed = HashSet::new();
        let mut to_flash = HashSet::new();
        let mut idx = 0;
        for row in 0..10 {
            for col in 0..10 {
                self.energy[idx] += 1;
                if self.energy[idx] > 9 {
                    to_flash.insert((row, col));
                }
                idx += 1;
            }
        }
        // Energy gains primed, ready for flashing step...
        while !to_flash.is_empty() {
            let (row, col) = to_flash.iter().copied().next().unwrap();
            to_flash.remove(&(row, col));
            flashed.insert((row, col));
            for nidx in Self::around(row, col) {
                if self.energy[nidx] < 10 {
                    self.energy[nidx] += 1;
                    if self.energy[nidx] > 9 {
                        to_flash.insert(((nidx / 10) as i32, (nidx % 10) as i32));
                    }
                }
            }
        }
        // Finally reset the flashed octopodes
        for (row, col) in flashed.iter().copied() {
            self.energy[Self::index_unchecked(row, col)] = 0;
        }

        flashed.len()
    }
}

fn part1(input: &Octopodes) -> usize {
    let mut input = input.clone();
    (0..100).map(|_| input.step()).sum()
}

fn part2(input: &Octopodes) -> usize {
    let mut input = input.clone();
    let mut idx = 1;
    loop {
        if input.step() == 100 {
            break idx;
        }
        idx += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

    #[test]
    fn testcase1() {
        let input = Octopodes::from_str(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 1656);
    }

    #[test]
    fn testcase2() {
        let input = Octopodes::from_str(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 195);
    }
}

fn main() -> Result<()> {
    let input = read_input(11)?;
    let input = Octopodes::from_str(&input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
