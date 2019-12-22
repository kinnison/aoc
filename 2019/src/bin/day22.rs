use aoc2019::*;

fn modmult(a: i128, b: i128, m: i128) -> i128 {
    let ret = (a * b) % m;
    if ret < 0 {
        ret + m
    } else {
        ret
    }
}

#[derive(Debug, Copy, Clone, ParseByRegex)]
enum ShuffOp {
    #[regex = r"deal into new stack"]
    Reverse,
    #[regex = r"cut (-?\d+)"]
    Cut(i128),
    #[regex = "deal with increment ([0-9]+)"]
    Modulo(i128),
}

// Critical observation for today's puzzle part 1:
// Each operation can be mathematically mapped therefoew
// we shouldn't need to shuffle.

impl ShuffOp {
    fn project_cardpos(&self, cardpos: i128, cardcount: i128) -> i128 {
        use ShuffOp::*;
        match self {
            Reverse => cardcount - (cardpos + 1),
            Modulo(n) => modmult(cardpos, *n, cardcount),
            Cut(n) => modmult(cardpos - n, 1, cardcount),
        }
    }
}

fn part1(input: &[ShuffOp]) -> i128 {
    input
        .iter()
        .fold(2019, |n, op| op.project_cardpos(n, 10_007))
}

// For part 2, inverting an operation is pretty simple

impl ShuffOp {
    fn project_cardpos_backward(&self, cardpos: i128, cardcount: i128) -> i128 {
        use ShuffOp::*;
        match self {
            // Reverse is its own inverse
            Reverse => cardcount - (cardpos + 1),
            // Cut(n) inverts to Cut(-n)
            Cut(n) => modmult(cardpos + n, 1, cardcount),
            // Modulo is the hard one, we need the modular inverse
            Modulo(n) => modmult(cardpos, modinverse(*n, cardcount).unwrap(), cardcount),
        }
    }
}

// So test that
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_backwards_projector() {
        let rules: Vec<ShuffOp> = read_input_as_vec(22).expect("Bad input");
        for i in 0..10_007 {
            let targ = rules.iter().fold(i, |n, op| op.project_cardpos(n, 10_007));
            let rev = rules
                .iter()
                .rev()
                .fold(targ, |n, op| op.project_cardpos_backward(n, 10_007));
            assert_eq!(i, rev);
        }
    }
}

// Unfortunately doing that for the trillions of operations we need to do will
// take a substantial amount of time
#[allow(dead_code)]
fn naive_part2(input: &[ShuffOp]) -> i128 {
    // Find out which card ends up in position 2020
    let rules: Vec<_> = input.iter().rev().copied().collect();
    let mut pos = 2020;

    static DECK: i128 = 119_315_717_514_047;

    for _ in 0..101_741_582_076_661usize {
        pos = rules
            .iter()
            .fold(pos, |pos, op| op.project_cardpos_backward(pos, DECK));
    }

    pos
}

// It ought to be possible to simplify any sequence of operations
// To do that we can certainly simplify pairs of operations
// We also try and bubble Modulo operations one way and Cut operations the other
// in the hope that we can simplify repeatedly

impl ShuffOp {
    fn simplify_sequence(input: &[ShuffOp], decksize: i128) -> Vec<ShuffOp> {
        let mut ret = input.to_vec();
        let mut ofs = 0;
        while ofs < ret.len() - 1 {
            // At the end, ofs is ret.len() - 2 which means that
            // ofs+1 is ret.len()-1 which is the last item, so this
            // is always safe
            let op1 = ret[ofs];
            let op2 = ret[ofs + 1];
            use ShuffOp::*;
            ofs = match (op1, op2) {
                (Reverse, Reverse) => {
                    // Two reverses, cancel
                    ret.remove(ofs);
                    ret.remove(ofs);
                    0
                }
                (Cut(n1), Cut(n2)) => {
                    // Two cuts, just add the offsets
                    ret[ofs] = Cut(modmult(n1 + n2, 1, decksize));
                    ret.remove(ofs + 1);
                    0
                }
                (Modulo(n1), Modulo(n2)) => {
                    // Two deal-outs, just multiply the offsets
                    ret[ofs] = Modulo(modmult(n1, n2, decksize));
                    ret.remove(ofs + 1);
                    0
                }
                (Cut(cn), Modulo(mn)) => {
                    // Cut then deal becomes deal then cut with the multiple
                    // of the offsets
                    ret[ofs] = Modulo(mn);
                    ret[ofs + 1] = Cut(modmult(cn, mn, decksize));
                    0
                }
                (Reverse, Modulo(mn)) => {
                    // Reverse-then-modulo is a bit harder
                    // This one actually gets bigger
                    ret[ofs] = Modulo(mn);
                    ret[ofs + 1] = Cut(-mn + 1);
                    ret.insert(ofs + 2, Reverse);
                    0
                }
                (Reverse, Cut(cn)) => {
                    // Reverse, then cut can become invert cut then reverse
                    ret[ofs] = Cut(-cn);
                    ret[ofs + 1] = Reverse;
                    0
                }
                (_, _) => {
                    // Anything else, we move onward
                    ofs + 1
                }
            };
        }

        ret
    }
}

#[cfg(test)]
mod test2 {
    use super::*;
    #[test]
    fn verify_simplification() {
        let input: Vec<ShuffOp> = read_input_as_vec(22).expect("Bad input?");
        let simple = ShuffOp::simplify_sequence(&input, 10_007);
        for i in 0..10_007 {
            assert_eq!(
                input.iter().fold(i, |n, op| op.project_cardpos(n, 10_007)),
                simple.iter().fold(i, |n, op| op.project_cardpos(n, 10_007))
            );
        }
    }
}

// Now that we can simplify what we're up to, we can probably build out a
// power set to cope with part 2...
static P2REPEAT: i128 = 101_741_582_076_661;
static P2DECKSIZE: i128 = 119_315_717_514_047;
fn p2_repeat_shuffle(input: &[ShuffOp]) -> Vec<ShuffOp> {
    // Our goal is to repeat the input P2REPEAT times as though the deck
    // were P2DECKSIZE large
    // To do that, we can use powers of 2...
    let mut powers: Vec<Vec<ShuffOp>> = Vec::new();
    let mut p2: i128 = 1;
    let mut steps = input.to_vec();
    while p2 < P2REPEAT {
        powers.push(steps.clone());
        p2 <<= 1;
        let newlen = steps.len() * 2;
        let newsteps: Vec<_> = steps.into_iter().cycle().take(newlen).collect();
        steps = ShuffOp::simplify_sequence(&newsteps, P2DECKSIZE);
    }
    // Now powers is a vector where each entry is a simplified sequence
    // for running the input sequence by that power
    steps = Vec::new();
    #[allow(clippy::needless_range_loop)]
    for b in 0..127 {
        if (P2REPEAT & (1 << b)) != 0 {
            steps.extend_from_slice(&powers[b]);
        }
    }
    ShuffOp::simplify_sequence(&steps, P2DECKSIZE)
}

fn part2(input: &[ShuffOp]) -> i128 {
    // First up acquire the simplified repeated seq
    let seq = p2_repeat_shuffle(input);
    // Next we're going to apply that in reverse to 2020
    seq.iter()
        .rev()
        .fold(2020, |n, op| op.project_cardpos_backward(n, P2DECKSIZE))
}

fn main() -> Result<()> {
    let input: Vec<ShuffOp> = read_input_as_vec(22)?;

    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", naive_part2(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
