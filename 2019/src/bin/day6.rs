use aoc2019::*;

#[derive(Debug, ParseByRegex)]
#[regex = r"(?P<center>[^\)]+)\)(?P<satellite>.+)"]
struct Orbit {
    center: String,
    satellite: String,
}

fn part1(orbits: &[Orbit]) -> usize {
    let mut depth = HashMap::new();
    depth.insert("COM".to_owned(), 0);
    while depth.len() < orbits.len() {
        for orbit in orbits {
            if !depth.contains_key(&orbit.satellite) {
                if let Some(n) = depth.get(&orbit.center).copied() {
                    depth.insert(orbit.satellite.clone(), n + 1);
                }
            }
        }
    }

    depth.values().sum()
}

fn find_route<'a>(orbits: &'a [Orbit], from: &str) -> Vec<&'a str> {
    let mut loc = from;
    let mut ret = Vec::new();
    while loc != "COM" {
        for orbit in orbits {
            if orbit.satellite == loc {
                ret.push(orbit.center.as_ref());
                loc = &orbit.center;
            }
        }
    }
    ret
}

fn part2(orbits: &[Orbit]) -> usize {
    let my_route = find_route(orbits, "YOU");
    let santa_route = find_route(orbits, "SAN");
    let santa_map: HashSet<&str> = santa_route.iter().copied().collect();
    for (mydepth, mypos) in my_route.iter().enumerate() {
        if santa_map.contains(mypos) {
            for (santadepth, santapos) in santa_route.iter().enumerate() {
                if mypos == santapos {
                    return mydepth + santadepth;
                }
            }
        }
    }
    0
}

fn main() -> Result<()> {
    let input: Vec<Orbit> = read_input_as_vec(6)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
