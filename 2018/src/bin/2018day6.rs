use aoc2018::*;

#[derive(ParseByRegex, Copy, Clone, Debug)]
#[regex = r"^(?P<x>\d+), (?P<y>\d+)"]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    points: Vec<Point>,
    width: usize,
    height: usize,
    painted: Option<Vec<Option<usize>>>,
}

fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    (if x1 > x2 { x1 - x2 } else { x2 - x1 }) + (if y1 > y2 { y1 - y2 } else { y2 - y1 })
}

impl Grid {
    fn new(points: &[Point]) -> Grid {
        let mut minx = std::usize::MAX;
        let mut miny = std::usize::MAX;
        let mut maxx = std::usize::MIN;
        let mut maxy = std::usize::MIN;
        for p in points {
            minx = min(minx, p.x);
            miny = min(miny, p.y);
            maxx = max(maxx, p.x);
            maxy = max(maxy, p.y);
        }
        Grid {
            points: points
                .iter()
                .map(|p| Point {
                    x: p.x - minx + 1,
                    y: p.y - miny + 1,
                })
                .collect(),
            width: maxx - minx + 3,
            height: maxy - miny + 3,
            painted: None,
        }
    }

    fn paint(&mut self) {
        let mut painted = Vec::with_capacity(self.width * self.height);
        painted.resize(self.width * self.height, None);
        // Find nearest point for the grid
        for x in 0..self.width {
            for y in 0..self.height {
                painted[x + (y * self.width)] = self.find_nearest(x, y);
            }
        }
        // Now, for each point on the edges of the grid, find the set of
        // points to erase from consideration
        let mut to_erase = HashSet::new();
        for x in 0..self.width {
            if let Some(n) = painted[x] {
                to_erase.insert(n);
            }
            if let Some(n) = painted[x + (self.width * (self.height - 1))] {
                to_erase.insert(n);
            }
        }
        for y in 0..self.height {
            if let Some(n) = painted[y * self.width] {
                to_erase.insert(n);
            }
            if let Some(n) = painted[y * self.width + self.width - 1] {
                to_erase.insert(n);
            }
        }

        for entry in painted.iter_mut() {
            if let Some(n) = entry {
                if to_erase.contains(&n) {
                    *entry = None;
                }
            }
        }
        // Now the only painted cells are those which are finite and close
        // to a point uniquely.
        self.painted = Some(painted);
    }

    fn find_nearest(&self, x: usize, y: usize) -> Option<usize> {
        let mut nearest = None;
        let mut nearest_dist = std::usize::MAX;
        for (n, p) in self.points.iter().enumerate() {
            let dist = manhattan_distance(x, y, p.x, p.y);
            if dist == nearest_dist {
                nearest = None;
            } else if dist < nearest_dist {
                nearest = Some(n);
                nearest_dist = dist;
            }
        }
        nearest
    }

    fn biggest_finite(&self) -> Option<(usize, usize)> {
        let mut counts = HashMap::new();
        let painted = self
            .painted
            .as_ref()
            .expect("Cannot count an unpainted grid");
        for cell in painted.iter() {
            if let Some(n) = cell {
                *counts.entry(n).or_insert(0) += 1;
            }
        }
        let mut biggest: Option<usize> = None;
        let mut biggest_size = 0;
        for (n, c) in counts.into_iter() {
            if c > biggest_size {
                biggest = Some(*n);
                biggest_size = c;
            }
        }
        biggest.map(|n| (n, biggest_size))
    }

    fn count_totals_below(&self, v: usize) -> usize {
        // Goal: count the number of coordinates whose total manhattan distance
        // is below v
        let mut count = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                let mut thistot = 0;
                for pt in &self.points {
                    thistot += manhattan_distance(x, y, pt.x, pt.y);
                }
                if thistot < v {
                    count += 1;
                }
            }
        }
        count
    }
}

fn part1(input: &[Point]) -> Result<usize> {
    let mut grid = Grid::new(input);
    grid.paint();
    let (_idx, count) = grid.biggest_finite().ok_or("Unusual!")?;
    Ok(count)
}

fn part2(input: &[Point], maxtot: usize) -> usize {
    let grid = Grid::new(input);
    grid.count_totals_below(maxtot)
}

fn main() -> Result<()> {
    let test_input: Vec<Point> = input_as_vec(
        r#"
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
    "#,
    )?;
    let input: Vec<Point> = read_input_as_vec(6)?;
    println!("Loaded {} points in the test grid", test_input.len());
    println!("Loaded {} points from puzzle input", input.len());
    println!("Test 1: {}", part1(&test_input)?);
    println!("Test 2: {}", part2(&test_input, 32));
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input, 10000));
    Ok(())
}
