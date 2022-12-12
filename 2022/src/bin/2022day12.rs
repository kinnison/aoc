use aoc2022::*;

#[derive(Debug, Clone)]
struct Input {
    width: usize,
    height: usize,
    start: (i32, i32),
    end: (i32, i32),
    map: HashMap<(i32, i32), u8>,
}

impl<T> From<T> for Input
where
    T: AsRef<str>,
{
    fn from(input: T) -> Self {
        let input = input.as_ref().trim();
        let width = input.lines().next().unwrap().trim().len();
        let height = input.lines().count();
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut map = HashMap::new();
        for (rown, row) in input.lines().enumerate() {
            let rown = rown as i32;
            for (coln, cell) in row.trim().bytes().enumerate() {
                let coln = coln as i32;
                match cell {
                    b'S' => {
                        map.insert((rown, coln), b'a');
                        start = (rown, coln);
                    }
                    b'E' => {
                        map.insert((rown, coln), b'z');
                        end = (rown, coln);
                    }
                    b'a'..=b'z' => {
                        map.insert((rown, coln), cell);
                    }
                    _ => {
                        panic!("Bad input at {},{}", rown, coln);
                    }
                }
            }
        }
        Self {
            width,
            height,
            start,
            end,
            map,
        }
    }
}

fn route_from(input: &Input, start: (i32, i32)) -> usize {
    let route = pathfinding::directed::dijkstra::dijkstra(
        &start,
        |&(row, col)| {
            let cur = *input.map.get(&(row, col)).unwrap();
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .map(move |(rowo, colo)| (row + rowo, col + colo))
                .filter(|pos| input.map.contains_key(pos))
                .filter(move |pos| {
                    let h = *input.map.get(pos).unwrap();
                    h <= (cur + 1)
                })
                .map(|pos| (pos, 1))
        },
        |&pos| pos == input.end,
    );
    if let Some(route) = route {
        route.0.len() - 1
    } else {
        usize::MAX
    }
}

fn part1(input: &Input) -> usize {
    route_from(input, input.start)
}

fn part2(input: &Input) -> usize {
    input
        .map
        .iter()
        .filter(|&(_, v)| *v == b'a')
        .map(|(k, _)| *k)
        .map(|start| route_from(input, start))
        .sorted()
        .next()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

    #[test]
    fn testcase1() {
        let input = Input::from(TEST_INPUT);
        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn testcase2() {
        let input = Input::from(TEST_INPUT);
        assert_eq!(part2(&input), 29);
    }
}

pub fn main() -> Result<()> {
    let input = read_input(12)?;
    let input = Input::from(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
