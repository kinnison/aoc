use aoc2015::*;

struct Map {
    routes: HashMap<String, HashMap<String, usize>>,
}

impl Map {
    fn new(input: &str) -> Map {
        lazy_static! {
            static ref LINE: Regex = Regex::new("^([^ ]+) to ([^ ]+) = ([0-9]+)$").unwrap();
        }

        let mut ret = Map {
            routes: HashMap::new(),
        };

        for line in input.lines() {
            if let Some(cap) = LINE.captures(line) {
                let first = cap.get(1).unwrap().as_str().to_owned();
                let second = cap.get(2).unwrap().as_str().to_owned();
                let dist = cap.get(3).unwrap().as_str().parse().unwrap();
                ret.insert(first, second, dist);
            } else {
                panic!("Unable to parse line: '{}'", line);
            }
        }

        ret
    }

    fn insert(&mut self, first: String, second: String, dist: usize) {
        self.routes
            .entry(first.clone())
            .or_default()
            .insert(second.clone(), dist);
        self.routes.entry(second).or_default().insert(first, dist);
    }

    fn all_locations(&self) -> Vec<&str> {
        let mut ret = Vec::new();
        for k in self.routes.keys() {
            ret.push(k.as_str());
        }
        ret
    }

    fn dist(&self, first: &str, second: &str) -> Option<usize> {
        self.routes.get(first).and_then(|m| m.get(second)).copied()
    }
}

fn distances(input: &Map) -> Vec<usize> {
    let mut locs = input.all_locations();

    let ret = Heap::new(&mut locs);
    let ret = ret.map(|permutation| {
        let mut dist = 0;
        for win in permutation.windows(2) {
            if let Some(step) = input.dist(win[0], win[1]) {
                dist += step;
            } else {
                return None;
            }
        }
        Some(dist)
    });
    let ret = ret.filter(Option::is_some);
    let ret = ret.map(Option::unwrap);
    ret.collect()
}

fn part1(input: &Map) -> usize {
    distances(input)
        .iter()
        .fold(std::usize::MAX, |shortest, dist| shortest.min(*dist))
}

fn part2(input: &Map) -> usize {
    distances(input)
        .iter()
        .fold(std::usize::MIN, |longest, dist| longest.max(*dist))
}

fn main() -> Result<()> {
    let input_map = Map::new(&read_input(9)?);
    println!("Part 1: {}", part1(&input_map));
    println!("Part 2: {}", part2(&input_map));
    Ok(())
}
