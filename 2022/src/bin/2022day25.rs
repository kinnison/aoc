use aoc2022::*;

type StorageVal = i64;

struct SnafuNumber(StorageVal);

impl FromStr for SnafuNumber {
    type Err = Infallible;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        // Parsing a snafu number involves reading from right to left with increasing powers of five
        let digits = s
            .trim()
            .chars()
            .rev()
            .enumerate()
            .map(|(p, c)| {
                let power = StorageVal::from(5).pow(p as u32);
                match c {
                    '=' => -2 * power,
                    '-' => -1 * power,
                    '0' => 0,
                    '1' => power,
                    '2' => 2 * power,
                    _ => panic!("Unknown SNAFU digit {c}"),
                }
            })
            .sum();
        Ok(SnafuNumber(digits))
    }
}

impl ToString for SnafuNumber {
    fn to_string(&self) -> String {
        let mut ret = String::new();
        let mut val = self.0;
        while val > 0 {
            let (quo, rem) = (val / 5, val % 5);
            ret.push(match rem {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '=',
                4 => '-',
                _ => unreachable!(),
            });
            if rem <= 2 {
                val = quo;
            } else {
                val = quo + 1;
            }
        }
        if ret.is_empty() {
            ret.push('0');
        }
        ret.chars().rev().collect()
    }
}

fn part1(input: &[SnafuNumber]) -> String {
    SnafuNumber(input.iter().map(|sn| sn.0).sum()).to_string()
}

//fn part2(input: &[SnafuNumber]) -> StorageVal {
//    todo!()
//}

#[cfg(test)]
mod test {
    use super::*;

    static SNAFU_CASES: &[(StorageVal, &str)] = &[
        (1, "            1"),
        (2, "            2"),
        (3, "           1="),
        (4, "           1-"),
        (5, "           10"),
        (6, "           11"),
        (7, "           12"),
        (8, "           2="),
        (9, "           2-"),
        (10, "           20"),
        (15, "          1=0"),
        (20, "          1-0"),
        (2022, "       1=11-2"),
        (12345, "      1-0---0"),
        (314159265, "1121-1110-1=0"),
    ];

    #[test]
    fn cases() {
        for &(dec, snafstr) in SNAFU_CASES {
            let parsed = SnafuNumber::from_str(snafstr.trim()).unwrap();
            assert_eq!(parsed.0, dec);
            assert_eq!(snafstr.trim(), parsed.to_string());
        }
    }

    static TEST_INPUT: &str = r#"1=-0-2
    12111
    2=0=
    21
    2=01
    111
    20012
    112
    1=-1=
    1-12
    12
    1=
    122"#;

    #[test]
    fn testcase1() {
        let input: Vec<SnafuNumber> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), "2=-1=0");
    }

    //#[test]
    //fn testcase2() {
    //    let input: Vec<SnafuNumber> = input_as_vec(TEST_INPUT).unwrap();
    //    assert_eq!(part2(&input), 70);
    //}
}

pub fn main() -> Result<()> {
    let input: Vec<SnafuNumber> = read_input_as_vec(25)?;
    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2(&input));
    Ok(())
}
