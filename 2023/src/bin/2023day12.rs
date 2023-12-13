use aoc2023::*;

pub fn main() -> Result<()> {
    let input: Vec<SpringMap> = read_input_as_vec(12)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Debug)]
struct SpringMap {
    springs: Vec<Spring>,
    sets: Vec<usize>,
}

impl FromStr for SpringMap {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (springs, sets) = s.trim().split_once(' ').unwrap();
        let springs = springs.chars().map(Spring::from_char).collect_vec();
        let sets = sets
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec();
        Ok(Self { springs, sets })
    }
}

#[derive(Debug, Clone, Copy)]
enum Spring {
    Working,
    Broken,
    Unknown,
}

impl Spring {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Working,
            '#' => Self::Broken,
            _ => Self::Unknown,
        }
    }

    fn maybe_broken(self) -> bool {
        !matches!(self, Self::Working)
    }

    fn maybe_working(self) -> bool {
        !matches!(self, Self::Broken)
    }

    fn is_working(self) -> bool {
        matches!(self, Self::Working)
    }

    fn is_broken(self) -> bool {
        matches!(self, Self::Broken)
    }
}

impl SpringMap {
    fn _ways(springs: &[Spring], sets: &[usize]) -> usize {
        //println!("_ways({springs:?}, {sets:?}");
        // We're going to go set by set, to satisfy things
        // If we run out of sets, there had best be no broken springs left
        if sets.is_empty() {
            if springs.iter().copied().any(Spring::is_broken) {
                //println!("No more sets, but there are broken springs left!");
                return 0; // No way to satisfy more broken springs
            } else {
                //println!("No more sets, no more broken springs, hurrah");
                return 1; // We can satisfy what's left by it being all working
            }
        }
        // If we run out of springs, there'd best be no sets left
        if springs.is_empty() {
            if sets.is_empty() {
                //println!("No more springs, no more sets, hurrah");
                return 1; // One way to do nothing
            } else {
                //println!("No more springs, but more sets, booo!");
                return 0; // No way to satisfy more sets with no springs
            }
        }

        // Okay, so we have at least one spring, and at least one set left to satisfy it.
        let mut count = 0;
        // If the spring could be working, then we skip it and count what's left
        if springs[0].maybe_working() {
            //println!("First spring may be working, recursing...");
            count += Self::_ways(&springs[1..], sets);
            //println!("Back to {springs:?}, {sets:?}");
        }
        // If the spring could be broken, then we need to try and complete the current set
        if springs[0].maybe_broken() {
            //println!("First spring is broken, do we have enough springs left?");
            // Step one, are there enough springs left at all to satisfy this set?
            if springs.len() >= sets[0] {
                // Next question, are there any working springs in that slice?
                //println!("Enough springs left, are there enough maybe_broken?");
                if !springs
                    .iter()
                    .take(sets[0])
                    .copied()
                    .any(Spring::is_working)
                {
                    // Okay, so the next sets[0] springs are or could be broken,
                    // so let's assume they are.  But, is the *subsequent* spring (if there is one)
                    // broken?  If it is, the set is too long
                    //println!("Enough maybe broken, but is the subsequent spring (if any) working?");
                    if !matches!(
                        springs.iter().skip(sets[0]).copied().next(),
                        Some(Spring::Broken)
                    ) {
                        //println!("Looks good, recursing...");
                        // All good, we have sets[0] maybe_broken springs, and we are not
                        // running into another spring, so let's call that good
                        let slicefrom = if sets[0] < springs.len() {
                            sets[0] + 1
                        } else {
                            sets[0]
                        };
                        count += Self::_ways(&springs[slicefrom..], &sets[1..]);
                    }
                }
            }
        }
        //println!("Done here, found {count} ways so far");
        count
    }

    fn ways(&self) -> usize {
        dbg!(Self::_ways(&self.springs, &self.sets))
    }

    fn unfold(&self) -> Self {
        let sets = self.sets.repeat(5);
        let mut springs = Vec::new();
        for _ in 0..5 {
            springs.extend_from_slice(&self.springs);
            springs.push(Spring::Unknown);
        }
        springs.pop();
        Self { springs, sets }
    }
}

fn part1(input: &[SpringMap]) -> usize {
    input.iter().map(SpringMap::ways).sum()
}

fn part2(input: &[SpringMap]) -> usize {
    input.iter().map(|l| l.unfold().ways()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn testcase0() {
        let input: Vec<SpringMap> = input_as_vec("?###???????? 3,2,1").unwrap();
        assert_eq!(input[0].ways(), 10);
    }

    #[test]
    fn testcase1() {
        let input: Vec<SpringMap> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn testcase2() {
        let input: Vec<SpringMap> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 525152);
    }
}
