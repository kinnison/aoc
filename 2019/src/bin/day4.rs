use aoc2019::*;

fn passcode_valid_1(code: [u8; 6]) -> bool {
    let mut seen_repeat = false;
    for win in code.windows(2) {
        if win[0] > win[1] {
            return false;
        }
        if win[0] == win[1] {
            seen_repeat = true;
        }
    }
    seen_repeat
}

fn passcode_from_str(s: &str) -> [u8; 6] {
    assert!(s.len() == 6);
    let mut ret = [0; 6];
    for (i, c) in s.bytes().enumerate() {
        ret[i] = c - b'0';
    }
    ret
}

fn passcode_valid_2(code: [u8; 6]) -> bool {
    for i in 1..6 {
        if code[i] < code[i - 1] {
            return false;
        }
    }
    code.iter()
        .group_by(|v| *v)
        .into_iter()
        .map(|(_, g)| g.count())
        .any(|v| v == 2)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn passcodes_from_str() {
        assert_eq!(passcode_from_str("123456"), [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_cases_1() {
        assert!(passcode_valid_1(passcode_from_str("122345")));
        assert!(passcode_valid_1(passcode_from_str("111123")));
        assert!(passcode_valid_1(passcode_from_str("111111")));
        assert!(!passcode_valid_1(passcode_from_str("223450")));
        assert!(!passcode_valid_1(passcode_from_str("123789")));
    }

    #[test]
    fn test_cases_2() {
        assert!(passcode_valid_2(passcode_from_str("122345")));
        assert!(!passcode_valid_2(passcode_from_str("111123")));
        assert!(!passcode_valid_2(passcode_from_str("111111")));
        assert!(!passcode_valid_2(passcode_from_str("223450")));
        assert!(!passcode_valid_2(passcode_from_str("123789")));
        assert!(!passcode_valid_2(passcode_from_str("123444")));
        assert!(!passcode_valid_2(passcode_from_str("666999")));
        assert!(passcode_valid_2(passcode_from_str("111122")));
        assert!(passcode_valid_2(passcode_from_str("112233")));
    }
}

fn loop_check_codes<F>(first: &str, second: &str, check_valid: F) -> usize
where
    F: Fn([u8; 6]) -> bool,
{
    let mut pos = passcode_from_str(first);
    let mut ret = 0;
    if check_valid(pos) {
        ret += 1;
    }
    let stop_at = passcode_from_str(second);
    loop {
        pos[5] += 1;
        for i in (0..6).rev() {
            if pos[i] == 10 {
                pos[i] = 0;
                pos[i - 1] += 1;
            } else {
                break;
            }
        }
        if check_valid(pos) {
            ret += 1;
        }
        if pos == stop_at {
            break;
        }
    }

    ret
}

fn part1(first: &str, second: &str) -> usize {
    loop_check_codes(first, second, passcode_valid_1)
}

fn part2(first: &str, second: &str) -> usize {
    loop_check_codes(first, second, passcode_valid_2)
}

fn main() -> Result<()> {
    let input = read_input(4)?;
    let (first, second) = input.split_at(input.find('-').unwrap());
    let second = &second[1..];
    println!("first: {}", first);
    println!("second: {}", second);
    println!("Part 1: {}", part1(first, second));
    println!("Part 1: {}", part2(first, second));
    Ok(())
}
