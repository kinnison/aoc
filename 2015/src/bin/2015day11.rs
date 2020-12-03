use aoc2015::*;

#[derive(Clone)]
struct Password {
    inner: Vec<u8>,
}

impl Password {
    fn from_str(input: &str) -> Password {
        Password {
            inner: input.trim().bytes().map(|ch| ch - b'a').collect(),
        }
    }

    fn increment(&mut self) {
        let mut idx = 7;
        loop {
            self.inner[idx] += 1;
            if self.inner[idx] < 26 {
                break;
            }
            self.inner[idx] = 0;
            idx -= 1;
        }
    }

    fn requirements_met(&self) -> bool {
        // Passwords must include one increasing straight of at least three letters,
        // like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
        if !self
            .inner
            .windows(3)
            .any(|win| (win[2] == win[1] + 1) && (win[1] == win[0] + 1))
        {
            return false;
        }

        // Passwords may not contain the letters i, o, or l, as these letters can
        // be mistaken for other characters and are therefore confusing.
        if self.inner.iter().any(|&ch| ch == 8 || ch == 14 || ch == 11) {
            return false;
        }

        // Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.
        for idx in 0..6 {
            if self.inner[idx] == self.inner[idx + 1] {
                for win in self.inner[idx + 2..].windows(2) {
                    if win[0] == win[1] {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn increment_to_valid(&mut self) {
        loop {
            self.increment();
            if self.requirements_met() {
                break;
            }
        }
    }

    fn as_string(&self) -> String {
        self.inner.iter().map(|ch| (ch + b'a') as char).collect()
    }
}

fn part1(input: &Password) -> String {
    let mut v = input.clone();
    v.increment_to_valid();
    v.as_string()
}

fn part2(input: &Password) -> String {
    let mut v = input.clone();
    v.increment_to_valid();
    v.increment_to_valid();
    v.as_string()
}

fn main() -> Result<()> {
    //test();
    let input = Password::from_str(&read_input(11)?);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
