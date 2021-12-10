use aoc2021::*;

#[derive(Debug)]
enum SCheck {
    OK,
    Incomplete(Vec<char>),
    BadClose(char),
}

impl SCheck {
    fn badscore(&self) -> u64 {
        match self {
            SCheck::OK => 0,
            SCheck::Incomplete(_) => 0,
            SCheck::BadClose(c) => match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("Unexpected bad close {}", c),
            },
        }
    }

    fn syntax(input: &str) -> Self {
        let mut ostack = Vec::new();
        for c in input.trim().chars() {
            match c {
                '(' | '<' | '[' | '{' => ostack.push(c),
                ']' | '>' | ')' | '}' => {
                    let o = ostack[ostack.len() - 1];
                    match (o, c) {
                        ('(', ')') => {}
                        ('[', ']') => {}
                        ('{', '}') => {}
                        ('<', '>') => {}
                        _ => return Self::BadClose(c),
                    }
                    ostack.pop();
                }

                _ => panic!("Bad character {}", c),
            }
        }
        if ostack.is_empty() {
            Self::OK
        } else {
            Self::Incomplete(ostack)
        }
    }

    fn incomplete_score(&self) -> Option<u64> {
        match self {
            SCheck::OK => None,
            SCheck::BadClose(_) => None,
            SCheck::Incomplete(seq) => Some(
                seq.iter()
                    .rev()
                    .copied()
                    .map(|c| match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("Unexpected open {}", c),
                    })
                    .fold(0, |acc, v| (acc * 5) + v),
            ),
        }
    }
}

fn part1(input: &[String]) -> u64 {
    input.iter().map(|s| SCheck::syntax(s).badscore()).sum()
}

fn part2(input: &[String]) -> u64 {
    let mut scores = input
        .iter()
        .flat_map(|s| SCheck::syntax(s).incomplete_score())
        .collect_vec();
    scores.sort_unstable();
    assert_eq!(scores.len() % 2, 1);
    scores[scores.len() >> 1]
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    #[test]
    fn testcase1() {
        let input: Vec<String> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 26397);
    }

    #[test]
    fn testcase2() {
        let input: Vec<String> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 288957);
    }
}

fn main() -> Result<()> {
    let input: Vec<String> = read_input_as_vec(10)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
