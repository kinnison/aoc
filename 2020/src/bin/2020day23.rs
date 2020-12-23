use aoc2020::*;

fn do_move(circle: &mut VecDeque<u32>) {
    let vlen = circle.len() as u32;
    let curcup = circle[0];
    circle.rotate_left(1);
    let cup1 = circle.pop_front().unwrap();
    let cup2 = circle.pop_front().unwrap();
    let cup3 = circle.pop_front().unwrap();
    let mut goal = curcup - 1;
    while goal == 0 || goal == cup1 || goal == cup2 || goal == cup3 {
        goal = (goal + vlen) % (vlen + 1);
    }
    let pos = circle
        .iter()
        .copied()
        .enumerate()
        .find(|(_, cup)| *cup == goal)
        .unwrap()
        .0;
    circle.rotate_left(pos + 1);
    circle.push_front(cup3);
    circle.push_front(cup2);
    circle.push_front(cup1);
    circle.rotate_right(pos + 1);
}

fn run_rounds(circle: &mut VecDeque<u32>, rounds: usize) {
    for n in 1..=rounds {
        if (n % 1000) == 0 {
            println!("Done {} rounds", n);
        }
        do_move(circle);
    }
}

fn gen_label(circle: &mut VecDeque<u32>) -> String {
    let mut ret = String::new();
    while circle[0] != 1 {
        circle.rotate_left(1);
    }
    for _ in 0..8 {
        circle.rotate_left(1);
        ret.push((circle[0] as u8 + b'0') as char);
    }
    ret
}

fn part1(input: &VecDeque<u32>) -> String {
    let mut circle = input.clone();
    run_rounds(&mut circle, 100);
    gen_label(&mut circle)
}

fn part2(input: &VecDeque<u32>) -> u64 {
    let mut circle = input.clone();
    circle.reserve(1_000_000 - 9);
    for i in 10..=1_000_000 {
        circle.push_back(i);
    }
    run_rounds(&mut circle, 10_000_000);
    while circle[0] != 1 {
        circle.rotate_left(1);
    }
    (circle[1] as u64) * (circle[2] as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"389125467"#;

    #[test]
    fn testcase1() {
        let mut input = TEST_INPUT
            .chars()
            .map(|c| (c as u32) - (b'0' as u32))
            .collect();
        run_rounds(&mut input, 10);
        assert_eq!(gen_label(&mut input), "92658374");
        let input = TEST_INPUT
            .chars()
            .map(|c| (c as u32) - (b'0' as u32))
            .collect();
        assert_eq!(part1(&input), "67384529");
    }

    #[test]
    fn testcase2() {
        let input = TEST_INPUT
            .chars()
            .map(|c| (c as u32) - (b'0' as u32))
            .collect();
        assert_eq!(part2(&input), 149245887792);
    }
}

fn main() -> Result<()> {
    let input: String = read_input(23)?;
    let input = input
        .trim()
        .chars()
        .map(|c| (c as u32) - (b'0' as u32))
        .collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
