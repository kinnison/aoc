use aoc2018::*;

#[derive(Debug, Copy, Clone, ParseByRegex)]
#[regex = r"^pos=<(?P<x>-?\d+),(?P<y>-?\d+),(?P<z>-?\d+)>, r=(?P<r>\d+)$"]
struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    r: i32,
}

impl Nanobot {
    fn distance_to(&self, other: &Nanobot) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn other_in_range(&self, other: &Nanobot) -> bool {
        self.distance_to(other) <= self.r
    }
}

static TEST_INPUT: &str = r"
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1
";

fn bot_count_in_range(bots: &[Nanobot], chosen: &Nanobot) -> usize {
    bots.iter().filter(|e| chosen.other_in_range(e)).count()
}

fn part1(inputs: &[Nanobot]) -> usize {
    let biggest = inputs.iter().enumerate().fold((0, inputs[0].r), |s, e| {
        if s.1 > e.1.r {
            s
        } else {
            (e.0, e.1.r)
        }
    });

    let chosen = inputs[biggest.0];
    bot_count_in_range(inputs, &chosen)
}

static TEST_INPUT2: &str = r"
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5
";

// I had a naïve solution which would have found the answer in about 2 years.
// I ended up learning about projecting the nanobot's ranges into a 1d space
// defined by the manhattan distance and then looked for overlapping ranges.
// I'm still not convinced I understand *why* this works, but it does.
// Kudos to @tinaun for the insight.
fn part2(inputs: &[Nanobot]) -> i32 {
    // We only care about manhattan distances,
    // since that's what the solution needs.
    // Each bot has a min-manhat and max-manhat from the origin point
    // defined by its location +/- its range
    let ranges = inputs.iter().map(|b| {
        let dist = b.x + b.y + b.z;
        (dist - b.r, dist + b.r + 1) // The +1 puts the bot *just* out of range
    });
    // Given that set of ranges, we want to find the intersections
    // Firstly we count how often a range starts at a given point
    let mut occurrence = HashMap::new();
    for (start, end) in ranges {
        *occurrence.entry(start).or_insert(0) += 1;
        *occurrence.entry(end).or_insert(0) -= 1;
    }
    // Find the most common range start by sorting the map by pos
    let mut poss: Vec<(i32, i32)> = occurrence.drain().collect();
    poss.sort_unstable();
    // And then scanning, maintaining a running total
    let mut total = 0;
    let mut best = 0;
    let mut best_start = 0;
    for (pos, v) in poss.iter() {
        total += *v;
        if total > best {
            best = total;
            best_start = *pos;
        }
    }
    // Finally we find the position immediately after that best start
    let best_end = poss.iter().find(|e| e.0 > best_start).expect("Oddness!").0;
    // The result is one back from the best_end to put it back in the
    // range of the nanobot
    best_end - 1
}

fn main() -> Result<()> {
    let test_input: Vec<Nanobot> = input_as_vec(TEST_INPUT)?;
    println!("Test 1: {}", part1(&test_input));
    let test_input: Vec<Nanobot> = input_as_vec(TEST_INPUT2)?;
    println!("Test 2: {}", part2(&test_input));

    let input: Vec<Nanobot> = read_input_as_vec(23)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
