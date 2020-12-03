use aoc2015::*;

/* Bad idea 1, takes too long to run:

struct Groupings {
    pos: Vec<usize>,
}

impl Groupings {
    fn new(num: usize) -> Groupings {
        let mut pos = Vec::new();
        pos.resize(num, 0);
        Groupings { pos }
    }

    fn at_end(&self) -> bool {
        self.pos.iter().all(|&v| v == 2)
    }
}

impl Iterator for Groupings {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_end() {
            return None;
        }
        let mut ret = self.pos.clone();
        ret.reverse();
        for idx in 0..self.pos.len() {
            if self.pos[idx] == 2 {
                self.pos[idx] = 0;
            } else {
                self.pos[idx] = self.pos[idx] + 1;
                break;
            }
        }
        Some(ret)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Arrangement {
    balanced: bool,
    count: usize,
    qe: u128,
}

impl Arrangement {
    fn new(gifts: &Vec<usize>, groups: &Vec<usize>) -> Arrangement {
        let mut tots = [0, 0, 0];
        let mut qe: u128 = 1;
        let mut count = 0;
        for idx in 0..gifts.len() {
            tots[groups[idx]] += gifts[idx];
        }
        let balanced = tots[0] == tots[1] && tots[1] == tots[2];
        if balanced {
            for idx in 0..gifts.len() {
                if groups[idx] == 0 {
                    count += 1;
                    let g128 = gifts[idx] as u128;
                    qe = qe * g128;
                }
            }
        }
        Arrangement {
            balanced,
            count,
            qe,
        }
    }
}

fn part1(input: &Vec<usize>) -> u128 {
    let mut fewest = std::usize::MAX;
    let mut bestqe = std::u128::MAX;
    for group in Groupings::new(input.len()) {
        let arr = Arrangement::new(input, &group);
        if arr.count < fewest {
            fewest = arr.count;
            bestqe = arr.qe;
        } else if arr.count == fewest {
            if arr.qe < bestqe {
                bestqe = arr.qe
            }
        }
    }
    bestqe
}

*/

/* Sadly permutations are still painful

fn part1(input: &Vec<usize>) -> u128 {
    let gift_tot: usize = input.iter().sum();
    let group_goal = gift_tot / 3;
    let arrs = 3_usize.pow(input.len() as u32);
    println!("Theoretically there are {} arrangements", arrs);
    println!("Gifts total {} making group goal {}", gift_tot, group_goal);
    let mut fewest = std::usize::MAX;
    let mut bestqe = std::u128::MAX;
    let mut input_copy = input.clone();
    let mut considered: usize = 0;
    heap_recursive(&mut input_copy, |perm| {
        considered += 1;
        if (considered % (arrs / 100)) == 0 {
            println!("Considering {}", considered);
        }
        let mut foottotal = 0;
        let mut footcount = 0;
        let mut footqe: u128 = 1;
        'foot: for idx in 0..perm.len() {
            foottotal += perm[idx];
            footcount += 1;
            footqe = footqe * (perm[idx] as u128);
            if foottotal > group_goal {
                return ();
            }
            if foottotal == group_goal {
                break 'foot;
            }
        }
        if footcount > fewest || (footcount == fewest && footqe > bestqe) {
            return ();
        }
        // If we get here, then the footwell is a possible winner, so let's try
        // and balance the rest of the sleigh
        let mut nextblock = 0;
        'block: for idx in footcount..perm.len() {
            nextblock += perm[idx];
            if nextblock > group_goal {
                return ();
            }
            if nextblock == group_goal {
                break 'block;
            }
        }
        // If we reach here, we got to the goal for foot and block, so the sleigh
        // is balanced...
        if footcount < fewest {
            fewest = footcount;
            bestqe = footqe;
            println!("New best bet: {} gifts with QE {}", fewest, bestqe);
        } else if footqe < bestqe {
            bestqe = footqe;
            println!("New best bet with QE {}", bestqe);
        }
    });
    bestqe
}

*/

fn part1(input: &[usize]) -> u128 {
    // First up, find the minimum length grouping which adds to our goal
    let gift_sum: usize = input.iter().sum();
    let goal_weight: usize = gift_sum / 3;
    println!("Gifts total {} so goal is {}", gift_sum, goal_weight);
    for group_len in 1..=input.len() {
        println!("Considering footgroups of size {}", group_len);
        let mut bestqe = std::u128::MAX;
        for combo in input.iter().combinations(group_len) {
            let combo_sum: usize = combo.iter().copied().sum();
            if combo_sum == goal_weight {
                let qe: u128 = combo.into_iter().map(|&v| v as u128).product();
                if qe < bestqe {
                    bestqe = qe;
                }
            }
        }
        if bestqe != std::u128::MAX {
            return bestqe;
        }
    }
    unreachable!()
}

fn part2(input: &[usize]) -> u128 {
    // First up, find the minimum length grouping which adds to our goal
    let gift_sum: usize = input.iter().sum();
    let goal_weight: usize = gift_sum / 4;
    println!("Gifts total {} so goal is {}", gift_sum, goal_weight);
    for group_len in 1..=input.len() {
        println!("Considering footgroups of size {}", group_len);
        let mut bestqe = std::u128::MAX;
        for combo in input.iter().combinations(group_len) {
            let combo_sum: usize = combo.iter().copied().sum();
            if combo_sum == goal_weight {
                let qe: u128 = combo.into_iter().map(|&v| v as u128).product();
                if qe < bestqe {
                    bestqe = qe;
                }
            }
        }
        if bestqe != std::u128::MAX {
            return bestqe;
        }
    }
    unreachable!()
}

fn main() -> Result<()> {
    let input: Vec<usize> = read_input(24)?
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
