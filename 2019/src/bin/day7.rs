use aoc2019::*;

fn run_amplifier(model: &intcode::VM, phase: i64, input: i64) -> Result<i64> {
    let mut vm = model.clone();
    let input = [phase, input];
    let mut output = Vec::new();
    vm.full_interpret(&input, &mut output)?;
    if output.len() != 1 {
        Err(format!("Output was {:?}", output).into())
    } else {
        Ok(output[0])
    }
}

fn run_sequence1(model: &intcode::VM, sequence: &[i64]) -> Result<i64> {
    let mut signal = 0;
    for phase in sequence.iter().copied() {
        signal = run_amplifier(model, phase, signal)?;
    }
    Ok(signal)
}

fn best_sequence1(model: &intcode::VM) -> Result<Vec<i64>> {
    let mut seq = vec![0, 1, 2, 3, 4];
    let mut best_heap = seq.clone();
    let mut best_score = run_sequence1(model, &best_heap)?;
    for seq in Heap::new(&mut seq) {
        let score = run_sequence1(model, &seq)?;
        if score > best_score {
            best_heap = seq.clone();
            best_score = score;
        }
    }
    Ok(best_heap)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_case_1() {
        static CASES: &[(&str, &[i64], i64)] = &[
            (
                "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
                &[4, 3, 2, 1, 0],
                43210,
            ),
            (
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
                &[0, 1, 2, 3, 4],
                54321,
            ),
            (
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
                &[1, 0, 4, 3, 2],
                65210),
        ];
        for (model, sequence, target) in CASES.iter() {
            let model = intcode::VM::from_str(model).expect("Unable to parse model");
            assert_eq!(
                run_sequence1(&model, sequence).expect("Unable to run sequence"),
                *target
            );
            let best = best_sequence1(&model).expect("Unable to run model");
            assert_eq!(&best, sequence);
        }
    }
}

fn part1(model: &intcode::VM) -> Result<i64> {
    let seq = best_sequence1(model)?;
    run_sequence1(model, &seq)
}

fn main() -> Result<()> {
    let input = read_input(7)?;
    let input = intcode::VM::from_str(&input)?;

    println!("Part 1: {}", part1(&input)?);
    Ok(())
}
