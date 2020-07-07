fn curve_step(i: &str) -> String {
    let mut s: String = String::new();
    s.push_str(i);
    s.push('0');
    for ch in i.chars().rev() {
        s.push(if ch == '0' { '1' } else { '0' });
    }
    return s;
}

fn fill_disc(i: &str, tlen: usize) -> String {
    let mut s = i.to_string();
    while s.len() < tlen {
        s = curve_step(&s);
    }
    s.truncate(tlen);
    s
}

fn checksum(i: &str) -> String {
    let mut s = String::new();
    if (i.len() & 1) != 0 {
        return i.to_string();
    }
    let ref mut chs = i.chars();
    'lo: loop {
        if let Some(c1) = chs.next() {
            if let Some(c2) = chs.next() {
                s.push(if c1 == c2 { '1' } else { '0' });
            } else {
                panic!("Somehow we couldn't extract a second char from an even length string");
            }
        } else {
            break 'lo;
        }
    }
    if s.len() == 0 {
        panic!("No loop?");
    }
    if (s.len() & 1) == 0 {
        checksum(&s)
    } else {
        s
    }
}

fn main() {
    println!(
        "Problem 1 -> {}",
        checksum(&fill_disc("10111011111001111", 272))
    );
    println!(
        "Problem 1 -> {}",
        checksum(&fill_disc("10111011111001111", 35651584))
    );
}
