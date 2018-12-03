use aoc2018::*;

struct Claim {
    pub id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn try_from<S: AsRef<str>>(input: S) -> Result<Claim> {
        lazy_static! {
            static ref PARSE: Regex =
                Regex::new("^#([^ ]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)$")
                    .expect("Compile error in Regular Expression");
        }
        if let Some(cap) = PARSE.captures(input.as_ref()) {
            let id = cap.get(1).ok_or("No id?")?.as_str().parse()?;
            let x = cap.get(2).ok_or("No x?")?.as_str().parse()?;
            let y = cap.get(3).ok_or("No y?")?.as_str().parse()?;
            let width = cap.get(4).ok_or("No width?")?.as_str().parse()?;
            let height = cap.get(5).ok_or("No height?")?.as_str().parse()?;
            Ok(Claim {
                id,
                x,
                y,
                width,
                height,
            })
        } else {
            Err(format!("Unable to parse '{}'", input.as_ref()))?
        }
    }

    fn all_coords(&self) -> impl Iterator<Item = (usize, usize)> {
        let xs = self.x..(self.x + self.width);
        let ys = self.y..(self.y + self.height);
        xs.flat_map(move |x| ys.clone().map(move |y| (x, y)))
    }
}

fn part1(input: &[Claim]) -> usize {
    let mut seen = HashMap::new();
    for claim in input {
        for coord in claim.all_coords() {
            //println!("ID #{} claiming coordinate {:?}", claim.id, coord);
            *seen.entry(coord).or_insert(0) += 1;
        }
    }
    seen.values().filter(|&v| *v > 1).count()
}

fn part2(input: &[Claim]) -> Result<usize> {
    let mut seen = HashMap::new();
    // Fill the map again
    for claim in input {
        for coord in claim.all_coords() {
            *seen.entry(coord).or_insert(0) += 1;
        }
    }
    // Now scan for a claim whose coordinates all have exactly one claim
    'claims: for claim in input {
        for coord in claim.all_coords() {
            if *seen.get(&coord).ok_or("Missing coordinate!")? != 1 {
                continue 'claims;
            }
        }
        return Ok(claim.id);
    }
    Err("Unable to find a non-overlapping claim!")?
}

fn main() -> Result<()> {
    let test_input: Result<Vec<Claim>> = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
        .iter()
        .map(Claim::try_from)
        .collect();
    let test_input = test_input?;
    println!("Test 1: {}", part1(&test_input));
    println!("Test 2: {}", part2(&test_input)?);
    let input = read_input(3)?;
    let input: Result<Vec<Claim>> = input.lines().map(Claim::try_from).collect();
    let input = input?;
    println!("Loaded {} claims", input.len());
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}
