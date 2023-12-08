use aoc2023::*;

pub fn main() -> Result<()> {
    let input = read_input(7)?;
    let input = parse_hands(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Debug)]
struct Hands {
    hands: Vec<String>,
    scores: Vec<u64>,
    joker_scores: Vec<u64>,
    bets: Vec<u64>,
}

fn parse_hands(input: &str) -> Hands {
    let mut hands = Vec::new();
    let mut scores = Vec::new();
    let mut joker_scores = Vec::new();
    let mut bets = Vec::new();

    for row in input.trim().lines() {
        let (hand, bet) = row.split_once(' ').unwrap();
        hands.push(hand.into());
        scores.push(score_hand(hand));
        joker_scores.push(
            "23456789TQKA"
                .chars()
                .map(|joker| joker_score_hand(hand, joker))
                .max()
                .unwrap(),
        );
        bets.push(bet.parse().unwrap());
    }

    Hands {
        hands,
        scores,
        joker_scores,
        bets,
    }
}

fn card_score(card: char) -> u64 {
    match card {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => unreachable!(),
    }
}

fn score_hand(hand: &str) -> u64 {
    // Hand score is as follows:  Kind, then cards.
    // Kind is one of:
    // Five of a kind, four of a kind, full house, three of a kind, two pair, one pair, nothing
    let mut cards: HashMap<char, usize> = HashMap::new();
    let mut score = 0;

    for ch in hand.chars() {
        *cards.entry(ch).or_default() += 1;
        score <<= 4;
        score |= card_score(ch);
    }

    // 0xCCCCC (five cards)

    let kind = match cards.len() {
        5 => 0, // nothing
        4 => 1, // one pair
        3 => {
            if !cards.values().any(|&v| v == 3) {
                // 2 pair
                2
            } else {
                // 3 of a kind
                3
            }
        }
        2 => {
            if !cards.values().any(|&v| v == 4) {
                // Full house
                4
            } else {
                // Four of a kind
                5
            }
        }
        1 => 6, // Five of a kind
        _ => unreachable!(),
    };

    score | (kind << 20)
}

fn joker_card_score(card: char) -> u64 {
    match card {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => unreachable!(),
    }
}

fn joker_score_hand(hand: &str, joker: char) -> u64 {
    // Hand score is as follows:  Kind, then cards.
    // Kind is one of:
    // Five of a kind, four of a kind, full house, three of a kind, two pair, one pair, nothing
    let mut cards: HashMap<char, usize> = HashMap::new();
    let mut score = 0;

    for mut ch in hand.chars() {
        score <<= 4;
        score |= joker_card_score(ch);
        if ch == 'J' {
            ch = joker;
        }
        *cards.entry(ch).or_default() += 1;
    }

    // 0xCCCCC (five cards)

    let kind = match cards.len() {
        5 => 0, // nothing
        4 => 1, // one pair
        3 => {
            if !cards.values().any(|&v| v == 3) {
                // 2 pair
                2
            } else {
                // 3 of a kind
                3
            }
        }
        2 => {
            if !cards.values().any(|&v| v == 4) {
                // Full house
                4
            } else {
                // Four of a kind
                5
            }
        }
        1 => 6, // Five of a kind
        _ => unreachable!(),
    };

    score | (kind << 20)
}

fn part1(input: &Hands) -> u64 {
    // We rank the hands / scores, and then compute the sum
    let mut ranks = (0..input.hands.len()).collect_vec();
    ranks.sort_by_key(|rank| input.scores[*rank]);

    ranks
        .into_iter()
        .enumerate()
        .map(|(rank, handnr)| input.bets[handnr] * ((rank as u64) + 1))
        .sum()
}

fn part2(input: &Hands) -> u64 {
    // We rank the hands / joker_scores, and then compute the sum
    let mut ranks = (0..input.hands.len()).collect_vec();
    ranks.sort_by_key(|rank| input.joker_scores[*rank]);

    ranks
        .into_iter()
        .enumerate()
        .map(|(rank, handnr)| input.bets[handnr] * ((rank as u64) + 1))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn testcase1() {
        let input = parse_hands(TEST_INPUT);
        eprintln!("{input:?}");
        assert_eq!(part1(&input), 6440);
    }

    #[test]
    fn testcase2() {
        let input = parse_hands(TEST_INPUT);
        assert_eq!(part2(&input), 5905);
    }
}
