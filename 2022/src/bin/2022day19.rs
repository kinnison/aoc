use aoc2022::*;

#[derive(ParseByRegex, Copy, Clone, Debug)]
#[regex = r"Blueprint (?P<bpnum>\d+): Each ore robot costs (?P<orebotore>\d+) ore. Each clay robot costs (?P<claybotore>\d+) ore. Each obsidian robot costs (?P<obsbotore>\d+) ore and (?P<obsbotclay>\d+) clay. Each geode robot costs (?P<geobotore>\d+) ore and (?P<geobotobs>\d+) obsidian."]
struct Blueprint {
    bpnum: usize,
    orebotore: usize,
    claybotore: usize,
    obsbotore: usize,
    obsbotclay: usize,
    geobotore: usize,
    geobotobs: usize,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
}

impl Resources {
    fn begin(&mut self, action: Action, bp: &Blueprint) {
        match action {
            Action::Wait => (),
            Action::BuildOreBot => {
                self.ore -= bp.orebotore;
            }
            Action::BuildClayBot => {
                self.ore -= bp.claybotore;
            }
            Action::BuildObsBot => {
                self.ore -= bp.obsbotore;
                self.clay -= bp.obsbotclay;
            }
            Action::BuildGeoBot => {
                self.ore -= bp.geobotore;
                self.obsidian -= bp.geobotobs;
            }
        }
    }
    fn rollback(&mut self, action: Action, bp: &Blueprint) {
        match action {
            Action::Wait => (),
            Action::BuildOreBot => {
                self.ore += bp.orebotore;
            }
            Action::BuildClayBot => {
                self.ore += bp.claybotore;
            }
            Action::BuildObsBot => {
                self.ore += bp.obsbotore;
                self.clay += bp.obsbotclay;
            }
            Action::BuildGeoBot => {
                self.ore += bp.geobotore;
                self.obsidian += bp.geobotobs;
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct BotArmy {
    orebots: usize,
    claybots: usize,
    obsbots: usize,
    geobots: usize,
}

impl Default for BotArmy {
    fn default() -> Self {
        Self {
            orebots: 1,
            claybots: 0,
            obsbots: 0,
            geobots: 0,
        }
    }
}

impl BotArmy {
    fn gather(&self, resources: &mut Resources) {
        resources.ore += self.orebots;
        resources.clay += self.claybots;
        resources.obsidian += self.obsbots;
        resources.geodes += self.geobots;
    }
    fn ungather(&self, resources: &mut Resources) {
        resources.ore -= self.orebots;
        resources.clay -= self.claybots;
        resources.obsidian -= self.obsbots;
        resources.geodes -= self.geobots;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Action {
    Wait,
    BuildOreBot,
    BuildClayBot,
    BuildObsBot,
    BuildGeoBot,
}

impl Action {
    fn possibilities(bp: &Blueprint, resources: &Resources) -> Vec<Self> {
        let mut ret = vec![Self::Wait];
        if resources.ore >= bp.orebotore {
            ret.push(Self::BuildOreBot)
        }
        if resources.ore >= bp.claybotore {
            ret.push(Self::BuildClayBot)
        }
        if resources.ore >= bp.obsbotore && resources.clay >= bp.obsbotclay {
            ret.push(Self::BuildObsBot)
        }
        if resources.ore >= bp.geobotore && resources.obsidian >= bp.geobotobs {
            ret.push(Self::BuildGeoBot)
        }
        ret.reverse();
        ret
    }

    fn apply(self, bots: &mut BotArmy) {
        match self {
            Action::Wait => (),
            Action::BuildOreBot => {
                bots.orebots += 1;
            }
            Action::BuildClayBot => {
                bots.claybots += 1;
            }
            Action::BuildObsBot => {
                bots.obsbots += 1;
            }
            Action::BuildGeoBot => {
                bots.geobots += 1;
            }
        }
    }
    fn unapply(self, bots: &mut BotArmy) {
        match self {
            Action::Wait => (),
            Action::BuildOreBot => {
                bots.orebots -= 1;
            }
            Action::BuildClayBot => {
                bots.claybots -= 1;
            }
            Action::BuildObsBot => {
                bots.obsbots -= 1;
            }
            Action::BuildGeoBot => {
                bots.geobots -= 1;
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct ActionState {
    minute: usize,
    bots: BotArmy,
    resources: Resources,
}

impl BotArmy {
    fn eval_blueprint(bp: &Blueprint, minutes: usize) -> usize {
        // Evaluate a blueprint and return the number of geodes recovered at best inside minutes
        let mut queue = VecDeque::new();
        queue.push_back(ActionState {
            minute: 0,
            bots: BotArmy::default(),
            resources: Resources::default(),
        });

        let mut state_seen = HashSet::new();
        let mut best_geodes = 0;

        let most_ore = bp
            .orebotore
            .max(bp.claybotore)
            .max(bp.obsbotore)
            .max(bp.geobotore);

        while let Some(state) = queue.pop_front() {
            best_geodes = best_geodes.max(state.resources.geodes);
            // For part 1 it was enough to just say if we'd seen the state before
            if state_seen.contains(&state) ||
            // But part 2 took far too long, so add to the heuristic that there's no point
            // continuing if the state we're looking at is so far behind that it can't catch up
            (state.resources.geodes + 1) < best_geodes
            {
                // Either there's no hope we can find enough geodes, or we've seen this state before
                // so stop anyway
                continue;
            }

            state_seen.insert(state);
            if state.minute == minutes {
                continue;
            }

            // We try each action, geo, obs, clay, ore, wait (in that order)
            // skipping any we cannot do this turn
            for action in Action::possibilities(bp, &state.resources) {
                if action == Action::BuildOreBot && state.bots.orebots == most_ore {
                    // No point building more ore bots at this time
                    continue;
                }
                let mut newstate = state;
                // We're going to perform action
                newstate.resources.begin(action, bp);
                newstate.bots.gather(&mut newstate.resources);
                action.apply(&mut newstate.bots);
                newstate.minute += 1;
                queue.push_back(newstate);
            }
        }

        println!("Blueprint {} produces {}", bp.bpnum, best_geodes);

        best_geodes
    }
}

fn part1(input: &[Blueprint]) -> usize {
    input
        .iter()
        .map(|bp| bp.bpnum * BotArmy::eval_blueprint(bp, 24))
        .sum()
}

fn part2(input: &[Blueprint]) -> usize {
    input
        .iter()
        .take(3)
        .map(|bp| BotArmy::eval_blueprint(bp, 32))
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
"#;

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(dbg!(&input)), 33);
    }

    #[test]
    fn testcase2() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), (46 * 62));
    }
}

pub fn main() -> Result<()> {
    let input: Vec<Blueprint> = read_input_as_vec(19)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
