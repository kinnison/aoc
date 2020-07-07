use aoc2018::*;

#[derive(Copy, Clone, ParseByRegex)]
#[regex = r"^(?P<players>\d+) players;.*worth (?P<points>\d+) points$"]
struct Setup {
    players: usize,
    points: usize,
}

static TEST_VEC: &[(usize, usize, usize)] = &[
    (9, 25, 32),
    (10, 1618, 8_317),
    (13, 7999, 146_373),
    (17, 1104, 2764),
    (21, 6111, 54_718),
    (30, 5807, 37_305),
];

struct Circle {
    marbles: VecDeque<usize>,
}

impl Circle {
    fn new(marbles: usize) -> Circle {
        let mut marbles = VecDeque::with_capacity(marbles);
        marbles.push_front(0);
        Circle { marbles }
    }

    fn rotate_clockwise(&mut self, n: usize) {
        // To rotate clockwise is to move the current insertion pointer clockwise
        // which is to pop front end push back
        for _i in 0..n {
            let val = self.marbles.pop_front().unwrap();
            self.marbles.push_back(val);
        }
    }

    fn rotate_counter_clockwise(&mut self, n: usize) {
        // To rotate CCW is to move CCW which is pop back, push front
        for _i in 0..n {
            let val = self.marbles.pop_back().unwrap();
            self.marbles.push_front(val);
        }
    }

    fn make_move(&mut self, marbnum: usize) -> usize {
        // Making a move can result in zero or more points.  Never negative
        // points because of the marble ordering...
        if (marbnum % 23) == 0 {
            // Special move
            // Remove the marble 7 counterclockwise and return its score
            self.rotate_counter_clockwise(7);
            marbnum + self.marbles.pop_front().unwrap()
        } else {
            // Normal move, insert marbnum between 1 and 2 clockwise of cur
            // That means, calculate offset 2 clockwise of cur, then insert at
            // that position, shuffling the rest up.
            self.rotate_clockwise(2);
            self.marbles.push_front(marbnum);
            0
        }
    }
}

struct Game {
    setup: Setup,
    circle: Circle,
    scores: Vec<usize>,
}

impl Game {
    fn new(setup: &Setup) -> Game {
        Game {
            setup: *setup,
            circle: Circle::new(setup.points),
            scores: (0..setup.players).map(|_| 0).collect(),
        }
    }

    fn highest_score(&self) -> usize {
        self.scores.iter().cloned().max().expect("No players?")
    }

    fn run(&mut self) {
        for marble in 1..=self.setup.points {
            let player = (marble - 1) % self.setup.players;
            let scored = self.circle.make_move(marble);
            self.scores[player] += scored;
        }
    }
}

fn part1(setup: &Setup) -> usize {
    let mut game = Game::new(setup);
    game.run();
    game.highest_score()
}

fn main() -> Result<()> {
    for &test1 in TEST_VEC.iter() {
        let setup = Setup {
            players: test1.0,
            points: test1.1,
        };
        println!(
            "Test 1: {} players; {} points; should score {}, scored {}",
            test1.0,
            test1.1,
            test1.2,
            part1(&setup)
        );
        assert_eq!(part1(&setup), test1.2);
    }
    let input: Setup = ParseByRegex::parse_by_regex(read_input(9)?.trim())?;
    println!("Part 1: {}", part1(&input));
    let mut other = input;
    other.points *= 100;
    println!("Part 2: {}", part1(&other));
    Ok(())
}
