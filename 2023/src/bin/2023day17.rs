use aoc2023::*;

pub fn main() -> Result<()> {
    let input = read_input(17)?;
    let input = RoadMap::from_str(&input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(input: &RoadMap) -> u64 {
    pathfinding::prelude::astar(
        &(0, 0, Facing::North),
        |state| input.successors(state, 0, 3),
        |state| input.heuristic(state),
        |state| input.success(state),
    )
    .unwrap()
    .1
}

fn part2(input: &RoadMap) -> u64 {
    pathfinding::prelude::astar(
        &(0, 0, Facing::North),
        |state| input.successors(state, 3, 7),
        |state| input.heuristic(state),
        |state| input.success(state),
    )
    .unwrap()
    .1
}

#[derive(Debug, Clone)]
struct RoadMap {
    cost: Vec<Vec<u64>>,
}

impl FromStr for RoadMap {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let cost = s
            .trim()
            .lines()
            .map(|s| s.trim().bytes().map(|b| (b - b'0') as u64).collect_vec())
            .collect_vec();
        Ok(Self { cost })
    }
}

impl RoadMap {
    fn cost_to_enter(&self, row: i32, col: i32) -> u64 {
        if row < 0 || col < 0 {
            u64::MAX
        } else {
            let urow = row as usize;
            let ucol = col as usize;
            if urow >= self.cost.len() || ucol >= self.cost[0].len() {
                u64::MAX
            } else {
                self.cost[urow][ucol]
            }
        }
    }

    fn successors(
        &self,
        state: &(i32, i32, Facing),
        skip_dist: i32,
        try_dist: i32,
    ) -> Vec<((i32, i32, Facing), u64)> {
        let mut ret = Vec::new();
        if *state == (0, 0, Facing::North) {
            self._successors(&(0, 0, Facing::East), skip_dist, try_dist, &mut ret);
            self._successors(&(0, 0, Facing::South), skip_dist, try_dist, &mut ret);
        } else {
            self._successors(state, skip_dist, try_dist, &mut ret);
        }
        //eprintln!("At {state:?} successors are {ret:?}");
        ret
    }

    fn _successors(
        &self,
        state: &(i32, i32, Facing),
        skip_dist: i32,
        try_dist: i32,
        ret: &mut Vec<((i32, i32, Facing), u64)>,
    ) {
        let srow = state.0;
        let scol = state.1;
        let sdir = state.2;
        // We are at srow, scol, facing sdir.  We want to take the possible number of fwds we can,
        // and then store each of those with a turn left / right facing into ret
        let (rofs, cofs) = sdir.row_col_offset();
        let mut cost: u64 = 0;
        for step in 1..=skip_dist {
            let row = srow + (rofs * step);
            let col = scol + (cofs * step);
            cost = cost.saturating_add(self.cost_to_enter(row, col));
        }
        for step in 1..=try_dist {
            let row = srow + (rofs * (step + skip_dist));
            let col = scol + (cofs * (step + skip_dist));
            cost = cost.saturating_add(self.cost_to_enter(row, col));
            if cost < u64::MAX {
                // We can enter this cell, so let's do so
                ret.push(((row, col, sdir.turn_left_deg(90)), cost));
                ret.push(((row, col, sdir.turn_right_deg(90)), cost));
            }
        }
    }

    fn heuristic(&self, state: &(i32, i32, Facing)) -> u64 {
        // Our A* heuristic is simply the manhattan distance from here to there
        let drow = (self.cost.len() as i32) - 1;
        let dcol = (self.cost[0].len() as i32) - 1;
        (state.0.abs_diff(drow) + state.1.abs_diff(dcol)) as u64
    }

    fn success(&self, state: &(i32, i32, Facing)) -> bool {
        // Success is if we've reached the bottom right
        let urow = state.0 as usize;
        let ucol = state.1 as usize;
        urow == self.cost.len() - 1 && ucol == self.cost[0].len() - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533
    "#;

    #[test]
    fn testcase1() {
        let input = RoadMap::from_str(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 102);
    }

    #[test]
    fn testcase2() {
        let input = RoadMap::from_str(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 94);
    }

    static TEST_INPUT2: &str = r#"111111111111
    999999999991
    999999999991
    999999999991
    999999999991
    "#;

    #[test]
    fn testcase3() {
        let input = RoadMap::from_str(TEST_INPUT2).unwrap();
        assert_eq!(part2(&input), 71);
    }
}
