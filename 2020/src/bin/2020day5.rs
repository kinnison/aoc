use aoc2020::*;

fn seat_nr(seat: &str) -> u16 {
    // Seats are RRRRRRRCCC
    // where R is F/B and C is L/R
    // F is 0, B is 1
    // L is 0, R is 1
    let mut ret = 0;
    for bit in seat.chars() {
        ret <<= 1;
        ret |= match bit {
            'F' | 'L' => 0,
            'B' | 'R' => 1,
            _ => unimplemented!(),
        };
    }
    ret
}

fn part1(input: &[&str]) -> u16 {
    // highest seat ID
    input.iter().copied().map(seat_nr).max().unwrap()
}

fn part2(input: &[&str]) -> u16 {
    let mut seats: Vec<_> = input.iter().copied().map(seat_nr).collect();
    seats.sort_unstable();
    // Look for two almost consecutive values in the vec
    let mut myseat = 0;
    for (a, b) in seats.iter().copied().tuple_windows() {
        if b - a == 2 {
            // b and a have a gap between them, report it
            myseat = a + 1;
        }
    }
    myseat
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testcase1() {
        assert_eq!(seat_nr("FBFBBFFRLR"), 357);
        assert_eq!(seat_nr("BFFFBBFRRR"), 567);
        assert_eq!(seat_nr("FFFBBBFRRR"), 119);
        assert_eq!(seat_nr("BBFFBBFRLL"), 820);
    }

    #[test]
    fn testcase2() {}
}

fn main() -> Result<()> {
    let input: Vec<String> = read_input_as_vec(5)?;
    let input: Vec<_> = input.iter().map(|s| s.as_str()).collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
