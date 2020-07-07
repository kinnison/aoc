use aoc2019::*;

static PATTERN: [i8; 4] = [0, 1, 0, -1];

#[derive(Debug, Clone)]
struct FFT {
    value: Vec<i8>,
}

impl FFT {
    fn new(input: &[u8]) -> Self {
        Self {
            value: input.iter().copied().map(|i| (i - b'0') as i8).collect(),
        }
    }

    fn get_output(&self) -> String {
        self.value
            .iter()
            .copied()
            .map(|i| ((i as u8) + b'0') as char)
            .take(8)
            .collect()
    }

    fn run_phases(&mut self) {
        for _ in 0..100 {
            self.run_one_phase();
        }
    }

    fn run_one_phase(&mut self) {
        let mut phase_output = Vec::with_capacity(self.value.len());

        for i in 1..=self.value.len() {
            let comb = PATTERN
                .iter()
                .copied()
                .flat_map(|n| std::iter::repeat(n as i32).take(i))
                .cycle()
                .skip(1);
            let total: i32 = self
                .value
                .iter()
                .copied()
                .zip(comb)
                .map(|(v, c)| (v as i32) * c)
                .sum();
            phase_output.push((total.abs() % 10) as i8);
        }

        self.value = phase_output;
    }
}

fn part1(input: &str) -> String {
    let mut copy = FFT::new(input.as_bytes());
    copy.run_phases();
    copy.get_output()
}

fn part2(input: &str) -> String {
    // The critical observation here is that the pattern [0, 1, 0, -1]
    // repeats each entry by the offset and that for index N into the sequence,
    // the first 1 is at that index.  As such, if the offset is at least halfway
    // through the repeated sequence, then:
    // 1. All input digits before the offset have 0 factors and so are not
    //    affecting the output at all
    // 2. All input digits at or after the offset have 1 factors, and so we need
    //    simply to do a sum of the rest of the digits each time
    // 3. By reversing the suffix, we can do that by scanning the input
    //    and the first (last) digit will always be itself, and each subsequent
    //    one will be the sum of those which went before
    let offset: usize = input[..7].parse().expect("Unable to parse offset");
    // The offset needs to be more than halfway
    assert!(offset >= (input.len() * 5_000));
    // The suffix we care about
    let suffix_len = (input.len() * 10_000) - offset;
    // Calculate by reversing the input, cycling that, and taking the suffix
    let mut suffix: Vec<_> = input
        .as_bytes()
        .iter()
        .copied()
        .rev()
        .cycle()
        .take(suffix_len)
        .map(|n| (n - b'0') as i32)
        .collect();
    // Now run the phases as a partial sum
    for _ in 0..100 {
        let next: Vec<_> = suffix
            .iter()
            // Scan is like fold, only it yields a value per entry in the
            // iteration, so we walk the list, returning the partial sums
            // as we go
            .scan(0, |sum, x| {
                *sum += x;
                Some(*sum % 10)
            })
            .collect();
        suffix = next;
    }

    // Finally grab the value back out by reversing the suffix and taking the
    // first 8 bytes to render as a string
    suffix
        .iter()
        .rev()
        .take(8)
        .copied()
        .map(|v| ((v as u8) + b'0') as char)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cases_1() {
        static TESTS: &[(&str, &str)] = &[
            ("80871224585914546619083218645595", "24176176"),
            ("19617804207202209144916044189917", "73745418"),
            ("69317163492948606335995924319873", "52432133"),
        ];
        for (input, check) in TESTS {
            assert_eq!(&part1(input), check);
        }
    }

    #[test]
    fn test_cases_2() {
        static TESTS: &[(&str, &str)] = &[
            ("03036732577212944063491565474664", "84462026"),
            ("02935109699940807407585447034323", "78725270"),
            ("03081770884921959731165446850517", "53553731"),
        ];
        for (input, check) in TESTS {
            assert_eq!(&part2(input), check);
        }
    }
}

fn main() -> Result<()> {
    let input = read_input(16)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
