use aoc2019::*;

#[cfg(test)]
mod test {
    use super::*;

    fn run_and_ret(prog: &str, peek_addr: i64) -> i64 {
        println!("Running test: {:?}", prog);
        let mut vm = intcode::VM::from_str(prog).expect("Unable to parse program");
        vm.interpret().expect("Unable to execute");
        vm.peek(peek_addr).expect("Unable to peek")
    }
    #[test]
    fn test_cases_1() {
        assert_eq!(run_and_ret("1,0,0,0,99", 0), 2);
        assert_eq!(run_and_ret("2,3,0,3,99", 3), 6);
        assert_eq!(run_and_ret("2,4,4,5,99,0", 5), 9801);
        assert_eq!(run_and_ret("1,1,1,4,99,5,6,0,99", 0), 30);
    }

    #[test]
    fn test_cases_2() {}
}

fn run_nv(code: &str, noun: i64, verb: i64) -> i64 {
    let mut vm = intcode::VM::from_str(code).expect("Unable to parse program");
    vm.poke(1, noun).expect("Unable to poke at 1");
    vm.poke(2, verb).expect("Unable to poke at 1");
    vm.interpret().expect("Unable to interpret");
    vm.peek(0).expect("Error peeking 0")
}

fn part1(code: &str) -> i64 {
    run_nv(code, 12, 2)
}

fn part2(code: &str) -> i64 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_nv(code, noun, verb) == 19_690_720 {
                return (100 * noun) + verb;
            }
        }
    }
    unreachable!()
}

fn main() -> Result<()> {
    let input = read_input(2)?;
    let input = input.trim();
    println!("Input: {:?}", input);
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
    Ok(())
}
