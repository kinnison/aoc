use aoc2021::*;

struct FishState {
    ages: VecDeque<usize>,
}

impl FishState {
    fn from_ages(fish: &[usize]) -> Self {
        let mut ages = VecDeque::new();
        ages.resize(9, 0);
        for age in fish.iter().copied() {
            ages[age] += 1;
        }
        Self { ages }
    }

    fn tick_day(&mut self) {
        // Each "day" we acquire how many fish are spawning new fish today
        let created = self.ages.pop_front().unwrap();
        // then we advance the day, by adding the spawned fish count to the back of the vecdeque
        self.ages.push_back(created);
        // Then we return the fish who spawned new fish to age 6
        self.ages[6] += created;
    }

    fn count_fish(&self) -> usize {
        self.ages.iter().sum()
    }

    fn fish_count_after(mut self, days: usize) -> usize {
        for _ in 0..days {
            self.tick_day();
        }
        self.count_fish()
    }
}

fn part1(input: &[usize]) -> usize {
    FishState::from_ages(input).fish_count_after(80)
}

fn part2(input: &[usize]) -> usize {
    FishState::from_ages(input).fish_count_after(256)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"3,4,3,1,2"#;

    #[test]
    fn testcase1() {
        let input: Vec<usize> = input_by_split_pat(TEST_INPUT, ",").unwrap();
        assert_eq!(part1(&input), 5934);
    }

    #[test]
    fn testcase2() {
        let input: Vec<usize> = input_by_split_pat(TEST_INPUT, ",").unwrap();
        assert_eq!(part2(&input), 26984457539);
    }
}

fn main() -> Result<()> {
    let input = read_input(6)?;
    let input = input_by_split_pat(input, ",")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
