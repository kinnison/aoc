use std::collections::HashMap;

// Ulam Spiral

const INPUT: i32 = 265149;

fn first(cycle: i32) -> i32 {
    ((cycle << 1) - 1) * ((cycle << 1) - 1)
}

fn cycleof(pos: i32) -> i32 {
    (((pos as f64).sqrt() as i32) + 1) >> 1
}

fn length(cycle: i32) -> i32 {
    cycle * 8
}

fn sector(index: i32) -> i32 {
    let c = cycleof(index);
    let offset = index - first(c);
    let n = length(c);
    (4 * offset) / n
}

fn pos_of(index: i32) -> (i32, i32) {
    let c = cycleof(index);
    let s = sector(index);
    let offset = index - first(c) - ((s * length(c)) >> 2);
    match s {
        0 => (-c, (-c + offset) + 1), // North
        1 => ((-c + offset) + 1, c),  // East
        2 => (c, (c - offset) - 1),   // South
        3 => ((c - offset) - 1, -c),  // West
        _ => panic!("Unknown sector {}", s),
    }
}

fn manhat(index: i32) -> i32 {
    let (vert, horz) = pos_of(index);
    vert.abs() + horz.abs()
}

fn problem1() -> i32 {
    let idx: i32 = INPUT - 1; // Shift to zero index for cycle etc to work
    manhat(idx)
}

fn problem2(minv: i32) -> i32 {
    let mut written = HashMap::new();
    written.insert((0, 0), 1);
    for i in 1.. {
        let mut sum: i32 = 0;
        let (a, b) = pos_of(i);
        for a_ofs in -1..2 {
            for b_ofs in -1..2 {
                if (a_ofs != 0) || (b_ofs != 0) {
                    sum += *written.entry(((a + a_ofs), (b + b_ofs))).or_insert(0);
                }
            }
        }
        written.insert((a, b), sum);
        if sum > minv {
            return sum;
        }
    }
    unreachable!();
}

fn main() {
    println!("Problem 1: {}", problem1());
    println!("Problem 2: {}", problem2(INPUT));
}
