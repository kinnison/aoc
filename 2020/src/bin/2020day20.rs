use std::convert::Infallible;

use aoc2020::*;

struct RawTile {
    id: u64,
    data: [bool; 100],
    edges: [u16; 8],
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

fn part1(input: &[RawTile]) -> u64 {
    // Our goal is to find four tiles each of which have two unique edges.
    // Since each edge *could* be inverted we effectively look for tiles
    // which have four unique edges.

    unimplemented!()
}

fn part2(input: &[RawTile]) -> u64 {
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
    fn testcase2() {}

    #[test]
    fn check_invert() {
        assert_eq!(RawTile::invert(0b_10101_01010), 0b_01010_10101);
    }
}

fn main() -> Result<()> {
    let input = read_input_as_vec_split(20, "\n\n")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
