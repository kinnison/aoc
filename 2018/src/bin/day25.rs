use aoc2018::*;

#[derive(ParseByRegex, Copy, Clone, Debug, Default)]
#[regex = r"^ *(?P<x>-?\d+), *(?P<y>-?\d+), *(?P<z>-?\d+), *(?P<t>-?\d+)$"]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    t: i32,
}

impl Point {
    fn manhattan(&self, other: &Point) -> i32 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.t - other.t).abs()
    }

    fn constellated(&self, other: &Point) -> bool {
        self.manhattan(other) <= 3
    }
}

fn part1(input: &[Point]) -> usize {
    let mut unchecked = input.to_vec();
    let mut cur_constellation = Vec::new();
    let mut nconstellations = 0;
    while !unchecked.is_empty() {
        if cur_constellation.is_empty() {
            // The current constellation is empty, pop a point off into it
            // and increment the count
            cur_constellation.push(unchecked.pop().unwrap());
            nconstellations += 1;
            continue;
        }
        let mut drained = false;
        let drainer = std::mem::replace(&mut unchecked, Vec::new());
        for p in drainer.into_iter() {
            if cur_constellation.iter().any(|cp| cp.constellated(&p)) {
                // In the constellation
                cur_constellation.push(p);
                drained = true;
            } else {
                // Not in it, put it back for later consideration
                unchecked.push(p);
            }
        }
        if !drained {
            // We didn't drain any into the constellation, so we need to give
            // up on this constellation now
            cur_constellation.resize(0, Point::default());
        }
    }
    nconstellations
}

static TESTS: &[(&str, usize)] = &[
    (
        r" 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0",
        2,
    ),
    (
        r"-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0",
        4,
    ),
    (
        r"1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2",
        3,
    ),
    (
        r"1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2",
        8,
    ),
];

fn main() -> Result<()> {
    for test in TESTS {
        let test_input: Vec<Point> = input_as_vec(test.0)?;
        assert_eq!(test.1, part1(&test_input));
    }
    let input: Vec<Point> = read_input_as_vec(25)?;
    println!("Part 1: {}", part1(&input));
    Ok(())
}
