use aoc2021::*;

#[derive(Clone)]
struct Input {
    algo: [bool; 512],
    image: HashSet<(i32, i32)>,
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
    flipped: bool,
}

impl FromStr for Input {
    type Err = GenError;

    fn from_str(input: &str) -> Result<Self> {
        let mut algo = [false; 512];
        let mut image = HashSet::new();

        let mut input = input.trim().lines().map(str::trim);

        let algo_str = input.next().ok_or("No algorithm?")?;
        algo_str
            .bytes()
            .map(|b| b == b'#')
            .enumerate()
            .for_each(|(i, b)| algo[i] = b);

        if input.next() != Some("") {
            return Err("No blank line?".into());
        }

        let minx = 0;
        let miny = 0;
        let mut maxx = 0;
        let mut maxy = -1;
        for (y, row) in input.enumerate() {
            maxy += 1;
            maxx = (row.len() - 1) as i32;
            row.bytes()
                .map(|b| b == b'#')
                .enumerate()
                .for_each(|(x, b)| {
                    if b {
                        image.insert((x as i32, y as i32));
                    }
                })
        }

        Ok(Self {
            algo,
            image,
            minx,
            maxx,
            miny,
            maxy,
            flipped: algo[0],
        })
    }
}

static KERNEL: [(i32, i32); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

impl Input {
    fn contains(&self, image: &HashSet<(i32, i32)>, coord: (i32, i32)) -> bool {
        image.contains(&coord)
    }
    fn normal_enhance(&mut self) {
        // Apply the kernel from minx-1 to maxx+1, miny-1 to maxy-1
        // i.e. all pixels which could be affected by the image in any way
        let old_image = std::mem::take(&mut self.image);
        let mut new_minx = i32::MAX;
        let mut new_miny = i32::MAX;
        let mut new_maxx = i32::MIN;
        let mut new_maxy = i32::MIN;

        for y in (self.miny - 1)..=(self.maxy + 1) {
            for x in (self.minx - 1)..=(self.maxx + 1) {
                let bit = KERNEL
                    .iter()
                    .copied()
                    .map(|(rx, ry)| self.contains(&old_image, (x + rx, y + ry)))
                    .fold(0, |mut acc, b| {
                        acc <<= 1;
                        if b {
                            acc |= 1
                        }
                        acc
                    });
                let bit = self.algo[bit as usize];
                if bit {
                    new_minx = min(new_minx, x);
                    new_maxx = max(new_maxx, x);
                    new_miny = min(new_miny, y);
                    new_maxy = max(new_maxy, y);
                    self.image.insert((x, y));
                }
            }
        }
        self.minx = new_minx;
        self.maxx = new_maxx;
        self.miny = new_miny;
        self.maxy = new_maxy;
    }

    fn output_idx(in_img: &HashSet<(i32, i32)>, (x, y): (i32, i32)) -> usize {
        KERNEL
            .iter()
            .copied()
            .map(|(rx, ry)| in_img.contains(&(x + rx, y + ry)))
            .fold(0, |mut acc, b| {
                acc <<= 1;
                if b {
                    acc |= 1
                }
                acc
            })
    }

    fn flipped_output_idx(
        prev_in_img: &HashSet<(i32, i32)>,
        in_img: &HashSet<(i32, i32)>,
        (x, y): (i32, i32),
    ) -> usize {
        KERNEL
            .iter()
            .copied()
            .map(|(rx, ry)| {
                let p = (rx + x, ry + y);
                // This pixel is lit if it's already lit
                in_img.contains(&p) || 
                // Or if it would be lit from the previous image
                (Self::output_idx(prev_in_img, p) == 0)
            })
            .fold(0, |mut acc, b| {
                acc <<= 1;
                if b {
                    acc |= 1;
                }
                acc
            })
    }

    fn flipped_enhance_twice(&mut self) {
        let mut tmp = HashSet::new();
        // First enhancement only needs to worry about all the points we *have*
        // So consider each point we have, and then consider all its neighbours
        for (x, y) in self.image.iter().copied() {
            let neighbours = KERNEL
                .iter()
                .copied()
                .map(|(rx, ry)| (x + rx, y + ry))
                .filter(|&p| self.algo[Self::output_idx(&self.image, p)]);
            tmp.extend(neighbours);
        }
        // Now determine the outer edge of the image we got from that enhancement
        let (minx, maxx) = tmp.iter().map(|&(x, _)| x).minmax().into_option().unwrap();
        let (miny, maxy) = tmp.iter().map(|&(_, y)| y).minmax().into_option().unwrap();
        println!("After first enhance, coord ranges are: ({},{}) -> ({},{}) and {} are lit", minx, miny, maxx, maxy, tmp.len());
        let mut out = HashSet::new();
        // Now consider the grid containing everything enclosed within our set.
        for (x, y) in (minx - 1..=maxx + 1).cartesian_product(miny - 1..=maxy + 1) {
            // This time, rather than the simple output_idx() call we need to consider that everything
            // which wasn't affected by a pixel in the first enhance will be flipped now.
            let neighbours = KERNEL
                .iter()
                .copied()
                .map(|(rx, ry)| (x + rx, y + ry))
                .filter(|&p| self.algo[Self::flipped_output_idx(&self.image, &tmp, p)]);
            out.extend(neighbours);
        }
        // Now pop things back in
        drop(std::mem::replace(&mut self.image, out));
    }

    fn enhance_twice(&mut self) {
        if self.flipped {
            self.flipped_enhance_twice();
        } else {
            self.normal_enhance();
            self.normal_enhance();
        }
    }
}

fn part1(input: &Input) -> usize {
    let mut working = input.clone();
    working.enhance_twice();
    working.image.len()
}

fn part2(input: &Input) -> usize {
    let mut working = input.clone();
    for _ in 0..25 {
        working.enhance_twice();
    }
    working.image.len()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
"#;

    #[test]
    fn testcase1() {
        let input = Input::from_str(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn testcase2() {
        let input = Input::from_str(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 3351);
    }

    #[test]
    fn output_idx() {
        let map = HashSet::new();
        assert_eq!(Input::output_idx(&map, (0,0)), 0);
    }

    #[test]
    fn flipped_output_idx() {
        let map = HashSet::new();
        assert_eq!(Input::flipped_output_idx(&map, &map, (0,0)), 511);
    }}

fn main() -> Result<()> {
    let input = read_input(20)?;
    let input = Input::from_str(&input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
