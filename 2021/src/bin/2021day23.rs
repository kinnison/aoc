use aoc2021::*;

use pathfinding::directed::dijkstra;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct AmbiCave {
    hallway: [u8; 7],
    caves: [[u8; 4]; 4],
}

impl FromStr for AmbiCave {
    type Err = String;
    fn from_str(input: &str) -> StdResult<Self, Self::Err> {
        let input = input
            .chars()
            .filter(|c| ('A'..='D').contains(c))
            .map(|c| (c as u8) - b'A' + 1)
            .collect_vec();
        if input.len() != 8 {
            Err("Wrong size cave".into())
        } else {
            Ok(AmbiCave {
                hallway: [0; 7],
                caves: [
                    [1, 1, input[4], input[0]],
                    [2, 2, input[5], input[1]],
                    [3, 3, input[6], input[2]],
                    [4, 4, input[7], input[3]],
                ],
            })
        }
    }
}

const HPOS: [usize; 7] = [1, 2, 4, 6, 8, 10, 11];
const HIDX: [usize; 12] = [0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 6];
const CPOS: [usize; 4] = [3, 5, 7, 9];

impl AmbiCave {
    fn ambi_cost(ambi: u8) -> usize {
        match ambi {
            0 => 0,
            1 => 1,
            2 => 10,
            3 => 100,
            4 => 1000,
            _ => unreachable!(),
        }
    }

    fn hallway_cave_dist(cave: usize, hidx: usize) -> usize {
        // Hallway costs are related to the cave positions
        // We measure hallway <-> front of cave, caller can add something if back-of-cave needed
        // as such, we return 1 to move from cave mouth to hallway, then the distance to the chosen hallway position
        let cpos = CPOS[cave];
        let hpos = HPOS[hidx];
        max(cpos, hpos) - min(cpos, hpos) + 1
    }

    fn cave_to_hallway_clear(&self, cave: usize, hidx: usize) -> bool {
        // The hallway is clear if there's nothing between the cave and the hallway target
        let cpos = CPOS[cave];
        let hpos = HPOS[hidx];
        let (hmin, hmax) = if cpos < hpos {
            (HIDX[cpos + 1], hidx)
        } else {
            (hidx, HIDX[cpos - 1])
        };
        (hmin..=hmax).all(|hpos| self.hallway[hpos] == 0)
    }

    fn hallway_to_cave_clear(&self, cave: usize, hidx: usize) -> bool {
        // The hallway is clear if there's nothing between the cave and the hallway target
        let cpos = CPOS[cave];
        let hpos = HPOS[hidx];
        let (hmin, hmax) = if cpos < hpos {
            (HIDX[cpos + 1], hidx - 1)
        } else {
            (hidx + 1, HIDX[cpos - 1])
        };
        (hmin..=hmax).all(|hpos| self.hallway[hpos] == 0)
    }

    fn state_ok(&self) -> bool {
        let mut counts = [0usize; 5];
        self.hallway
            .iter()
            .map(|c| *c as usize)
            .for_each(|c| counts[c] += 1);
        (0..4)
            .cartesian_product(0..4)
            .map(|(cave, pos)| self.caves[cave][pos] as usize)
            .for_each(|c| counts[c] += 1);
        counts == [7, 4, 4, 4, 4]
    }

    fn neighbours(&self) -> Vec<(Self, usize)> {
        let mut ret = Vec::new();

        // All possible neighbours of this node in the search graph
        // will either move an ambipod out of a cave, or into its target cave
        // It can only move if the hallway is clear

        for cave in 0..4 {
            for pos in 0..4 {
                let ambipod = self.caves[cave][pos];

                // No ambipod
                if ambipod == 0 {
                    continue;
                }
                // is our path out of the cave clear?
                if pos < 3 && self.caves[cave].iter().skip(pos + 1).any(|v| *v != 0) {
                    continue;
                }
                // Our path is clear out of the cave, but do we want to move?
                if pos > 0
                    && self.caves[cave]
                        .iter()
                        .take(pos + 1)
                        .all(|v| *v == (cave + 1) as u8)
                {
                    continue;
                }
                // Otherwise try and fit it into each hallway slot
                for hidx in 0..7 {
                    if self.cave_to_hallway_clear(cave, hidx) {
                        let mut cost = Self::hallway_cave_dist(cave, hidx);
                        cost += 3 - pos;
                        cost *= Self::ambi_cost(ambipod);
                        let mut newstate = *self;
                        newstate.caves[cave][pos] = 0;
                        newstate.hallway[hidx] = ambipod;
                        debug_assert!(newstate.state_ok());
                        ret.push((newstate, cost));
                    }
                }
            }
        }

        // Having moved out any ambipod which can move out, let's consider moving in any ambipod
        // in the hallway which has a clear line to its end position
        for hidx in 0..7 {
            let ambipod = self.hallway[hidx];
            if ambipod == 0 {
                continue;
            }
            // is the target cave "available"? first is the path clear
            let cave = (ambipod as usize) - 1;
            if !self.hallway_to_cave_clear(cave, hidx) {
                continue;
            }
            let base_cost = Self::hallway_cave_dist(cave, hidx);
            // the base cost is to the mouth of the cave.
            // we can only move into the cave if the cave contains nothing
            // which isn't us.
            if self.caves[cave].iter().any(|v| *v != 0 && *v != ambipod) {
                continue;
            }
            // OK, the cave has *only* us in it, so let's try and move in
            let pos = self.caves[cave]
                .iter()
                .copied()
                .enumerate()
                .find(|v| v.1 == 0)
                .expect("What?")
                .0;
            let mut newstate = *self;
            newstate.hallway[hidx] = 0;
            newstate.caves[cave][pos] = ambipod;
            let cost = (base_cost + 3 - pos) * Self::ambi_cost(ambipod);
            debug_assert!(newstate.state_ok());
            ret.push((newstate, cost));
        }

        ret
    }

    fn is_finished(&self) -> bool {
        let ret = self.hallway.iter().all(|v| *v == 0)
            && (0..4).all(|i| self.caves[i] == [(i as u8) + 1; 4]);
        ret
    }

    // Part 2 unfolds a bit of the paper
    fn unfold(&mut self) {
        for i in 0..4 {
            self.caves[i][0] = self.caves[i][2];
        }
        //#D#C#B#A#
        //#D#B#A#C#
        self.caves[0][1] = 4;
        self.caves[0][2] = 4;
        self.caves[1][1] = 2;
        self.caves[1][2] = 3;
        self.caves[2][1] = 1;
        self.caves[2][2] = 2;
        self.caves[3][1] = 3;
        self.caves[3][2] = 1;
        debug_assert!(self.state_ok());
    }
}

fn part1(input: &AmbiCave) -> usize {
    let (_path, cost) =
        dijkstra::dijkstra(input, AmbiCave::neighbours, AmbiCave::is_finished).unwrap();
    cost
}

fn part2(input: &AmbiCave) -> usize {
    let mut input = *input;
    input.unfold();
    let (_path, cost) =
        dijkstra::dijkstra(&input, AmbiCave::neighbours, AmbiCave::is_finished).unwrap();
    cost
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
"#;

    #[test]
    fn testcase1() {
        let input = AmbiCave::from_str(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 12521);
    }

    #[test]
    fn testcase2() {
        let input = AmbiCave::from_str(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 44169);
    }
}

fn main() -> Result<()> {
    let input = read_input(23)?;
    let input = AmbiCave::from_str(&input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
