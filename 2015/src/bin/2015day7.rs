use aoc2015::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Val {
    Fixed(u16),
    Wire(String),
}

#[derive(Debug, Clone)]
enum Gate {
    Assign { val: Val, wire: String },
    And { in1: Val, in2: Val, wire: String },
    Or { in1: Val, in2: Val, wire: String },
    LShift { in1: Val, in2: Val, wire: String },
    RShift { in1: Val, in2: Val, wire: String },
    Not { val: Val, wire: String },
}

type Wires = HashMap<String, u16>;

impl Val {
    fn from_str(input: &str) -> Val {
        if let Ok(value) = input.parse() {
            Val::Fixed(value)
        } else {
            Val::Wire(input.to_owned())
        }
    }

    fn value(&self, wires: &Wires) -> Option<u16> {
        match self {
            Val::Fixed(n) => Some(*n),
            Val::Wire(s) => {
                if let Some(v) = wires.get(s) {
                    Some(*v)
                } else {
                    None
                }
            }
        }
    }
}

impl Gate {
    fn from_str(input: &str) -> Gate {
        lazy_static! {
            static ref ASSIGN: Regex = Regex::new("^([^ ]+) -> ([^ ]+)$").unwrap();
            static ref NOT: Regex = Regex::new("^NOT ([^ ]+) -> ([^ ]+)$").unwrap();
            static ref AND: Regex = Regex::new("^([^ ]+) AND ([^ ]+) -> ([^ ]+)$").unwrap();
            static ref OR: Regex = Regex::new("^([^ ]+) OR ([^ ]+) -> ([^ ]+)$").unwrap();
            static ref LSHIFT: Regex = Regex::new("^([^ ]+) LSHIFT ([^ ]+) -> ([^ ]+)$").unwrap();
            static ref RSHIFT: Regex = Regex::new("^([^ ]+) RSHIFT ([^ ]+) -> ([^ ]+)$").unwrap();
        }
        if let Some(cap) = ASSIGN.captures(input) {
            let val = cap.get(1).unwrap().as_str();
            let wire = cap.get(2).unwrap().as_str();
            Gate::Assign {
                val: Val::from_str(val),
                wire: wire.to_owned(),
            }
        } else if let Some(cap) = NOT.captures(input) {
            let val = cap.get(1).unwrap().as_str();
            let wire = cap.get(2).unwrap().as_str();
            Gate::Not {
                val: Val::from_str(val),
                wire: wire.to_owned(),
            }
        } else if let Some(cap) = AND.captures(input) {
            let in1 = cap.get(1).unwrap().as_str();
            let in2 = cap.get(2).unwrap().as_str();
            let wire = cap.get(3).unwrap().as_str();
            Gate::And {
                in1: Val::from_str(in1),
                in2: Val::from_str(in2),
                wire: wire.to_owned(),
            }
        } else if let Some(cap) = OR.captures(input) {
            let in1 = cap.get(1).unwrap().as_str();
            let in2 = cap.get(2).unwrap().as_str();
            let wire = cap.get(3).unwrap().as_str();
            Gate::Or {
                in1: Val::from_str(in1),
                in2: Val::from_str(in2),
                wire: wire.to_owned(),
            }
        } else if let Some(cap) = LSHIFT.captures(input) {
            let in1 = cap.get(1).unwrap().as_str();
            let in2 = cap.get(2).unwrap().as_str();
            let wire = cap.get(3).unwrap().as_str();
            Gate::LShift {
                in1: Val::from_str(in1),
                in2: Val::from_str(in2),
                wire: wire.to_owned(),
            }
        } else if let Some(cap) = RSHIFT.captures(input) {
            let in1 = cap.get(1).unwrap().as_str();
            let in2 = cap.get(2).unwrap().as_str();
            let wire = cap.get(3).unwrap().as_str();
            Gate::RShift {
                in1: Val::from_str(in1),
                in2: Val::from_str(in2),
                wire: wire.to_owned(),
            }
        } else {
            panic!("Unable to parse gate: '{}'", input);
        }
    }

    fn apply(&self, wires: &mut Wires) -> bool {
        match self {
            Gate::Assign { val, wire } => {
                if let Some(val) = val.value(&wires) {
                    wires.insert(wire.clone(), val);
                    return true;
                }
            }
            Gate::Not { val, wire } => {
                if let Some(val) = val.value(&wires) {
                    wires.insert(wire.clone(), !val);
                    return true;
                }
            }
            Gate::And { in1, in2, wire } => {
                if let Some(in1) = in1.value(&wires) {
                    if let Some(in2) = in2.value(&wires) {
                        wires.insert(wire.clone(), in1 & in2);
                        return true;
                    }
                }
            }
            Gate::Or { in1, in2, wire } => {
                if let Some(in1) = in1.value(&wires) {
                    if let Some(in2) = in2.value(&wires) {
                        wires.insert(wire.clone(), in1 | in2);
                        return true;
                    }
                }
            }
            Gate::LShift { in1, in2, wire } => {
                if let Some(in1) = in1.value(&wires) {
                    if let Some(in2) = in2.value(&wires) {
                        wires.insert(wire.clone(), in1 << in2);
                        return true;
                    }
                }
            }
            Gate::RShift { in1, in2, wire } => {
                if let Some(in1) = in1.value(&wires) {
                    if let Some(in2) = in2.value(&wires) {
                        wires.insert(wire.clone(), in1 >> in2);
                        return true;
                    }
                }
            }
        };
        false
    }
}

fn part1(input: &[Gate]) -> u16 {
    let mut wires = HashMap::new();
    loop {
        let mut applied = true;
        for gate in input {
            let gate_ran = gate.apply(&mut wires);
            applied = applied && gate_ran;
        }
        if applied {
            break;
        }
    }
    wires["a"]
}

fn part2(input: &[Gate]) -> u16 {
    let val_a = part1(&input);
    let newinput: Vec<_> = input
        .iter()
        .map(|gate| match gate {
            Gate::Assign { val: _, wire } => {
                if wire == "b" {
                    Gate::Assign {
                        val: Val::Fixed(val_a),
                        wire: "b".to_owned(),
                    }
                } else {
                    gate.clone()
                }
            }
            _ => gate.clone(),
        })
        .collect();
    part1(&newinput)
}

fn main() -> Result<()> {
    let input: Vec<Gate> = read_input(7)?.lines().map(|s| Gate::from_str(s)).collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
