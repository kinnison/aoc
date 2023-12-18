use aoc2023::*;

pub fn main() -> Result<()> {
    let input: Vec<Instruction> = read_input_as_vec(18)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(input: &[Instruction]) -> i64 {
    let plan = DigPlan::from_instrs(input);
    plan.area()
}

fn part2(input: &[Instruction]) -> i64 {
    let input = input.iter().map(|i| i.transpose()).collect_vec();
    let plan = DigPlan::from_instrs(&input);
    plan.area()
}

#[derive(Debug, ParseByRegex)]
#[regex = r"(?P<dir>.) (?P<dist>\d+) \(\#(?P<hex>......)\)"]
struct Instruction {
    dir: Facing,
    dist: i32,
    hex: String,
}

impl Instruction {
    fn transpose(&self) -> Self {
        let dist: String = self.hex.chars().take(5).collect();
        let dist = i32::from_str_radix(&dist, 16).unwrap();
        let dir = match self.hex.chars().nth(5).unwrap() {
            '0' => Facing::East,
            '1' => Facing::South,
            '2' => Facing::West,
            '3' => Facing::North,
            _ => unreachable!(),
        };
        Self {
            dir,
            dist,
            hex: String::new(),
        }
    }
}

struct DigPlan {
    points: Vec<(i64, i64)>,
}

impl DigPlan {
    fn from_instrs(input: &[Instruction]) -> Self {
        // Instead of trying to store a grid, instead store the points of the polygon
        // This is in service of trying Gauss's area formula (shoelace formula)
        let mut points = vec![(0, 0)];

        let mut row = 0;
        let mut col = 0;

        for instr in input {
            let (rofs, cofs) = instr.dir.row_col_offset();
            row += (rofs * instr.dist) as i64;
            col += (cofs * instr.dist) as i64;

            points.push((row, col));
        }

        // For the area formula, return to zero,zero
        points.push((0, 0));

        Self { points }
    }

    fn gauss_area(&self) -> i64 {
        // Perform Gauss's formula
        // from https://en.m.wikipedia.org/wiki/Shoelace_formula
        let mut area = 0;
        for i in 0..(self.points.len() - 1) {
            let j = i + 1;
            area += self.points[i].0 * self.points[j].1;
            area -= self.points[i].1 * self.points[j].0;
        }

        area.abs() / 2
    }

    fn area(&self) -> i64 {
        // Because our coordinates are meant to be *inclusive* and
        // Gauss' formula isn't, we need to add half the perimeter
        // as well.
        // Finally, add one because apparently everything is off by one?
        self.gauss_area() + (self.perimeter() / 2) + 1
    }

    fn perimeter(&self) -> i64 {
        let mut perim = 0;
        for i in 0..(self.points.len() - 1) {
            let j = i + 1;
            // We add the manhattan distance between the points, since each
            // line is rectilinear this is the real perimeter
            perim += self.points[i].0.abs_diff(self.points[j].0);
            perim += self.points[i].1.abs_diff(self.points[j].1);
        }
        perim as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)
    "#;

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        println!("{input:?}");
        assert_eq!(part1(&input), 62);
    }

    #[test]
    fn testcase2() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 952_408_144_115);
    }
}
