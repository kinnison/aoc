use aoc2015::*;

fn presents_at(house: usize) -> usize {
    (0..house)
        .map(|v| v + 1)
        .map(|elf| if (house % elf) == 0 { elf * 10 } else { 0 })
        .sum()
}

// Being smarter about this though...
/*

Each house receives presents from any elf which is a divisor of its index.

We can calculate divisors in two easy steps.  First from 1 to sqrt(housenum)
and then by dividing housenum by each of those (obv. skipping sqrt(housenum) if
present so we don't double-count it)

Given the divisors, in part 1 each house receives 10 * sum(divisors)

*/

fn divisors(n: usize) -> Vec<usize> {
    let sqrt: usize = (n as f64).sqrt() as usize;
    let mut smalls: Vec<usize> = (1..=(sqrt + 1)).filter(|d| (n % d) == 0).collect();
    let bigs: Vec<usize> = smalls
        .iter()
        .filter(|&d| *d != sqrt)
        .map(|d| n / d)
        .collect();

    smalls.extend(bigs);
    smalls
}

fn part1(input: usize) -> usize {
    for house in 1.. {
        let divs = divisors(house);
        let sum: usize = divs.iter().sum();
        if (sum * 10) >= input {
            return house;
        }
    }
    unreachable!()
}

/*

For part 2, we filter the divisors such that we only accept divisors which
need a multiple of 50 or less, and then we sum and multiply by 11

*/

fn part2(input: usize) -> usize {
    for house in 1.. {
        let divs = divisors(house).into_iter().filter(|d| house / d <= 50);
        let sum: usize = divs.sum();
        if (sum * 11) >= input {
            return house;
        }
    }
    unreachable!()
}

/* Sadly this does need something fast, like --release to be of use :( */

fn main() -> Result<()> {
    let input: usize = read_input(20)?.trim().parse().unwrap();
    for house in 1..=9 {
        println!("House {} got {} presents.", house, presents_at(house));
    }
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
    Ok(())
}
