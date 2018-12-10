use aoc2018::*;

#[derive(Debug, Clone, Copy, ParseByRegex)]
#[regex = r#"^position=<\s*(?P<px>-?\d+),\s*(?P<py>-?\d+)> velocity=<\s*(?P<vx>-?\d+),\s*(?P<vy>-?\d+)>$"#]
struct MessagePoint {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

static TEST_INPUT: &str = r#"
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
"#;

#[derive(Clone)]
struct Grid {
    points: Vec<MessagePoint>,
}

impl Grid {
    fn new(input: &[MessagePoint]) -> Grid {
        Grid {
            points: input.to_vec(),
        }
    }

    fn tick(&mut self) {
        for p in self.points.iter_mut() {
            p.px += p.vx;
            p.py += p.vy;
        }
    }

    fn bounds(&self) -> (i32, i32, i32, i32) {
        self.points.iter().fold(
            (std::i32::MAX, std::i32::MAX, std::i32::MIN, std::i32::MIN),
            |bounds, p| {
                (
                    min(bounds.0, p.px),
                    min(bounds.1, p.py),
                    max(bounds.2, p.px),
                    max(bounds.3, p.py),
                )
            },
        )
    }
    fn print_grid(&self) {
        let (minx, miny, maxx, maxy) = self.bounds();
        let mut pointbag = HashSet::new();
        self.points.iter().for_each(|p| {
            pointbag.insert((p.px, p.py));
        });
        for y in miny..=maxy {
            for x in minx..=maxx {
                if pointbag.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn size(&self) -> u64 {
        let (minx, miny, maxx, maxy) = self.bounds();
        ((maxx - minx) as u64) * ((maxy - miny) as u64)
    }
}

fn find_best(input: &[MessagePoint]) {
    let mut grid = Grid::new(input);
    let mut bestsize = grid.size();
    let mut bestgrid = grid.clone();
    let mut besttime = 0;
    let mut seconds = 0;
    loop {
        grid.tick();
        seconds += 1;
        let grsize = grid.size();
        if grsize < bestsize {
            bestsize = grsize;
            bestgrid = grid.clone();
            besttime = seconds;
        } else if grsize > bestsize {
            // We've likely finished, so stop now
            break;
        }
    }
    println!("Best grid is after {} seconds:", besttime);
    bestgrid.print_grid();
}

fn main() -> Result<()> {
    let test_input: Vec<MessagePoint> = input_as_vec(TEST_INPUT)?;
    let input: Vec<MessagePoint> = read_input_as_vec(10)?;
    println!("Test vector gives:");
    find_best(&test_input);
    println!("Problem input gives:");
    find_best(&input);
    Ok(())
}
