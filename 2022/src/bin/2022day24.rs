use aoc2022::*;

#[derive(Clone, Debug)]
struct Valley {
    starts: HashSet<(char, i32, i32)>,
    width: i32,
    height: i32,
    cycle: i32,
    entrance: (i32, i32),
    exit: (i32, i32),
}

impl<T> From<T> for Valley
where
    T: AsRef<str>,
{
    fn from(input: T) -> Self {
        // Valley maps are always of the form:
        // #.#####
        // #...v.#
        // #..>..#
        // #.....#
        // #.....#
        // #.....#
        // #####.#
        // Though the width and height are variable and the blizzards inside are also
        // variable, there are no internal walls.
        let input = input.as_ref().trim();
        let height = (input.lines().count() - 2) as i32;
        let width = (input.lines().next().unwrap().trim().len() - 2) as i32;
        let starts = input
            .lines()
            .skip(1)
            .take(height as usize)
            .enumerate()
            .flat_map(|(row, l)| {
                l.trim()
                    .chars()
                    .skip(1)
                    .take(width as usize)
                    .enumerate()
                    .flat_map(move |(col, c)| match c {
                        'v' | '^' | '<' | '>' => Some((c, row as i32, col as i32)),
                        _ => None,
                    })
            })
            .collect();

        let cycle = (width as u32).lcm(height as u32) as i32;
        println!("Valley is {width} x {height}, cycling every {cycle}");

        let entrance = (-1, 0);
        let exit = (height, width - 1);

        Self {
            starts,
            width,
            height,
            cycle,
            entrance,
            exit,
        }
    }
}

impl Valley {
    fn dangerous(&self, row: i32, col: i32, time: i32) -> bool {
        // The given row/col/time is dangerous if any blizzard lands in it.
        // Rather than forward-project every blizzard to check, let's back-project what
        // blizzard would be here if there was one, and look for it
        self.starts
            .contains(&('>', row, (col - time).rem_euclid(self.width)))
            || self
                .starts
                .contains(&('<', row, (col + time).rem_euclid(self.width)))
            || self
                .starts
                .contains(&('^', (row + time).rem_euclid(self.height), col))
            || self
                .starts
                .contains(&('v', (row - time).rem_euclid(self.height), col))
    }

    // Trek from start to goal, start is always (-1,0) and goal is always (height,width-1)
    // for part 1.
    fn trek(&self, start: (i32, i32), goal: (i32, i32), tofs: i32) -> i32 {
        let mut queue = VecDeque::new();

        let mut seen = HashSet::new();

        queue.push_back((start.0, start.1, tofs));

        while let Some((row, col, time)) = queue.pop_front() {
            // It is {time} and we are at {row},{col}
            if (row, col) == goal {
                return time;
            }

            let tcoord = (row, col, time % self.cycle);
            if seen.contains(&tcoord) {
                // No point re-trying, we've been here at this point in a cycle before
                continue;
            }
            seen.insert(tcoord);

            // Okay, let's see if we can move around...
            for (rofs, cofs) in [(0, 0), (0, 1), (1, 0), (-1, 0), (0, -1)] {
                let nrow = row + rofs;
                let ncol = col + cofs;
                if nrow < 0 || nrow >= self.height || ncol < 0 || ncol >= self.width {
                    // We're off the blizzard area, so permit only entry/exit
                    if !((nrow == -1 && ncol == 0)
                        || (nrow == self.height && ncol == self.width - 1))
                    {
                        continue;
                    }
                }
                if !self.dangerous(nrow, ncol, time + 1) {
                    queue.push_back((nrow, ncol, time + 1));
                }
            }
        }
        unreachable!()
    }
}

fn part1(input: &Valley) -> i32 {
    input.trek(input.entrance, input.exit, 0)
}

fn part2(input: &Valley) -> i32 {
    let there = input.trek(input.entrance, input.exit, 0);
    let back = input.trek(input.exit, input.entrance, there);
    input.trek(input.entrance, input.exit, back)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;

    #[test]
    fn trivial() {
        let input = Valley::from(
            r#"#.#####
        #.....#
        #>....#
        #.....#
        #...v.#
        #.....#
        #####.#"#,
        );
        println!("{:#?}", input);
        panic!("Bah");
    }

    #[test]
    fn testcase1() {
        let input = Valley::from(TEST_INPUT);
        assert_eq!(part1(&input), 18);
    }

    #[test]
    fn testcase2() {
        let input = Valley::from(TEST_INPUT);
        assert_eq!(part2(&input), 54);
    }
}

pub fn main() -> Result<()> {
    let input = read_input(24)?;
    let input = Valley::from(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
