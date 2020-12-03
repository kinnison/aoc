macro_rules! DISC_N {
    () => {
        6
    };
}

static PUZZLE_INPUT: [(usize, usize, usize, usize); DISC_N!()] = [
    // (disc, posn, time, pos)
    (1, 5, 0, 2),
    (2, 13, 0, 7),
    (3, 17, 0, 10),
    (4, 3, 0, 2),
    (5, 19, 0, 9),
    (6, 7, 0, 0),
];

struct PuzzleState {
    sizes: [usize; DISC_N!() + 1],
    offsets: [usize; DISC_N!() + 1],
}

/*
 * In puzzle 1 at least, discs are offset by 1s in time each
 * And the goal is to make them "line up" in position zero at time N
 * which means disc 1 must be in position 0 at time N+1, disc 2 at N+2 etc.
 * Easiest way to do this is to initialise the state and offset by the
 * disc number at the same time, so we don't have to worry about it
 * in the future...
 */

fn initial_state() -> PuzzleState {
    let mut ret: PuzzleState = PuzzleState {
        sizes: [0; DISC_N!() + 1],
        offsets: [0; DISC_N!() + 1],
    };

    for (discn, size, at, ofs) in PUZZLE_INPUT.iter().copied() {
        // Since we're hoping to make it so that at time N the value
        // we store in the puzzle state is such that (N+offset)%size == 0
        // if the disc would be in the right place when we release at N
        ret.sizes[discn - 1] = size;
        ret.offsets[discn - 1] = (ofs + discn + size - at) % size;
    }

    // the last disc is initialised to something which will always be safe...
    ret.sizes[DISC_N!()] = 1;
    ret.offsets[DISC_N!()] = 0;
    ret
}

fn aligned(s: &PuzzleState, at: usize) -> bool {
    for i in 0..DISC_N!() + 1 {
        if ((at + s.offsets[i]) % s.sizes[i]) != 0 {
            return false;
        }
    }
    true
}

fn prob_opt(s: &PuzzleState) -> (usize, usize) {
    // to optimise we select a start N and step which means we only
    // test values where the largest disc might be in alignment...
    let mut dn: usize = 0;
    let mut ds: usize = s.sizes[0];
    for i in 1..DISC_N!() + 1 {
        if s.sizes[i] > ds {
            ds = s.sizes[i];
            dn = i;
        }
    }
    // At this point the largest disc is index dn
    // and its size is ds.  We know the offset of the disc
    let dofs: usize = s.offsets[dn];
    // the disc will be at 0 when (dofs+N)%ds == 0 since dofs < ds by design
    // we can represent that as dofs+N == ds, which means N == ds-dofs
    (ds - dofs, ds)
}

fn problem1() -> usize {
    let state = initial_state();
    let (mut n, step) = prob_opt(&state);
    while !aligned(&state, n) {
        n += step;
    }
    n
}

fn problem2() -> usize {
    let mut state = initial_state();
    // Add in the final disc
    state.sizes[DISC_N!()] = 11;
    state.offsets[DISC_N!()] = (DISC_N!() + 1 + 11) % 11;
    let (mut n, _step) = prob_opt(&state);
    while !aligned(&state, n) {
        n += 1;
    }
    n
}

fn main() {
    println!("Problem 1: {}", problem1());
    println!("Problem 2: {}", problem2());
}
