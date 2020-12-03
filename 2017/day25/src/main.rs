use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone)]
struct ActionPrint {
    write: bool,
    left: bool,
    cont: char,
}

#[derive(Clone)]
struct StatePrint {
    falsey: ActionPrint,
    truthy: ActionPrint,
}

impl StatePrint {
    fn new(
        write0: bool,
        move0: bool,
        cont0: char,
        write1: bool,
        move1: bool,
        cont1: char,
    ) -> StatePrint {
        StatePrint {
            falsey: ActionPrint {
                write: write0,
                left: move0,
                cont: cont0,
            },
            truthy: ActionPrint {
                write: write1,
                left: move1,
                cont: cont1,
            },
        }
    }
}

#[derive(Clone)]
struct BluePrint {
    start: char,
    sumafter: usize,
    states: HashMap<char, StatePrint>,
}

impl BluePrint {
    fn new(start: char, after: usize, states: &[(char, StatePrint)]) -> BluePrint {
        let mut ret = BluePrint {
            start,
            sumafter: after,
            states: HashMap::new(),
        };
        for &(ref ch, ref st) in states.iter() {
            ret.states.insert(*ch, st.clone());
        }
        ret
    }
}

fn test_print() -> BluePrint {
    let states = vec![
        ('A', StatePrint::new(true, false, 'B', false, true, 'B')),
        ('B', StatePrint::new(true, true, 'A', true, false, 'A')),
    ];
    BluePrint::new('A', 6, &states)
}

fn input_print() -> BluePrint {
    let states = vec![
        ('A', StatePrint::new(true, false, 'B', false, true, 'C')),
        ('B', StatePrint::new(true, true, 'A', true, true, 'D')),
        ('C', StatePrint::new(true, false, 'D', false, false, 'C')),
        ('D', StatePrint::new(false, true, 'B', false, false, 'E')),
        ('E', StatePrint::new(true, false, 'C', true, true, 'F')),
        ('F', StatePrint::new(true, true, 'E', true, false, 'A')),
    ];
    BluePrint::new('A', 12172063, &states)
}

struct Machine {
    print: BluePrint,
    state: char,
    pos: i64,
    tape: HashSet<i64>,
}

impl Machine {
    fn new(print: &BluePrint) -> Machine {
        Machine {
            print: print.clone(),
            state: print.start,
            pos: 0,
            tape: HashSet::new(),
        }
    }

    fn run(&mut self) {
        // We run the machine for print.sumafter loops...
        for _ in 0..self.print.sumafter {
            let state = self.print.states.get(&self.state).unwrap();
            let action = if self.tape.contains(&self.pos) {
                &state.truthy
            } else {
                &state.falsey
            };
            if action.write {
                self.tape.insert(self.pos);
            } else {
                self.tape.remove(&self.pos);
            }
            if action.left {
                self.pos -= 1;
            } else {
                self.pos += 1;
            }
            self.state = action.cont;
        }
    }

    fn checksum(&self) -> usize {
        self.tape.iter().len()
    }
}

fn problem1(input: &BluePrint) -> usize {
    let mut mach = Machine::new(input);
    mach.run();
    mach.checksum()
}

fn main() {
    let example = test_print();
    let input = input_print();
    println!("Problem 1 for example: {}", problem1(&example));
    println!("Problem 1 for input: {}", problem1(&input));
}
