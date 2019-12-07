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

fn run_sequence2(model: &intcode::VM, sequence: &[i64]) -> Result<i64> {
    let mut feedback = 0;
    let vms: Result<Vec<intcode::VM>> = sequence
        .iter()
        .map(|phase| {
            let mut vm = model.clone();
            let vmstate = vm.interpreter_step(None)?;
            assert_eq!(vmstate, intcode::VMState::WaitingOnInput);
            let vmstate = vm.interpreter_step(Some(*phase))?;
            assert_eq!(vmstate, intcode::VMState::WaitingOnInput);
            Ok(vm)
        })
        .collect();
    let mut vms = vms?;
    'outer: loop {
        for vm in vms.iter_mut() {
            let mut vmstate = vm.interpreter_step(None)?;
            'inner: loop {
                use intcode::VMState::*;
                match vmstate {
                    Runnable => panic!("Why did it return to us runnable?"),
                    Halted => break 'outer,
                    GaveOutput(v) => {
                        feedback = v;
                        break 'inner;
                    }
                    WaitingOnInput => vmstate = vm.interpreter_step(Some(feedback))?,
                }
            }
        }
    }
    Ok(feedback)
}

fn best_sequence<F>(model: &intcode::VM, start_seq: &[i64], score: F) -> Result<Vec<i64>>
where
    F: Fn(&intcode::VM, &[i64]) -> Result<i64>,
{
    let mut seq = start_seq.to_vec();
    let mut best_heap = seq.clone();
    let mut best_score = score(model, &best_heap)?;
    for seq in Heap::new(&mut seq) {
        let score = score(model, &seq)?;
        if score > best_score {
            best_heap = seq.clone();
            best_score = score;
        }
    }
    Ok(best_heap)
}

fn best_sequence1(model: &intcode::VM) -> Result<Vec<i64>> {
    best_sequence(model, &[0, 1, 2, 3, 4], run_sequence1)
}

fn best_sequence2(model: &intcode::VM) -> Result<Vec<i64>> {
    best_sequence(model, &[5, 6, 7, 8, 9], run_sequence2)
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

    #[test]
    fn test_case_2() {
        static CASES: &[(&str, &[i64], i64)] = &[(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
            &[9, 8, 7, 6, 5],
            139629729,
        ),
        (
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
            &[9, 7, 8, 5, 6],
            18216,
        )];
        for (model, sequence, target) in CASES.iter() {
            let model = intcode::VM::from_str(model).expect("Unable to parse model");
            assert_eq!(
                run_sequence2(&model, sequence).expect("Unable to run sequence"),
                *target
            );
            let best = best_sequence2(&model).expect("Unable to run model");
            assert_eq!(&best, sequence);
        }
    }
}

fn part1(model: &intcode::VM) -> Result<i64> {
    let seq = best_sequence1(model)?;
    run_sequence1(model, &seq)
}

fn part2(model: &intcode::VM) -> Result<i64> {
    let seq = best_sequence2(model)?;
    run_sequence2(model, &seq)
}

fn main() -> Result<()> {
    let input = read_input(7)?;
    let input = intcode::VM::from_str(&input)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}
