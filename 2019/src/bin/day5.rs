use aoc2019::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cases_1() {
        let mut vm = intcode::VM::from_str("3,0,4,0,99").expect("Unable to parse");
        let mut output = Vec::new();
        vm.full_interpret(&[100], &mut output)
            .expect("Unable to run");
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 100);
    }

    #[test]
    fn test_cases_2() {
        let mut vm = intcode::VM::from_str("1002,4,3,4,33").expect("Unable to parse");
        vm.interpret().expect("Unable to run");
    }

    #[test]
    fn test_cases_3() {
        let mut vm = intcode::VM::from_str("1101,100,-1,4,0").expect("Unable to parse");
        vm.interpret().expect("Unable to run");
    }
    #[test]
    fn test_cases_4() {
        let base_vm = intcode::VM::from_str("3,9,8,9,10,9,4,9,99,-1,8").expect("Unable to parse");
        let mut vm = base_vm.clone();
        let input = [8];
        let mut output = Vec::new();
        vm.full_interpret(&input, &mut output)
            .expect("Unable to run");
        assert_eq!(output[0], 1);
        vm = base_vm.clone();
        let input = [7];
        output.drain(..);
        vm.full_interpret(&input, &mut output)
            .expect("Unable to run");
        assert_eq!(output[0], 0);
        let base_vm = intcode::VM::from_str("3,3,1108,-1,8,3,4,3,99").expect("Unable to parse");
        let mut vm = base_vm.clone();
        let input = [8];
        let mut output = Vec::new();
        vm.full_interpret(&input, &mut output)
            .expect("Unable to run");
        assert_eq!(output[0], 1);
        vm = base_vm.clone();
        let input = [7];
        output.drain(..);
        vm.full_interpret(&input, &mut output)
            .expect("Unable to run");
        assert_eq!(output[0], 0);
    }

    #[test]
    fn test_cases_5() {
        let base_vm = intcode::VM::from_str("3,9,7,9,10,9,4,9,99,-1,8").expect("Unable to parse");
        let mut vm = base_vm.clone();
        let input = [8];
        let mut output = Vec::new();
        vm.full_interpret(&input, &mut output)
            .expect("Unable to run");
        assert_eq!(output[0], 0);
        vm = base_vm.clone();
        let input = [7];
        output.drain(..);
        vm.full_interpret(&input, &mut output)
            .expect("Unable to run");
        assert_eq!(output[0], 1);
        let base_vm = intcode::VM::from_str("3,3,1107,-1,8,3,4,3,99").expect("Unable to parse");
        let mut vm = base_vm.clone();
        let input = [8];
        let mut output = Vec::new();
        vm.full_interpret(&input, &mut output)
            .expect("Unable to run");
        assert_eq!(output[0], 0);
        vm = base_vm.clone();
        let input = [7];
        output.drain(..);
        vm.full_interpret(&input, &mut output)
            .expect("Unable to run");
        assert_eq!(output[0], 1);
    }
}

fn part1(vm: &intcode::VM) -> Result<i64> {
    let mut vm = vm.clone();
    let input = [1];
    let mut output = Vec::new();
    vm.full_interpret(&input, &mut output)?;
    for i in 0..(output.len() - 1) {
        if output[i] != 0 {
            println!("Bad output: {} at {}", output[i], i);
        }
    }
    Ok(output[output.len() - 1])
}

fn part2(vm: &intcode::VM) -> Result<i64> {
    let mut vm = vm.clone();
    let input = [5];
    let mut output = Vec::new();
    vm.full_interpret(&input, &mut output)?;
    for i in 0..(output.len() - 1) {
        if output[i] != 0 {
            println!("Bad output: {} at {}", output[i], i);
        }
    }
    Ok(output[output.len() - 1])
}

fn main() -> Result<()> {
    let input = read_input(5)?;
    let input = input.trim();
    let test_vm = intcode::VM::from_str(input)?;
    println!("Part 1: {}", part1(&test_vm)?);
    println!("Part 2: {}", part2(&test_vm)?);
    Ok(())
}
