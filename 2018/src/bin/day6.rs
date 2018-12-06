use aoc2018::*;

#[derive(ParseByRegex, Copy, Clone)]
#[regex = r"^(?P<x>\d+), (?P<y>\d+)"]
struct Point {
    x: usize,
    y: usize,
}

struct Grid {
    points: Vec<Point>,
}

impl Grid {
    fn new(points: &Vec<Point>) -> Grid {
        let (minx, miny) = points
            .iter()
            .fold((std::usize::MAX, std::usize::MAX), |(minx, miny), p| {
                (std::cmp::min(minx, p.x), std::cmp::min(miny, p.y))
            });
        Grid {
            points: points
                .iter()
                .map(|p| Point {
                    x: p.x - minx + 1,
                    y: p.y - miny + 1,
                })
                .collect(),
        }
    }
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
    Ok(())
}
