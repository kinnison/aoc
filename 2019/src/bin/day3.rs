use aoc2019::*;

#[derive(Debug, ParseByRegex)]
enum WireStep {
    #[regex = r"U(\d+)"]
    Up(i32),
    #[regex = r"D(\d+)"]
    Down(i32),
    #[regex = r"L(\d+)"]
    Left(i32),
    #[regex = r"R(\d+)"]
    Right(i32),
}

impl WireStep {
    pub fn points_from(&self, start: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
        let (xstep, ystep, count) = match self {
            Self::Up(n) => (0, -1, n),
            Self::Down(n) => (0, 1, n),
            Self::Left(n) => (-1, 0, n),
            Self::Right(n) => (1, 0, n),
        };
        let count: usize = *count as usize;
        std::iter::successors(Some(start), move |(x, y)| Some((x + xstep, y + ystep)))
            .skip(1)
            .take(count)
    }
}

#[derive(Debug)]
struct Wire {
    steps: Vec<WireStep>,
}

impl Wire {
    pub fn from_steps(steps: Vec<WireStep>) -> Self {
        Self { steps }
    }

    pub fn all_points(&self) -> HashSet<(i32, i32)> {
        let mut here = (0, 0);
        let mut ret = HashSet::new();
        for step in &self.steps {
            for point in step.points_from(here) {
                here = point;
                ret.insert(here);
            }
        }
        ret
    }

    pub fn overlap_distance(&self, other: &Wire) -> i32 {
        let mypoints = self.all_points();
        let theirpoints = other.all_points();
        let matches = mypoints.intersection(&theirpoints);
        let mut closest = std::i32::MAX;
        for (x, y) in matches {
            let distance = x.abs() + y.abs();
            if distance < closest {
                closest = distance;
            }
        }
        closest
    }

    pub fn all_points_with_distance(&self) -> HashMap<(i32, i32), usize> {
        let mut here = (0, 0);
        let mut distance = 0;
        let mut ret = HashMap::new();
        for step in &self.steps {
            for point in step.points_from(here) {
                here = point;
                distance += 1;
                ret.entry(here).or_insert(distance);
            }
        }
        ret
    }

    pub fn stepwise_overlap_distance(&self, other: &Wire) -> usize {
        let mypoints = self.all_points_with_distance();
        let otherpoints = other.all_points_with_distance();
        let mypoints_set: HashSet<(i32, i32)> = mypoints.keys().copied().collect();
        let otherpoints_set: HashSet<(i32, i32)> = otherpoints.keys().copied().collect();
        let overlap = mypoints_set.intersection(&otherpoints_set);
        let mut distance = std::usize::MAX;
        for point in overlap {
            let mysteps = mypoints[point];
            let othersteps = otherpoints[point];
            if (mysteps + othersteps) < distance {
                distance = mysteps + othersteps;
            }
        }
        distance
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_case_1_1() {
        let input: Vec<Vec<WireStep>> = input_as_lists(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83",
        )
        .unwrap();
        let wires: Vec<Wire> = input.into_iter().map(Wire::from_steps).collect();
        assert_eq!(wires.len(), 2);
        assert_eq!(wires[0].overlap_distance(&wires[1]), 159);
    }
    #[test]
    fn test_case_1_2() {
        let input: Vec<Vec<WireStep>> = input_as_lists(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
            U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        )
        .unwrap();
        let wires: Vec<Wire> = input.into_iter().map(Wire::from_steps).collect();
        assert_eq!(wires.len(), 2);
        assert_eq!(wires[0].overlap_distance(&wires[1]), 135);
    }
    #[test]
    fn test_case_2_1() {
        let input: Vec<Vec<WireStep>> = input_as_lists(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83",
        )
        .unwrap();
        let wires: Vec<Wire> = input.into_iter().map(Wire::from_steps).collect();
        assert_eq!(wires.len(), 2);
        assert_eq!(wires[0].stepwise_overlap_distance(&wires[1]), 610);
    }
    #[test]
    fn test_case_2_2() {
        let input: Vec<Vec<WireStep>> = input_as_lists(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
            U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        )
        .unwrap();
        let wires: Vec<Wire> = input.into_iter().map(Wire::from_steps).collect();
        assert_eq!(wires.len(), 2);
        assert_eq!(wires[0].stepwise_overlap_distance(&wires[1]), 410);
    }
}

fn part1(wires: &[Wire]) -> i32 {
    wires[0].overlap_distance(&wires[1])
}

fn part2(wires: &[Wire]) -> usize {
    wires[0].stepwise_overlap_distance(&wires[1])
}

fn main() -> Result<()> {
    let input: Vec<Vec<WireStep>> = read_input_as_lists(3)?;
    let input: Vec<Wire> = input.into_iter().map(Wire::from_steps).collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 1: {}", part2(&input));
    Ok(())
}
