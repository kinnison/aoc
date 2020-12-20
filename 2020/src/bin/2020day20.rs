use std::convert::Infallible;

use aoc2020::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct RawTile {
    id: u64,
    data: [bool; 100],
    edges: [u16; 8],
}

impl fmt::Display for RawTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;
        for row in 0..10 {
            for col in 0..10 {
                write!(
                    f,
                    "{}",
                    if self.data[(row * 10) + col] {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl RawTile {
    fn row(data: &[bool; 100], row: usize) -> u16 {
        let mut ret = 0;
        let start = row * 10;
        for ofs in 0..10 {
            ret <<= 1;
            // Bools are defined to be zero or one
            ret |= data[start + ofs] as u16;
        }
        ret
    }

    fn col(data: &[bool; 100], col: usize) -> u16 {
        let mut ret = 0;
        for row in 0..10 {
            ret <<= 1;
            ret |= data[col + (row * 10)] as u16;
        }
        ret
    }

    fn invert(edge: u16) -> u16 {
        // 10 bit reversal...
        let mut ret = 0;
        for bit in 0..10 {
            ret <<= 1;
            ret |= (edge >> bit) & 1;
        }
        ret
    }

    fn top(&self) -> u16 {
        RawTile::row(&self.data, 0)
    }

    fn bottom(&self) -> u16 {
        RawTile::row(&self.data, 9)
    }

    fn left(&self) -> u16 {
        RawTile::col(&self.data, 0)
    }

    fn right(&self) -> u16 {
        RawTile::col(&self.data, 9)
    }

    fn rotate(&mut self) {
        let orig = self.data;
        for row in 0..10 {
            for col in 0..10 {
                self.data[(col * 10) + 9 - row] = orig[(row * 10) + col];
            }
        }
    }

    fn flip(&mut self) {
        let orig = self.data;
        for row in 0..10 {
            for col in 0..10 {
                self.data[(row * 10) + col] = orig[((9 - row) * 10) + col];
            }
        }
    }

    fn pixel(&self, row: usize, col: usize) -> bool {
        self.data[(row * 10) + col]
    }
}

impl FromStr for RawTile {
    type Err = Infallible;

    fn from_str(value: &str) -> StdResult<Self, Self::Err> {
        let value = value.trim();
        let nl = value.find('\n').unwrap();
        let (fl, grid) = value.split_at(nl);
        // fl is 'Tile NNNN:'
        let tnum = &fl[5..9];
        let id = tnum.parse().unwrap();
        let mut data = [false; 100];
        grid.chars()
            .filter(|&c| c == '#' || c == '.')
            .enumerate()
            .for_each(|(n, c)| data[n] = c == '#');
        let mut edges = [
            RawTile::row(&data, 0),
            RawTile::col(&data, 0),
            RawTile::row(&data, 9),
            RawTile::col(&data, 9),
            0,
            0,
            0,
            0,
        ];

        for idx in 0..4 {
            edges[idx + 4] = RawTile::invert(edges[idx]);
        }

        Ok(Self { id, data, edges })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![false; width * height],
        }
    }

    fn monster() -> Self {
        const MONSTER: &str = "                  # #    ##    ##    ### #  #  #  #  #  #   ";
        assert_eq!(MONSTER.len(), 60);
        let mut ret = Grid::new(20, 3);
        MONSTER.chars().enumerate().for_each(|(i, c)| {
            ret.data[i] = c == '#';
        });
        ret
    }

    fn place_tile(&mut self, row: usize, col: usize, tile: &RawTile) {
        for trow in 1..9 {
            for tcol in 1..9 {
                let gridrow = row + trow - 1;
                let gridcol = col + tcol - 1;
                self.data[gridrow * self.width + gridcol] = tile.pixel(trow, tcol);
            }
        }
    }

    fn rotate(&mut self) {
        assert_eq!(self.width, self.height);
        let orig = std::mem::replace(&mut self.data, vec![false; self.width * self.height]);
        for row in 0..self.height {
            for col in 0..self.width {
                let destrow = col;
                let destcol = self.width - 1 - row;
                self.data[(destrow * self.width) + destcol] = orig[(row * self.width) + col];
            }
        }
    }

    fn flip(&mut self) {
        let orig = std::mem::replace(&mut self.data, vec![false; self.width * self.height]);
        for row in 0..self.height {
            for col in 0..self.width {
                self.data[(row * self.width) + col] =
                    orig[((self.height - 1 - row) * self.height) + col];
            }
        }
    }

    fn count_of(&self, other: &Grid) -> usize {
        assert!(self.width >= other.width);
        assert!(self.height >= other.height);
        let mut total = 0;
        for row in 0..=(self.height - other.height) {
            for col in 0..=(self.width - other.width) {
                if self.find_at(row, col, other) {
                    total += 1;
                }
            }
        }
        total
    }

    fn find_at(&self, row: usize, col: usize, other: &Grid) -> bool {
        for orow in 0..other.height {
            for ocol in 0..other.width {
                if other.pixel(orow, ocol) && !self.pixel(row + orow, col + ocol) {
                    return false;
                }
            }
        }
        true
    }

    fn pixel(&self, row: usize, col: usize) -> bool {
        self.data[(row * self.width) + col]
    }

    fn count_set(&self) -> usize {
        self.data.iter().copied().filter(|&b| b).count()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", if self.pixel(row, col) { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn find_corners(input: &[RawTile]) -> [u64; 4] {
    // Our goal is to find four tiles each of which have two unique edges.
    // Since each edge *could* be inverted we effectively look for tiles
    // which have four unique edges.

    // First let's build a map of edge to list of tiles which have it.
    let mut edge_to_tile = vec![Vec::new(); 1024];

    for (idx, tile) in input.iter().enumerate() {
        for edge in tile.edges.iter().copied() {
            edge_to_tile[edge as usize].push(idx)
        }
    }

    // Now we want to invert that into a map of tiles to number of unique
    // edges they hold
    let unique_edges: Vec<_> = input
        .iter()
        .map(|t| {
            t.edges
                .iter()
                .filter(|&&e| edge_to_tile[e as usize].len() == 1)
                .count()
        })
        .collect();

    let only_four = unique_edges
        .into_iter()
        .enumerate()
        .filter_map(|(idx, count)| {
            // idx is the tile inedx in the input
            // count is how many edges didn't have a partner tile
            if count == 4 {
                Some(input[idx].id)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(only_four.len(), 4);

    [only_four[0], only_four[1], only_four[2], only_four[3]]
}

fn part1(input: &[RawTile]) -> u64 {
    let corners = find_corners(input);
    corners.iter().product()
}

fn part2(input: &[RawTile]) -> usize {
    let corners = find_corners(input);
    // We know the four corners of the grid, though we do not know which corner
    // each is in, nor what orientation the grid is in.
    // We're going to have to rotate/flip our grid cells so we'll need copies
    // to mutate
    let mut grids = input.iter().map(|t| (t.id, *t)).collect::<HashMap<_, _>>();
    let zeroid = grids[&corners[0]].id;
    // It's helpful to know what edges map to which tile pairs
    let mut edge_to_tile = vec![Vec::new(); 1024];

    for tile in input.iter() {
        for edge in tile.edges.iter().copied() {
            edge_to_tile[edge as usize].push(tile.id)
        }
    }

    // If we can assert that we either get zero, one or two tiles for this edge
    // pair then we can guarantee to find which tile in which direction with ease
    for eset in edge_to_tile.iter() {
        assert!(eset.len() < 3);
    }

    // Now let's convert those pairs into an adjacency list
    let mut adj: HashMap<u64, HashSet<u64>> = HashMap::new();
    for edge in edge_to_tile.iter() {
        if edge.len() == 2 {
            adj.entry(edge[0]).or_default().insert(edge[1]);
            adj.entry(edge[1]).or_default().insert(edge[0]);
        }
    }

    // We'll represent our larger grid as a pair of maps first
    // Positions are (row,col)
    let mut grid_to_pos = HashMap::new();
    let mut pos_to_grid = HashMap::new();
    // Let's pick the first corner cell from above and start from there
    grid_to_pos.insert(zeroid, (0, 0));
    pos_to_grid.insert((0, 0), zeroid);

    // We want to grow our grid outward from this point, in each direction,
    // if at all possible.
    // We know which tiles we're growing to thanks to the adjacency matrix
    // so now we need to simply work out which direction and orientation for
    // that given tile.
    let mut to_consider = vec![zeroid];
    while !to_consider.is_empty() {
        let thisid = to_consider.pop().unwrap();
        let thistile = &grids[&thisid];
        let thistop = thistile.top();
        let thisbottom = thistile.bottom();
        let thisleft = thistile.left();
        let thisright = thistile.right();
        let thispos = grid_to_pos[&thisid];
        for adjid in adj.get(&thisid).unwrap().iter().copied() {
            if grid_to_pos.contains_key(&adjid) {
                continue;
            }
            // We need to fit adjid next to thisid, to do that we need to determine
            // which edge matches
            let grid = grids.get_mut(&adjid).unwrap();
            for i in 0..8 {
                let found = if thistop == grid.bottom() {
                    // We've found grid above thistile
                    Some((thispos.0 - 1, thispos.1))
                } else if thisbottom == grid.top() {
                    // We've found grid below thistile
                    Some((thispos.0 + 1, thispos.1))
                } else if thisright == grid.left() {
                    // We've found grid right of thistile
                    Some((thispos.0, thispos.1 + 1))
                } else if thisleft == grid.right() {
                    // We've found grid left of thistile
                    Some((thispos.0, thispos.1 - 1))
                } else {
                    None
                };
                if let Some(pos) = found {
                    grid_to_pos.insert(adjid, pos);
                    pos_to_grid.insert(pos, adjid);
                    break;
                }
                grid.rotate();
                if i == 3 {
                    grid.flip();
                }
            }
            // And now we can add adjid to the consider list
            to_consider.push(adjid);
        }
    }
    // In theory we've fit everything together now, so glue the grid together
    let minrow = pos_to_grid
        .keys()
        .copied()
        .map(|(row, _)| row)
        .min()
        .unwrap();
    let maxrow = pos_to_grid
        .keys()
        .copied()
        .map(|(row, _)| row)
        .max()
        .unwrap();
    let mincol = pos_to_grid
        .keys()
        .copied()
        .map(|(_, col)| col)
        .min()
        .unwrap();
    let maxcol = pos_to_grid
        .keys()
        .copied()
        .map(|(_, col)| col)
        .max()
        .unwrap();
    let gridcols = (maxcol - mincol + 1) * 8;
    let gridrows = (maxrow - minrow + 1) * 8;
    let mut grid = Grid::new(gridcols as usize, gridrows as usize);
    for (pos, gridid) in pos_to_grid {
        let tile = &grids[&gridid];
        let tilerow = pos.0 - minrow;
        let tilecol = pos.1 - mincol;
        let gridrow = tilerow * 8;
        let gridcol = tilecol * 8;
        grid.place_tile(gridrow as usize, gridcol as usize, tile);
    }
    grid.rotate();
    let monster = Grid::monster();
    for i in 0..8 {
        let monsters = grid.count_of(&monster);
        if monsters > 0 {
            return grid.count_set() - (monster.count_set() * monsters);
        }
        grid.rotate();
        if i == 3 {
            grid.flip();
        }
    }
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;

    #[test]
    fn testcase1() {
        let input = input_by_split_pat(TEST_INPUT, "\n\n").unwrap();
        assert_eq!(part1(&input), 20899048083289);
    }

    #[test]
    fn testcase2() {
        let input = input_by_split_pat(TEST_INPUT, "\n\n").unwrap();
        assert_eq!(part2(&input), 273);
    }

    #[test]
    fn check_invert() {
        assert_eq!(RawTile::invert(0b_10101_01010), 0b_01010_10101);
    }

    #[test]
    fn check_invariants() {
        let mut input = input_by_split_pat(TEST_INPUT, "\n\n").unwrap();
        let tile: RawTile = input.pop().unwrap();
        let mut rot = tile;
        rot.rotate();
        rot.rotate();
        rot.rotate();
        rot.rotate();
        // Four rotations are good
        assert_eq!(rot, tile);
        rot.flip();
        rot.flip();
        // Two flips are good
        assert_eq!(rot, tile);
        rot.rotate();
        rot.rotate();
        rot.flip();
        rot.rotate();
        rot.rotate();
        rot.flip();
        // Rotate and flip combined is good
        assert_eq!(rot, tile);
        rot.rotate();
        rot.flip();
        rot.rotate();
        rot.flip();
        // Finally check that rotate isn't a diagonal flip
        assert_eq!(rot, tile);
    }

    #[test]
    fn check_grid_invariants() {
        let mut tile = Grid::new(13, 13);
        tile.data
            .iter_mut()
            .enumerate()
            .for_each(|(idx, pixel)| *pixel = (idx % 3) == 1);
        let mut rot = tile.clone();
        rot.rotate();
        rot.rotate();
        rot.rotate();
        rot.rotate();
        // Four rotations are good
        assert_eq!(rot, tile);
        rot.flip();
        rot.flip();
        // Two flips are good
        assert_eq!(rot, tile);
        rot.rotate();
        rot.rotate();
        rot.flip();
        rot.rotate();
        rot.rotate();
        rot.flip();
        // Rotate and flip combined is good
        assert_eq!(rot, tile);
        rot.rotate();
        rot.flip();
        rot.rotate();
        rot.flip();
        // Finally check that rotate isn't a diagonal flip
        assert_eq!(rot, tile);
    }

    #[test]
    fn check_count_monsters() {
        let monster = Grid::monster();
        assert_eq!(monster.count_of(&monster), 1);
        assert_eq!(monster.count_set(), 15);
    }
}

fn main() -> Result<()> {
    let input = read_input_as_vec_split(20, "\n\n")?;
    println!("There are {} tiles", input.len());
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
