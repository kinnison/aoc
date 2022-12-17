use aoc2022::*;

static ROCKS: &[(char, &[(i64, i64)])] = &[
    /* - */
    ('-', &[(2, 0), (3, 0), (4, 0), (5, 0)]),
    /* + */
    ('+', &[(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)]),
    /* ⅃ */
    ('⅃', &[(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)]),
    /* l */
    ('|', &[(2, 0), (2, 1), (2, 2), (2, 3)]),
    /* o */
    ('o', &[(2, 0), (3, 0), (2, 1), (3, 1)]),
];

#[derive(Default)]
struct Shaft {
    contents: HashSet<(i64, i64)>,
    jet: usize,
    highest: i64,
    profile: [i64; 7],
}

impl Shaft {
    fn add_floor(&mut self) {
        // Since we want rocks to land at y=1, we set the floor at y=0
        self.contents.extend((0..7).map(|n| (n, 0)));
        // Profile is already all zeroes which is correct.
    }
    fn drop_rock(&mut self, jets: &str, rock: &[(i64, i64)]) {
        let mut rockset: HashSet<_> = rock
            .iter()
            .copied()
            .map(|(x, y)| (x, y + 4 + self.highest))
            .collect();
        loop {
            self.try_squirt(jets, &mut rockset);
            if self.try_drop(&mut rockset) {
                break;
            }
        }
        self.highest = max(self.highest, rockset.iter().map(|(_, y)| *y).max().unwrap());
        self.contents.extend(rockset.into_iter());
        for x in 0..7 {
            for y in (0..=self.highest).rev() {
                if self.contents.contains(&(x, y)) {
                    self.profile[x as usize] = self.highest - y;
                    break;
                }
            }
        }
    }

    fn try_squirt(&mut self, jets: &str, rock: &mut HashSet<(i64, i64)>) {
        let ofs = match jets.as_bytes()[self.jet] {
            b'<' => -1,
            b'>' => 1,
            x => panic!("Don't know what to do with jet character '{}'", x as char),
        };
        self.jet = (self.jet + 1) % jets.as_bytes().len();
        if !rock.iter().any(|&(x, y)| {
            ((x + ofs) < 0 || (x + ofs) == 7) || self.contents.contains(&(x + ofs, y))
        }) {
            // OK, we can do the move
            let mut newset = rock.drain().map(|(x, y)| (x + ofs, y)).collect();
            std::mem::swap(rock, &mut newset);
        }
    }

    fn try_drop(&mut self, rock: &mut HashSet<(i64, i64)>) -> bool {
        if rock
            .iter()
            .any(|&(x, y)| self.contents.contains(&(x, y - 1)))
        {
            // Stopped
            true
        } else {
            // Didn't stop, so replace
            let mut newset = rock.drain().map(|(x, y)| (x, y - 1)).collect();
            std::mem::swap(rock, &mut newset);
            false
        }
    }

    fn drop_rocks(&mut self, jets: &str, n: u64) -> i64 {
        // Map (profile,rockindex,jetindex) to (rockcount, height)
        let mut cache: HashMap<([i64; 7], usize, usize), (u64, i64)> = HashMap::new();
        let mut rockidx = 0;
        let mut dropped = 0;
        let mut heightgain = 0;
        //println!("Dropping {} rocks", n);
        while dropped < n {
            let rock = ROCKS[rockidx].1;
            self.drop_rock(jets, rock);
            match cache.entry((self.profile, rockidx, self.jet)) {
                Entry::Occupied(o) => {
                    //println!("Found a repeat of the current sequence");
                    let (count, height) = o.get();
                    //println!(
                    //    "We just dropped rock {} and we saw this before at rock {}",
                    //    dropped, count
                    //);
                    //println!(
                    //    "The cave is currently {} high, and we saw this before when it was {} high",
                    //    self.highest, height
                    //);
                    // We can gain height by repeating ourselves...
                    let gain = (self.highest - height) as u64;
                    let drops = dropped - count;
                    let skips = (n - dropped) / drops;
                    //println!(
                    //    "This means we can skip {skips} drops of {drops} and gain {gain} height each time"
                    //);
                    //println!("Skipping {} drops for {} gain", drops * skips, gain * skips);
                    heightgain += skips * gain;
                    dropped += skips * drops;
                    //println!("This leaves {} rocks to drop", n - dropped);
                }
                Entry::Vacant(v) => {
                    v.insert((dropped, self.highest));
                }
            }
            rockidx = (rockidx + 1) % ROCKS.len();
            dropped += 1;
        }

        self.highest + (heightgain as i64)
    }
}

fn part1(input: &str) -> i64 {
    let mut shaft = Shaft::default();
    shaft.add_floor();
    for (_, rock) in ROCKS.iter().cycle().take(2022) {
        shaft.drop_rock(input, rock);
    }
    shaft.contents.iter().map(|(_, y)| *y).max().unwrap()
}

fn part2(input: &str) -> i64 {
    // we need to detect a cycle
    // realistically the cycle key is (rock_index, jet_index, top_profile)
    let mut shaft = Shaft::default();
    shaft.add_floor();
    shaft.drop_rocks(input, 1000000000000)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

    #[test]
    fn testcase1() {
        assert_eq!(part1(TEST_INPUT), 3068);
    }

    #[test]
    fn testcase2() {
        assert_eq!(part2(TEST_INPUT), 1514285714288);
    }
}

pub fn main() -> Result<()> {
    let input = read_input(17)?;
    let input = input.trim();
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
    Ok(())
}
