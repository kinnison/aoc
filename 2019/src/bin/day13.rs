use aoc2019::*;

fn part1(input: &intcode::VM) -> Result<usize> {
    let mut screen: HashMap<(i64, i64), i64> = HashMap::new();
    let mut x = None;
    let mut y = None;
    let mut vm = input.clone();
    loop {
        use intcode::VMState::*;
        match vm.interpreter_step(None)? {
            Runnable => {}
            Halted => break,
            GaveOutput(v) => {
                if x.is_none() {
                    x = Some(v);
                } else if y.is_none() {
                    y = Some(v);
                } else {
                    let x = x.take().unwrap();
                    let y = y.take().unwrap();
                    screen.insert((x, y), v);
                }
            }
            WaitingOnInput => unimplemented!(),
        }
    }
    Ok(screen.values().copied().filter(|v| *v == 2).count())
}

fn part2(input: &intcode::VM) -> Result<i64> {
    let mut screen: HashMap<(i64, i64), i64> = HashMap::new();
    let mut x = None;
    let mut y = None;
    let mut ballx = 0;
    let mut padx = 0;
    let mut score = 0;
    let mut vm = input.clone();
    let mut next_input = None;
    vm.poke(0, 2)?;
    loop {
        use intcode::VMState::*;
        match vm.interpreter_step(next_input.take())? {
            Runnable => {}
            Halted => break,
            GaveOutput(v) => {
                if x.is_none() {
                    x = Some(v);
                } else if y.is_none() {
                    y = Some(v);
                } else {
                    let x = x.take().unwrap();
                    let y = y.take().unwrap();
                    if (x == -1) && (y == 0) {
                        score = v;
                    } else {
                        screen.insert((x, y), v);
                        if v == 3 {
                            padx = x;
                        } else if v == 4 {
                            ballx = x;
                        }
                    }
                }
            }
            WaitingOnInput => {
                let joystick = if padx == ballx {
                    0
                } else if padx < ballx {
                    1
                } else {
                    -1
                };
                next_input = Some(joystick);
            }
        }
    }
    Ok(score)
}

fn main() -> Result<()> {
    let input = read_input(13)?;
    let input = intcode::VM::from_str(&input)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}
