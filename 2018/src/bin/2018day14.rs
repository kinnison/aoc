use aoc2018::*;

struct RecipeBoard {
    scores: Vec<u8>,
    elf1: usize,
    elf2: usize,
}

impl RecipeBoard {
    fn new() -> RecipeBoard {
        RecipeBoard {
            scores: vec![3, 7],
            elf1: 0,
            elf2: 1,
        }
    }

    fn tick(&mut self) {
        let elf1score = self.scores[self.elf1] as usize;
        let elf2score = self.scores[self.elf2] as usize;
        let sum: usize = elf1score + elf2score;
        // Adding two digits *always* produces a value of 18 or fewer, as such
        // we're creating either 1 or 2 recipes
        if sum >= 10 {
            self.scores.push(1);
            self.scores.push((sum - 10) as u8);
        } else {
            self.scores.push(sum as u8);
        }
        self.elf1 = (self.elf1 + 1 + elf1score) % self.scores.len();
        self.elf2 = (self.elf2 + 1 + elf2score) % self.scores.len();
    }
}

fn part1(input: usize) -> String {
    let mut board = RecipeBoard::new();
    while board.scores.len() < (input + 10 + 2) {
        board.tick();
    }
    let mut ret: String = String::new();
    for i in input..(input + 10) {
        ret.push((board.scores[i] + b'0') as char);
    }
    ret
}

static TEST1: &[(usize, &str)] = &[
    (9, "5158916779"),
    (5, "0124515891"),
    (18, "9251071085"),
    (2018, "5941429882"),
];

fn part2(input: &str) -> usize {
    let mut board = RecipeBoard::new();
    let digits: Vec<u8> = input.bytes().map(|b| b - b'0').collect();
    let mut lastlooked = 0;
    'outer: loop {
        while board.scores.len() < (lastlooked + 10_000) {
            board.tick();
        }
        if let Some(idx) = twoway::find_bytes(&board.scores[lastlooked..], &digits) {
            break 'outer lastlooked + idx;
        }
        lastlooked = board.scores.len() - digits.len();
    }
}

static TEST2: &[(&str, usize)] = &[("51589", 9), ("01245", 5), ("92510", 18), ("59414", 2018)];

fn main() -> Result<()> {
    for test in TEST1 {
        assert_eq!(test.1, part1(test.0));
    }
    for test in TEST2 {
        assert_eq!(test.1, part2(test.0));
    }
    println!("Tests passed.");
    let input = read_input(14)?;
    let input = input.trim();
    println!("Part 1: {}", part1(input.parse()?));
    println!("Part 2: {}", part2(input));
    Ok(())
}
