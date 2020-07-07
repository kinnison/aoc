// I think at least part 1 is the Josephus problem.
// generalised to k=2
// which means the "winner" is the elf in position 2l-1 where
// l is the number of elves minus the top set bit.

fn winner1(elves: u32) -> u32 {
    let l = elves - (elves.next_power_of_two() >> 1);
    (2 * l) + 1
}

// Part 2 changes slightly because we're eliminating the elf
// "across" the circle rather than the next elf, which means
// we're not doing k=2 but rather k=(n/2) for each elimination
//
// I don't know a direct solution for that, but brief research
// suggests that:
// given k(n) = n / 2
// f(n) = (f(n-1) + k(n) - 1) % n + 1
// where
// f(1) = 1

/* This implementation is poor because it uses a LOT of stack
fn winner2 (elves: u32) -> u32 {
    if elves == 1 {
        1
    } else {
        ((winner2(elves - 1) + (elves >> 1) - 1) % elves) + 1
    }
}
*/

fn winner2(n_elves: u32) -> u32 {
    let mut elves: Vec<u32> = Vec::new();
    for n in 1..(n_elves + 1) {
        elves.push(n);
    }
    let mut ofs = 0;
    while elves.len() > 1 {
        if (elves.len() % 100000) == 0 {
            println!("{} elves left", elves.len());
        }
        // Elf at ofs eliminates elf across from them which is elf at:
        let elim = ((elves.len() >> 1) + ofs) % elves.len();
        elves.remove(elim);
        if elim < ofs {
            ofs -= 1;
        }
        ofs = (ofs + 1) % elves.len();
    }
    elves[0]
}

fn main() {
    println!("Test: 5 elves, winner: {}", winner1(5));
    println!("Problem 1: {}", winner1(3018458));
    println!("Test: 5 elves, winner: {}", winner2(5));
    println!("Problem 2: {}", winner2(3018458));
}
