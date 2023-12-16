use aoc2023::*;

use rayon::prelude::*;

pub fn main() -> Result<()> {
    let input = read_input(16)?;
    let input = LightMap::from_str(&input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(input: &LightMap) -> usize {
    input.energised(1, 1, Facing::East)
}

fn part2(input: &LightMap) -> usize {
    let width = (input.grid[0].len() - 2) as i32;
    let height = (input.grid.len() - 2) as i32;

    let mut starts = Vec::new();

    for row in 1..=height {
        starts.push((row, 1, Facing::East));
        starts.push((row, width, Facing::West));
    }

    for col in 1..=width {
        starts.push((1, col, Facing::South));
        starts.push((height, col, Facing::North));
    }

    starts
        .into_par_iter()
        .map(|(srow, scol, sdir)| input.energised(srow, scol, sdir))
        .max()
        .unwrap()
}

#[derive(Debug, Clone)]
struct LightMap {
    grid: Vec<Vec<Tile>>,
}

impl FromStr for LightMap {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let igrid = s
            .trim()
            .lines()
            .map(|s| s.trim().chars().map(Tile::from_char).collect_vec())
            .collect_vec();
        let mut grid = vec![];

        grid.push(vec![GridEdge; igrid[0].len() + 2]);
        for row in igrid {
            grid.push(
                Some(GridEdge)
                    .into_iter()
                    .chain(row)
                    .chain(Some(GridEdge))
                    .collect_vec(),
            );
        }
        grid.push(vec![GridEdge; grid[0].len()]);
        Ok(Self { grid })
    }
}

impl LightMap {
    fn at(&self, row: i32, col: i32) -> Tile {
        let row = row as usize;
        let col = col as usize;
        self.grid[row][col]
    }

    fn energised(&self, srow: i32, scol: i32, sdir: Facing) -> usize {
        let mut lightbeams: Vec<(i32, i32, Facing)> = vec![(srow, scol, sdir)];
        let mut seen: HashSet<(i32, i32, Facing)> = HashSet::new();
        let mut energised = HashSet::new();

        while !lightbeams.is_empty() {
            for beam @ (row, col, travelling) in std::mem::take(&mut lightbeams) {
                if seen.contains(&beam) {
                    // If we've already seen this beam, continue
                    continue;
                }
                // And if it's grid edge, continue
                let tile = self.at(row, col);
                if tile == GridEdge {
                    continue;
                }
                // Okay we've not seen this beam before which means we get to energise this cell
                energised.insert((row, col));
                let (beam1, beam2) = tile.process_incoming(travelling);
                if let Some(beam1) = beam1 {
                    let (nrow, ncol) = beam1.do_row_col_move(row, col);
                    lightbeams.push((nrow, ncol, beam1));
                }
                if let Some(beam2) = beam2 {
                    let (nrow, ncol) = beam2.do_row_col_move(row, col);
                    lightbeams.push((nrow, ncol, beam2));
                }
                // Finally, we've seen this lightbeam, so if we see it again we can ignore it
                seen.insert(beam);
            }
        }

        energised.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    GridEdge,
    Empty,
    VSplit,
    HSplit,
    PosMirror,
    NegMirror,
}

use rayon::prelude::IntoParallelIterator;
use Tile::*;

impl Tile {
    fn from_char(ch: char) -> Self {
        match ch {
            '.' => Empty,
            '|' => VSplit,
            '-' => HSplit,
            '/' => PosMirror,
            '\\' => NegMirror,
            _ => unreachable!(),
        }
    }

    fn process_incoming(self, travelling: Facing) -> (Option<Facing>, Option<Facing>) {
        use Facing::*;
        match (self, travelling) {
            (GridEdge, _) => (None, None),
            (Empty, _) => (Some(travelling), None),
            (VSplit, North | South) => (Some(travelling), None),
            (VSplit, East | West) => (Some(North), Some(South)),
            (HSplit, East | West) => (Some(travelling), None),
            (HSplit, North | South) => (Some(East), Some(West)),
            (PosMirror, East) => (Some(North), None),
            (PosMirror, West) => (Some(South), None),
            (PosMirror, North) => (Some(East), None),
            (PosMirror, South) => (Some(West), None),
            (NegMirror, East) => (Some(South), None),
            (NegMirror, South) => (Some(East), None),
            (NegMirror, North) => (Some(West), None),
            (NegMirror, West) => (Some(North), None),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#".|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|....
    "#;

    #[test]
    fn testcase1() {
        let input = LightMap::from_str(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 46);
    }

    #[test]
    fn testcase2() {
        let input = LightMap::from_str(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 51);
    }
}
