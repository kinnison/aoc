use aoc2021::*;

#[derive(ParseByRegex)]
#[regex = r"^Player . starting position: (?P<n>\d+)$"]
struct StartPos {
    n: u64,
}

fn roll_det(die: &mut u64) -> u64 {
    if *die == 100 {
        *die = 1;
    } else {
        *die += 1;
    }
    *die
}
fn part1(input: &[StartPos]) -> u64 {
    // Deterministic game, let's just play it open-coded
    let mut scores = [0, 0];
    let mut pos = [input[0].n, input[1].n];
    let mut turn = 0;
    let mut die = 0; // first roll will turn it to 1
    loop {
        let player = turn % 2;
        turn += 1;
        let roll = roll_det(&mut die) + roll_det(&mut die) + roll_det(&mut die);
        let newpos = ((pos[player] + roll - 1) % 10) + 1;
        pos[player] = newpos;
        scores[player] += newpos;
        if scores[player] >= 1000 {
            // This player wins, score is the other player's score
            // multiplied by the count of die rolls (3x turn count)
            break scores[(player + 1) % 2] * 3 * (turn as u64);
        }
    }
}

// Thoughts, when you roll 3 dirac dice the outcome of each die is 1, 2, or 3
// which means you get totals of: 3, 4, 5, 6, 7, 8, 9 with frequencies of
// 1, 3, 6, 7, 6, 3, 1 which totals 27 different ways to roll each turn
// Since your score after a turn is your position on the track, you could
// score from 1 to 10, so at absolute worst, let's assume the winning player
// is in position 1 and keeps rolling dice which means they stay at position 1,
// it takes 21 turns to complete the game since the target goal is 21
//
// Since there are only 7 different ways to roll at each point in the game,
// and there are a finite 100 places the players could be (1->10)*(1->10)
// and a finite set of scores the players could have (0->20)*(0->20)
// in theory there'll be plenty of times the position/score overlap.
// Since the total game state count is 10*10*20*20 == 40,000 this is eminiently
// smaller than the total potential number of game sequences (over 558545864083284007)
// so let's try memoizing based on state to make life easier.
// In practice, only a very small number of games will take a player 21 turns, to win, so while
// the theoretical maximum is high, memoizing should mean we're not actually O(n^2)
#[memoize]
fn take_dirac_turn(pos: [u64; 2], score: [u64; 2], player: usize) -> [u64; 2] {
    const ROLLS: [u64; 27] = [
        3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 9,
    ];

    // How many times each player wins on this turn
    let mut wins = [0, 0];
    for roll in ROLLS {
        let mut pos = pos;
        let mut score = score;
        // We're rolling `roll` for `player`
        pos[player] += roll;
        if pos[player] > 10 {
            pos[player] -= 10
        }
        score[player] += pos[player];
        if score[player] >= 21 {
            // We have a win
            wins[player] += 1;
        } else {
            // We do not have a win, so roll again
            let subseq_wins = take_dirac_turn(pos, score, (player + 1) & 1);
            wins[0] += subseq_wins[0];
            wins[1] += subseq_wins[1];
        }
    }

    wins
}

fn part2(input: &[StartPos]) -> u64 {
    let wins = take_dirac_turn([input[0].n, input[1].n], [0, 0], 0);
    max(wins[0], wins[1])
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"Player 1 starting position: 4
Player 2 starting position: 8"#;

    #[test]
    fn testcase1() {
        let input: Vec<StartPos> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 739785);
    }

    #[test]
    fn testcase2() {
        let input: Vec<StartPos> = input_as_vec(TEST_INPUT).unwrap();
        let wins = take_dirac_turn([input[0].n, input[1].n], [0, 0], 0);
        assert_eq!(wins, [444356092776315, 341960390180808]);
    }
}

fn main() -> Result<()> {
    let input: Vec<StartPos> = read_input_as_vec(21)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
