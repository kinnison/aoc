use aoc2021::*;

struct Cave {
    risks: HashMap<(i32, i32), i32>,
    width: i32,
    height: i32,
}

impl FromStr for Cave {
    type Err = GenError;

    fn from_str(input: &str) -> Result<Self> {
        let input = input.trim();
        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;
        let mut risks = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.trim().bytes().enumerate() {
                assert!((b'0'..=b'9').contains(&ch));
                let coord = (i32::try_from(x)?, i32::try_from(y)?);
                let risk = i32::from(ch - b'0');
                risks.insert(coord, risk);
            }
        }
        Ok(Self {
            risks,
            width,
            height,
        })
    }
}

impl Cave {
    // Essentially dijkstra's algorithm
    fn shortest_path(&self) -> i32 {
        let target = (self.width - 1, self.height - 1);
        let mut costs = vec![vec![i32::MAX; self.width as usize]; self.height as usize];
        // We use a binaryheap which is, for all intents and purposes an always-ordered vec
        // values are (-cost, x, y)
        // using negative cost because that way when we pop we're doing a min-heap not a max-heap
        // i.e. we're always getting the smallest cost (since -1 is bigger than -2 and we want the 1)
        // this is effectively a less-type-filled version of the dijkstra on
        // https://doc.rust-lang.org/std/collections/binary_heap/index.html
        // (which I found with searching for "rust std dijkstra" with google)
        // Dijkstra works on edge costs, the edge cost for any edge into (x,y) is the risk at (x,y)
        let mut walk_queue = std::collections::BinaryHeap::new();
        walk_queue.push((0, 0, 0));
        while let Some((cost, cur_x, cur_y)) = walk_queue.pop() {
            let cost = -cost;
            if (cur_x, cur_y) == target {
                return cost;
            }
            for (new_x, new_y) in [
                (cur_x - 1, cur_y),
                (cur_x + 1, cur_y),
                (cur_x, cur_y - 1),
                (cur_x, cur_y + 1),
            ] {
                // not all orthog coords will be real (i.e. map edges) so don't just index...
                if let Some(risk) = self.risks.get(&(new_x, new_y)).copied() {
                    let next_cost = cost + risk;
                    if next_cost < costs[new_x as usize][new_y as usize] {
                        walk_queue.push((-next_cost, new_x, new_y));
                        costs[new_x as usize][new_y as usize] = next_cost;
                    }
                }
            }
        }
        unreachable!()
    }

    fn expanded_map(&self) -> Self {
        let width = self.width * 5;
        let height = self.height * 5;
        let mut risks = HashMap::new();
        for ((x, y), r) in self.risks.iter() {
            let x = *x;
            let y = *y;
            let r = *r;
            for tile_x in 0..5 {
                for tile_y in 0..5 {
                    let x = x + (tile_x * self.width);
                    let y = y + (tile_y * self.height);
                    let mut r = r + tile_x + tile_y;
                    // wrap risk into 1..=9
                    if r > 9 {
                        r -= 9;
                    }
                    risks.insert((x, y), r);
                }
            }
        }
        Self {
            risks,
            width,
            height,
        }
    }
}
fn part1(input: &Cave) -> i32 {
    input.shortest_path()
}

fn part2(input: &Cave) -> i32 {
    input.expanded_map().shortest_path()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;

    #[test]
    fn testcase1() {
        let input = Cave::from_str(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 40);
    }

    #[test]
    fn testcase2() {
        let input = Cave::from_str(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 315);
    }
}

fn main() -> Result<()> {
    let input = read_input(15)?;
    let input = Cave::from_str(&input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
