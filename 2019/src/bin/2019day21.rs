use aoc2019::*;

static PROG1: &str = "OR A T\nAND B T\nAND C T\nNOT T J\nAND D J\nWALK\n";
static PROG2: &str = "NOT H J\nOR  C J\nAND B J\nAND A J\nNOT J J\nAND D J\nRUN\n";

fn part12(input: &intcode::VM, prog: &str) -> Result<i64> {
    let mut vm = input.clone();
    let mut output = Vec::new();
    let input: Vec<_> = prog.as_bytes().iter().map(|v| *v as i64).collect();
    vm.full_interpret(&input, &mut output)?;
    Ok(output[output.len() - 1])
}

fn main() -> Result<()> {
    let input = read_input(21)?;
    let input = intcode::VM::from_str(&input)?;

    if cfg!(feature = "interactive") {
        let mut vm = input;
        println!("Result: {}", vm.run_ascii_machine()?);
    } else {
        println!("Part 1: {}", part12(&input, PROG1)?);
        println!("Part 2: {}", part12(&input, PROG2)?);
    }
    Ok(())
}
