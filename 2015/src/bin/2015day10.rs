use aoc2015::*;

#[derive(Clone)]
struct LookAndSay {
    value: Vec<u8>,
}

impl LookAndSay {
    fn from_str(value: &str) -> LookAndSay {
        LookAndSay {
            value: value.trim().bytes().map(|ch| ch - b'0').collect(),
        }
    }

    fn go(&self) -> LookAndSay {
        let mut ret = LookAndSay { value: Vec::new() };

        let mut current = self.value[0];
        let mut count = 1;
        for idx in 1..self.value.len() {
            let ch = self.value[idx];
            if ch == current {
                count += 1;
            } else {
                ret.value.push(count);
                ret.value.push(current);
                current = ch;
                count = 1;
            }
        }

        ret.value.push(count);
        ret.value.push(current);

        ret
    }

    fn len(&self) -> usize {
        self.value.len()
    }
}

fn part1(input: &LookAndSay) -> usize {
    (0..40).fold(input.clone(), |v, _| v.go()).len()
}

fn part2(input: &LookAndSay) -> usize {
    (0..50).fold(input.clone(), |v, _| v.go()).len()
}

fn main() -> Result<()> {
    let input = LookAndSay::from_str(&read_input(10)?);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
