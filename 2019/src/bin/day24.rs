use aoc2019::*;

fn bit_for(x: u32, y: u32) -> u32 {
    y * 5 + x
}

// The puzzle hints at the use of a u32 to store our grid
// To do this, we need to calculate the adjacency values.
// In theory this could be constant, but let's compute for now
fn adjacency_bits_part1() -> Vec<u32> {
    let mut ret = Vec::new();
    for y in 0..5i32 {
        for x in 0..5i32 {
            let mut adj = 0u32;
            for pos in &surrounds((x, y)) {
                if pos.0 >= 0 && pos.1 >= 0 && pos.0 <= 4 && pos.1 <= 4 {
                    adj |= 1 << bit_for(pos.0 as u32, pos.1 as u32);
                }
            }
            ret.push(adj);
        }
    }
    ret
}

fn grid_to_u32(input: &str) -> u32 {
    input
        .chars()
        .filter_map(|c| match c {
            '.' => Some(0),
            '#' => Some(1),
            _ => None,
        })
        .rev()
        .fold(0, |acc, bit| (acc << 1) | bit)
}

fn mutate_grid_1(grid: u32, adj_map: &[u32]) -> u32 {
    let mut new_grid = 0;
    for y in 0..5u32 {
        for x in 0..5u32 {
            let bit = bit_for(x, y);
            let adj = adj_map[bit as usize];
            let ones = (grid & adj).count_ones();
            if (grid & (1 << bit)) == 0 {
                // Cell is dead, does it vivify?
                if ones == 1 || ones == 2 {
                    new_grid |= 1 << bit;
                }
            } else {
                // Cell is alive, does it stay that way?
                if ones == 1 {
                    new_grid |= 1 << bit;
                }
            }
        }
    }
    new_grid
}

// Part 1 is the "score" for a grid which in our representation *is* the grid
fn part1(input: u32) -> u32 {
    let mut seen = HashSet::new();
    let mut grid = input;
    let adj_map = adjacency_bits_part1();
    loop {
        if !seen.insert(grid) {
            break grid;
        }
        grid = mutate_grid_1(grid, &adj_map);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn print_grid(grid: u32) {
        for bit in 0..25 {
            if (grid & (1 << bit)) == 0 {
                print!(".");
            } else {
                print!("#");
            }
            if (bit % 5) == 4 {
                println!();
            }
        }
    }

    #[test]
    fn test_1() {
        static GRID: &str = r"....##..#.#..##..#..#....";
        let adj_map = adjacency_bits_part1();
        let mut grid = grid_to_u32(GRID);
        println!("Grid as bits: {:b}", grid);
        println!("Initial grid:");
        print_grid(grid);
        println!();
        for i in 1..=4 {
            println!("After {} minute{}:", i, if i == 1 { "" } else { "s" });
            grid = mutate_grid_1(grid, &adj_map);
            print_grid(grid);
            println!();
        }
        assert_eq!(part1(grid_to_u32(GRID)), 2_129_920);
    }
}

// For part 2 we need to go Plutonian and go up and down levels
// For that reason, adjacency goes up and down, fortunately only in a single
// zooming line.  Tiles on the top row are adjacent to one cell in the level
// above.  Tiles around the middle tile are adjacent to five cells in the
// level below.  Nobody is adjacent to the middle cell which does not exist.
fn adjacency_bits_part2() -> Vec<(u32, u32, u32)> {
    let mut ret = Vec::new();
    for y in 0..5i32 {
        for x in 0..5i32 {
            let mut outer_bits = 0;
            let mut these_bits = 0;
            let mut inner_bits = 0;
            if y != 2 || x != 2 {
                // Not in the center, so first compute these bits
                for pos in &surrounds((x, y)) {
                    if pos.0 >= 0 && pos.1 >= 0 && pos.0 <= 4 && pos.1 <= 4 {
                        these_bits |= 1 << bit_for(pos.0 as u32, pos.1 as u32);
                    }
                }
                // Top row is adjacent to cell 7 of the outer set
                if y == 0 {
                    outer_bits |= 1 << 7;
                }
                // Bottom row is adjacent to cell 17 of the outer set
                if y == 4 {
                    outer_bits |= 1 << 17;
                }
                // Left column is adjacent to cell 11 of the outer set
                if x == 0 {
                    outer_bits |= 1 << 11;
                }
                // Right column is adjacent to cell 13 of the outer set
                if x == 4 {
                    outer_bits |= 1 << 13;
                }
                // Now for the inner set...
                if y == 1 && x == 2 {
                    // Adjacent to the top row of the inner set
                    inner_bits |= 1 + (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4);
                }
                if y == 3 && x == 2 {
                    // Adjacent to the bottom row of the inner set
                    inner_bits |= (1 << 20) + (1 << 21) + (1 << 22) + (1 << 23) + (1 << 24);
                }
                if x == 1 && y == 2 {
                    // Adjacent to the left column of the inner set
                    inner_bits |= 1 + (1 << 5) + (1 << 10) + (1 << 15) + (1 << 20);
                }
                if x == 3 && y == 2 {
                    // Adjacent to the right column of the inner set
                    inner_bits |= (1 << 4) + (1 << 9) + (1 << 14) + (1 << 19) + (1 << 24);
                }
            }
            ret.push((outer_bits, these_bits, inner_bits));
        }
    }
    ret
}

// For part 2 we start with an infinitely empty set except for level 0
// which has our input.  We then process any level which is, or is adjacent to
// a non-zero level.  i.e. each time a level is non-zero, vivify the levels
// above/below it for computation
fn part2(input: u32) -> u32 {
    let adj_map = adjacency_bits_part2();
    let mut levels: HashMap<i32, u32> = HashMap::new();
    levels.insert(0, input);
    levels.insert(-1, 0);
    levels.insert(1, 0);
    for _round in 1..=200 {
        let mut new_levels: HashMap<i32, u32> = HashMap::new();
        for (level, grid) in levels.iter().map(|(x, y)| (*x, *y)) {
            let mut new_grid = 0;
            for y in 0..5u32 {
                for x in 0..5u32 {
                    let bit = bit_for(x, y);
                    let (inner, these, outer) = adj_map[bit as usize];
                    let ones = (grid & these).count_ones()
                        + (levels.get(&(level - 1)).copied().unwrap_or_default() & outer)
                            .count_ones()
                        + (levels.get(&(level + 1)).copied().unwrap_or_default() & inner)
                            .count_ones();
                    if (grid & (1 << bit)) == 0 {
                        // Cell is dead, does it vivify?
                        if ones == 1 || ones == 2 {
                            new_grid |= 1 << bit;
                        }
                    } else {
                        // Cell is alive, does it stay that way?
                        if ones == 1 {
                            new_grid |= 1 << bit;
                        }
                    }
                }
            }
            new_levels.insert(level, new_grid);
            if new_grid != 0 {
                new_levels.entry(level - 1).or_default();
                new_levels.entry(level + 1).or_default();
            }
        }
        levels = new_levels;
    }

    // Now count bugs in all active levels
    let mut bugs = 0;
    for (_, grid) in levels.iter() {
        bugs += grid.count_ones();
    }
    bugs
}

fn main() -> Result<()> {
    let input = read_input(24)?;
    let input = grid_to_u32(&input);

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
    Ok(())
}
