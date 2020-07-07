fn gen_row(prev: &Vec<bool>) -> Vec<bool> {
    let mut ret: Vec<bool> = Vec::new();
    let l = prev.len();
    for n in 0..l {
        let left = if n == 0 { false } else { prev[n - 1] };
        let center = prev[n];
        let right = if n == (l - 1) { false } else { prev[n + 1] };
        ret.push(
            (left && center && !right)
                || (center && right && !left)
                || (left && !center && !right)
                || (right && !center && !left),
        );
    }
    ret
}

fn str_to_row(s: &str) -> Vec<bool> {
    let mut ret: Vec<bool> = Vec::new();
    for c in s.chars() {
        ret.push(c == '^');
    }
    ret
}

fn count_safe(input: &Vec<Vec<bool>>) -> usize {
    input.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |acc, tile| if *tile { acc } else { acc + 1 })
    })
}

fn puzzle1(input: &str, rowcount: usize) -> usize {
    let mut rows: Vec<Vec<bool>> = Vec::new();
    rows.push(str_to_row(input));
    while rows.len() < rowcount {
        let newrow: Vec<bool>;
        {
            let ref lastrow = &rows[rows.len() - 1];
            newrow = gen_row(lastrow);
        }
        rows.push(newrow);
    }
    count_safe(&rows)
}

static INPUT: &'static str = ".^^^.^.^^^.^.......^^.^^^^.^^^^..^^^^^.^.^^^..^^.^.^^..^.^..^^...^.^^.^^^...^^.^.^^^..^^^^.....^....";

fn main() {
    println!("Puzzle 1: {}", puzzle1(INPUT, 40));
    println!("Puzzle 2: {}", puzzle1(INPUT, 400000));
}
