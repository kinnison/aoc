use aoc2019::*;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn intcode_vm_checks() {
        static TESTS: &[(&str, usize, i64)] = &[
            (
                "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
                16,
                109,
            ),
            ("1102,34915192,34915192,7,4,7,99,0", 1, 1219070632396864),
            ("104,1125899906842624,99", 1, 1125899906842624),
        ];
        for (input, len, first) in TESTS.iter() {
            let mut vm = intcode::VM::from_str(input).expect("Could not parse program");
            let mut output = Vec::new();
            vm.full_interpret(&[], &mut output).expect("Could not run");
            assert_eq!(output.len(), *len);
            assert_eq!(output[0], *first);
        }
    }
}

fn run_step(input: &intcode::VM, part: i64) -> Result<i64> {
    let mut vm = input.clone();
    let mut output = Vec::new();
    vm.full_interpret(&[part], &mut output)?;
    assert_eq!(output.len(), 1);
    Ok(output[0])
}

fn main() -> Result<()> {
    let input = read_input(9)?;
    let input = input.trim();
    let input = intcode::VM::from_str(input)?;
    println!("Part 1: {}", run_step(&input, 1)?);
    println!("Part 2: {}", run_step(&input, 2)?);
    Ok(())
}
