use aoc2022::*;

#[derive(Debug, Clone)]
struct InputPair {
    left: InputValue,
    right: InputValue,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
enum InputValue {
    Num(usize),
    List(Vec<InputValue>),
}

impl FromStr for InputPair {
    type Err = Infallible;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let (left, right) = s.trim().split_once('\n').unwrap();
        let left = serde_json::from_str(left.trim()).unwrap();
        let right = serde_json::from_str(right.trim()).unwrap();
        Ok(Self { left, right })
    }
}

impl PartialOrd for InputValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (InputValue::Num(left), InputValue::Num(right)) => left.partial_cmp(right),
            (InputValue::Num(left), right @ InputValue::List(_)) => {
                InputValue::List(vec![InputValue::Num(*left)]).partial_cmp(right)
            }
            (left @ InputValue::List(_), InputValue::Num(right)) => {
                left.partial_cmp(&InputValue::List(vec![InputValue::Num(*right)]))
            }
            (InputValue::List(left), InputValue::List(right)) => {
                let mut left = left.iter();
                let mut right = right.iter();
                loop {
                    match (left.next(), right.next()) {
                        (None, None) => break Some(Ordering::Equal),
                        (None, Some(_)) => break Some(Ordering::Less),
                        (Some(_), None) => break Some(Ordering::Greater),
                        (Some(left), Some(right)) => match left.partial_cmp(right) {
                            Some(Ordering::Equal) => continue,
                            v @ Some(_) => break v,
                            None => unreachable!(),
                        },
                    }
                }
            }
        }
    }
}

impl Ord for InputValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part1(input: &[InputPair]) -> usize {
    let mut total = 0;
    for (i, pair) in input.iter().enumerate() {
        if pair.left < pair.right {
            total += i + 1;
        }
    }
    total
}

fn part2(input: &[InputPair]) -> usize {
    let decoder = InputPair::from_str("[[2]]\n[[6]]").unwrap();
    let mut packets = vec![decoder.left.clone(), decoder.right.clone()];
    for pair in input {
        packets.push(pair.left.clone());
        packets.push(pair.right.clone());
    }
    packets.sort_unstable();

    let idx1 = packets.binary_search(&decoder.left).unwrap();
    let idx2 = packets.binary_search(&decoder.right).unwrap();

    (idx1 + 1) * (idx2 + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;

    #[test]
    fn testcase1() {
        let input: Vec<InputPair> = input_as_chunks(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn testcase2() {
        let input: Vec<InputPair> = input_as_chunks(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 140);
    }
}

pub fn main() -> Result<()> {
    let input: Vec<InputPair> = read_input_as_chunks(13)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
