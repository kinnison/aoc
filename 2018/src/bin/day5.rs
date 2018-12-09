use aoc2018::*;

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
struct Ascii(u8);

impl Ascii {
    fn from_char(c: char) -> Ascii {
        Ascii(c as u8)
    }

    fn to_char(self) -> char {
        self.0 as char
    }

    fn pairs_with(self, other: Ascii) -> bool {
        // If they're equal *except* capitalisation then
        // xoring the two together should result in exactly 0x20
        (self.0 ^ other.0) == 0x20
    }
}

fn react(input: &str, dropping: Option<char>) -> String {
    let mut ret: Vec<Ascii> = match dropping {
        None => input.chars().map(Ascii::from_char).collect(),
        Some(ch) => input
            .chars()
            .filter(|&c| c != ch && c.to_ascii_lowercase() != ch)
            .map(Ascii::from_char)
            .collect(),
    };
    'outer: loop {
        if ret.is_empty() {
            break;
        }
        let mut i = 0;
        let mut changed = false;
        'charagain: while i < ret.len() - 1 {
            let (f, s) = (ret[i], ret[i + 1]);
            if f.pairs_with(s) {
                // Annihilate the pair
                ret.drain(i..i + 2);
                changed = true;
                if ret.is_empty() {
                    break 'outer;
                }
                continue 'charagain;
            }
            i += 1;
        }
        if !changed {
            break;
        }
    }
    ret.iter().map(|a| a.to_char()).collect()
}

fn part1(input: &str) -> usize {
    react(input, None).len()
}

fn part2(input: &str) -> usize {
    let chs: HashSet<char> = input.chars().filter(|c| c.is_lowercase()).collect();
    let mut best_size = std::usize::MAX;

    for ch in chs.iter() {
        //println!("Considering {}", ch);
        let size = react(input, Some(*ch)).len();
        if size < best_size {
            best_size = size
        }
    }

    best_size
}

static TESTS: [(&str, &str); 5] = [
    ("aA", ""),
    ("abBA", ""),
    ("abAB", "abAB"),
    ("aabAAB", "aabAAB"),
    ("dabAcCaCBAcCcaDA", "dabCBAcaDA"),
];

static TESTS2: [(char, usize); 4] = [('a', 6), ('b', 8), ('c', 4), ('d', 6)];

fn main() -> Result<()> {
    for (test_input, test_output) in TESTS.iter() {
        assert_eq!(test_output.len(), part1(test_input));
    }
    for (test_ch, test_len) in TESTS2.iter() {
        assert_eq!(*test_len, react("dabAcCaCBAcCcaDA", Some(*test_ch)).len());
    }
    assert_eq!(4, part2("dabAcCaCBAcCcaDA"));
    let input = read_input(5)?.trim().to_owned();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
