use aoc2021::*;

#[derive(ParseByRegex, Clone, Copy)]
enum Fold {
    #[regex = r"^fold along y=(.+)$"]
    Horiz(i32),
    #[regex = r"^fold along x=(.+)$"]
    Vert(i32),
}

#[derive(Clone)]
struct Instructions {
    dots: HashSet<(i32, i32)>,
    folds: Vec<Fold>,
}

impl FromStr for Instructions {
    type Err = GenError;

    fn from_str(input: &str) -> Result<Self> {
        let mut dots = HashSet::new();
        let mut input = input.trim().lines();
        while let Some(line) = input.next() {
            if line.is_empty() {
                break;
            }
            let nums: Vec<i32> = input_by_split_pat(line, ",")?;
            assert_eq!(nums.len(), 2);
            dots.insert((nums[0], nums[1]));
        }
        let mut folds = Vec::new();
        while let Some(line) = input.next() {
            folds.push(Fold::parse_by_regex(line)?);
        }

        Ok(Self { dots, folds })
    }
}

impl Instructions {
    fn do_fold(&mut self, fold: Fold) {
        let newdots = Self::do_one_fold(std::mem::take(&mut self.dots), fold);
        self.dots = newdots;
    }

    fn do_one_fold(dots: HashSet<(i32, i32)>, fold: Fold) -> HashSet<(i32, i32)> {
        dots.into_iter()
            .map(|(x, y)| match fold {
                Fold::Horiz(fy) => {
                    if y <= fy {
                        (x, y)
                    } else {
                        (x, fy - (y - fy))
                    }
                }
                Fold::Vert(fx) => {
                    if x <= fx {
                        (x, y)
                    } else {
                        (fx - (x - fx), y)
                    }
                }
            })
            .collect()
    }

    fn do_all_folds(&mut self) {
        let mut dots = std::mem::take(&mut self.dots);
        for f in self.folds.iter().copied() {
            let newdots = Self::do_one_fold(std::mem::take(&mut dots), f);
            dots = newdots;
        }
        self.dots = dots;
    }

    fn show(&self) {
        let (maxx, maxy) = self
            .dots
            .iter()
            .copied()
            .fold((0, 0), |(accx, accy), (x, y)| (max(accx, x), max(accy, y)));
        for y in 0..=maxy {
            for x in 0..=maxx {
                if self.dots.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
fn part1(input: &Instructions) -> usize {
    let mut input = input.clone();
    let first_fold = input.folds[0];
    input.do_fold(first_fold);
    input.dots.len()
}

fn part2(input: &Instructions) {
    let mut input = input.clone();
    input.do_all_folds();
    println!();
    input.show();
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#;

    #[test]
    fn testcase1() {
        let input = Instructions::from_str(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 17);
    }
}

fn main() -> Result<()> {
    let input = read_input(13)?;
    let input = Instructions::from_str(&input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2:");
    part2(&input);
    Ok(())
}
