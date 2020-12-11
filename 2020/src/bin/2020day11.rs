use aoc2020::*;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Floor,
            'L' => Cell::Empty,
            '#' => Cell::Occupied,
            _ => unimplemented!(),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Grid {
    rows: Vec<Vec<Cell>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let rows: Vec<Vec<_>> = input
            .lines()
            .map(|s| s.chars().map(Cell::from).collect())
            .collect();
        let width = rows[0].len() as i32;
        let height = rows.len() as i32;
        Self {
            rows,
            width,
            height,
        }
    }

    fn is_occupied(&self, row: i32, col: i32) -> bool {
        if row < 0 || row >= self.height {
            return false;
        }
        if col < 0 || col >= self.width {
            return false;
        }
        matches!(self.rows[row as usize][col as usize], Cell::Occupied)
    }

    fn count_occupied_seats(&self) -> usize {
        let mut count = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                if matches!(self.rows[row as usize][col as usize], Cell::Occupied) {
                    count += 1;
                }
            }
        }
        count
    }

    fn count_occupied_neighbours1(&self, row: i32, col: i32) -> usize {
        (-1..=1)
            .flat_map(|row_ofs| {
                (-1..=1).filter_map(move |col_ofs| {
                    if (row_ofs != 0 || col_ofs != 0)
                        && self.is_occupied(row + row_ofs, col + col_ofs)
                    {
                        Some(())
                    } else {
                        None
                    }
                })
            })
            .count()
    }

    fn look_in_direction(&self, mut row: i32, mut col: i32, row_ofs: i32, col_ofs: i32) -> Cell {
        // Do one step
        //println!(
        //    "Looking from {} {} in direction {} {}",
        //    row, col, row_ofs, col_ofs
        //);
        row += row_ofs;
        col += col_ofs;
        while row >= 0 && col >= 0 && row < self.height && col < self.width {
            //println!(
            //    "Cell at {} {} is {}",
            //    row,
            //    col,
            //    match self.rows[row as usize][col as usize] {
            //        Cell::Occupied => "occupied",
            //        Cell::Empty => "empty",
            //        Cell::Floor => "floor",
            //    }
            //);
            match self.rows[row as usize][col as usize] {
                Cell::Occupied => return Cell::Occupied,
                Cell::Empty => return Cell::Empty,
                Cell::Floor => {}
            }
            row += row_ofs;
            col += col_ofs;
        }
        Cell::Floor
    }

    fn count_occupied_neighbours2(&self, row: i32, col: i32) -> usize {
        let mut count = 0;
        for row_ofs in -1..=1 {
            for col_ofs in -1..=1 {
                if row_ofs != 0 || col_ofs != 0 {
                    let seen = self.look_in_direction(row, col, row_ofs, col_ofs);
                    if matches!(seen, Cell::Occupied) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn step1(&self) -> Grid {
        let mut rows = Vec::new();
        for row in 0..self.height {
            let mut newrow = Vec::new();
            for col in 0..self.width {
                newrow.push(match self.rows[row as usize][col as usize] {
                    Cell::Floor => Cell::Floor, // Floor... Floor never changes
                    Cell::Empty => {
                        if self.count_occupied_neighbours1(row, col) == 0 {
                            Cell::Occupied
                        } else {
                            Cell::Empty
                        }
                    }
                    Cell::Occupied => {
                        if self.count_occupied_neighbours1(row, col) > 3 {
                            Cell::Empty
                        } else {
                            Cell::Occupied
                        }
                    }
                });
            }
            rows.push(newrow);
        }
        Grid {
            rows,
            width: self.width,
            height: self.height,
        }
    }

    fn step2(&self) -> Grid {
        let mut rows = Vec::new();
        for row in 0..self.height {
            let mut newrow = Vec::new();
            for col in 0..self.width {
                newrow.push(match self.rows[row as usize][col as usize] {
                    Cell::Floor => Cell::Floor, // Floor... Floor never changes
                    Cell::Empty => {
                        if self.count_occupied_neighbours2(row, col) == 0 {
                            Cell::Occupied
                        } else {
                            Cell::Empty
                        }
                    }
                    Cell::Occupied => {
                        if self.count_occupied_neighbours2(row, col) > 4 {
                            Cell::Empty
                        } else {
                            Cell::Occupied
                        }
                    }
                });
            }
            rows.push(newrow);
        }
        Grid {
            rows,
            width: self.width,
            height: self.height,
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            for cell in row {
                write!(
                    f,
                    "{}",
                    match *cell {
                        Cell::Floor => '.',
                        Cell::Empty => 'L',
                        Cell::Occupied => '#',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(input: &Grid) -> usize {
    let mut thisgrid = input.clone();
    loop {
        let newgrid = thisgrid.step1();
        if newgrid == thisgrid {
            break newgrid.count_occupied_seats();
        }
        thisgrid = newgrid;
    }
}

fn part2(input: &Grid) -> usize {
    let mut thisgrid = input.clone();
    //println!("{}\n", thisgrid);
    //let mut rounds = 0;
    loop {
        let newgrid = thisgrid.step2();
        //rounds += 1;
        //println!("Round {}:\n{}\n", rounds, newgrid);
        //if rounds == 1 {
        //    for col in 90..newgrid.width {
        //        println!(
        //            "Neighbours of 1 {}: {}",
        //            col,
        //            newgrid.count_occupied_neighbours2(1, col)
        //        );
        //    }
        //}
        if newgrid == thisgrid {
            break newgrid.count_occupied_seats();
        }
        thisgrid = newgrid;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;

    #[test]
    fn testcase1() {
        let input = Grid::new(TEST_INPUT);
        assert_eq!(part1(&input), 37);
    }

    #[test]
    fn testcase2() {
        let input = Grid::new(TEST_INPUT);
        assert_eq!(part2(&input), 26);
    }
}

fn main() -> Result<()> {
    let input: String = read_input(11)?;
    let input = Grid::new(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
