use aoc2019::*;

#[derive(Debug, ParseByRegex)]
enum WireStep {
    #[regex = r"U(\d+)"]
    Up(usize),
    #[regex = r"D(\d+)"]
    Down(usize),
    #[regex = r"L(\d+)"]
    Left(usize),
    #[regex = r"R(\d+)"]
    Right(usize),
}

impl WireStep {
    pub fn points_from(&self, start: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
        let (xstep, ystep, count) = match self {
            Self::Up(n) => (0, -1, *n),
            Self::Down(n) => (0, 1, *n),
            Self::Left(n) => (-1, 0, *n),
            Self::Right(n) => (1, 0, *n),
        };
        std::iter::successors(Some(start), move |(x, y)| Some((x + xstep, y + ystep)))
            .skip(1)
            .take(count)
    }
}

#[derive(Debug)]
struct Wire {
    points: Vec<(i32, i32)>,
}

impl Wire {
    pub fn from_steps(steps: Vec<WireStep>) -> Self {
        let mut here = (0, 0);
        let mut points = vec![here];
        for step in steps {
            for point in step.points_from(here) {
                here = point;
                points.push(here);
            }
        }
        Self { points }
    }

    pub fn all_points(&self) -> HashSet<(i32, i32)> {
        self.points.iter().skip(1).copied().collect()
    }

    pub fn overlap_distance(&self, other: &Wire) -> i32 {
        let theirpoints = other.all_points();
        let mut closest = std::i32::MAX;
        for (x, y) in self.points.iter().skip(1).copied() {
            if theirpoints.contains(&(x, y)) {
                let distance = x.abs() + y.abs();
                if distance < closest {
                    closest = distance;
                }
            }
        }
        closest
    }

    pub fn all_points_with_distance(&self) -> HashMap<(i32, i32), usize> {
        self.points
            .iter()
            .copied()
            .enumerate()
            .skip(1)
            .map(|(a, b)| (b, a))
            .collect()
    }

    pub fn stepwise_overlap_distance(&self, other: &Wire) -> usize {
        let otherpoints = other.all_points_with_distance();
        let mut distance = std::usize::MAX;
        for (mysteps, point) in self.points.iter().copied().enumerate().skip(1) {
            if let Some(othersteps) = otherpoints.get(&point) {
                if (mysteps + othersteps) < distance {
                    distance = mysteps + othersteps;
                }
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
