use aoc2023::*;

pub fn main() -> Result<()> {
    let input = read_input(20)?;
    let input = Circuit::from_str(&input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(input: &Circuit) -> usize {
    let mut circuit = input.clone();
    let mut stash = HashMap::new();
    let mut pushes = 0;
    stash.insert(circuit.state(), pushes);
    let found_at = loop {
        pushes += 1;
        circuit.push_button();
        circuit.quiesce();
        match stash.entry(circuit.state()) {
            Entry::Occupied(o) => {
                break *o.get();
            }
            Entry::Vacant(v) => {
                v.insert(pushes);
            }
        }
        if pushes == 1000 {
            //println!("We didn't find a loop");
            return circuit.lows * circuit.highs;
        }
    };

    println!("After push {pushes} we found a repeated state from after push {found_at}");

    assert_eq!(found_at, 0);

    let mut low = circuit.lows;
    let mut high = circuit.highs;
    circuit.lows = 0;
    circuit.highs = 0;
    let more = 1000 % pushes;
    let mul = 1000 / pushes;
    low *= mul;
    high *= mul;

    println!("Running {more} more cycles");
    for _ in 0..more {
        circuit.push_button();
        circuit.quiesce();
    }

    low += circuit.lows;
    high += circuit.highs;

    low * high
}

fn part2(input: &Circuit) -> u64 {
    let mut circuit = input.clone();
    let rx = circuit.partnames["rx"];
    let inputs_to_rx = circuit
        .elements
        .iter()
        .enumerate()
        .filter_map(|(idx, ele)| {
            if ele.outputs.contains(&rx) {
                Some(idx)
            } else {
                None
            }
        })
        .collect_vec();
    //println!("RX is {rx} and elements which input to it are {inputs_to_rx:?}");
    assert_eq!(inputs_to_rx.len(), 1);
    let inputs_to_rx_input = circuit
        .elements
        .iter()
        .enumerate()
        .filter_map(|(idx, ele)| {
            if ele.outputs.contains(&inputs_to_rx[0]) {
                Some(idx)
            } else {
                None
            }
        })
        .collect_vec();
    //println!("Inputs to RX inputs are: {inputs_to_rx_input:?}");
    let mut vals = vec![];
    for to_find in inputs_to_rx_input {
        circuit = input.clone();
        for i in 1u64.. {
            circuit.push_button();
            if circuit.quiesce_rx(to_find, inputs_to_rx[0]) {
                // On this button push, this particular thing went high
                vals.push(i);
                break;
            }
        }
    }
    //println!("We found {vals:?}");
    vals.into_iter().fold(1, |acc, v| acc.lcm(v))
}

#[derive(Debug, Default, Clone)]
struct Circuit {
    partnames: HashMap<String, usize>,
    elements: Vec<Element>,
    queue: VecDeque<(usize, usize, Pulse)>,
    lows: usize,
    highs: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    #[default]
    Low,
}

impl Pulse {
    fn invert(&mut self) {
        if matches!(self, Pulse::High) {
            *self = Pulse::Low;
        } else {
            *self = Pulse::High;
        }
    }
}

impl Circuit {
    fn push_button(&mut self) {
        let bcast = self.partnames["broadcaster"];
        self.emit(bcast, bcast, Pulse::Low)
    }

    fn emit(&mut self, sender: usize, part: usize, pulse: Pulse) {
        match pulse {
            Pulse::High => self.highs += 1,
            Pulse::Low => self.lows += 1,
        }
        self.queue.push_back((sender, part, pulse));
    }

    fn quiesce(&mut self) {
        // Run queue until quiescence
        while let Some((sender, part, pulse)) = self.queue.pop_front() {
            //println!("{sender} pulses {part} {pulse:?}");
            for (target, pulse) in self.elements[part].receive(sender, pulse) {
                //println!("==> {part} queues {target} {pulse:?}");
                self.emit(part, target, pulse);
            }
        }
    }

    fn quiesce_rx(&mut self, from_watch: usize, to_watch: usize) -> bool {
        // Run queue until quiescence
        let mut saw_it = false;
        while let Some((sender, part, pulse)) = self.queue.pop_front() {
            //println!("{sender} pulses {part} {pulse:?}");
            if sender == from_watch && part == to_watch && pulse == Pulse::High {
                saw_it = true;
            }
            for (target, pulse) in self.elements[part].receive(sender, pulse) {
                //println!("==> {part} queues {target} {pulse:?}");
                self.emit(part, target, pulse);
            }
        }
        saw_it
    }

    fn state(&self) -> Vec<Pulse> {
        let mut ret = vec![];
        for ele in &self.elements {
            match &ele.kind {
                ElementKind::FlipFlop(v) => {
                    ret.push(*v);
                }
                ElementKind::Conjunction(n) => {
                    ret.extend(n.iter().copied().flatten());
                }
                _ => {}
            }
        }
        ret
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Element {
    kind: ElementKind,
    outputs: Vec<usize>,
}

impl Element {
    fn receive(&mut self, sender: usize, pulse: Pulse) -> impl IntoIterator<Item = (usize, Pulse)> {
        //println!("  ==> {sender} pulsing me {pulse:?}");
        let outpulse = match &mut self.kind {
            ElementKind::Broadcast => Some(pulse),
            ElementKind::FlipFlop(cur) => {
                //println!("    Note, I am a flipflop");
                if matches!(pulse, Pulse::High) {
                    None
                } else {
                    //println!("    I had {cur:?}");
                    cur.invert();
                    //println!("    So I am now {cur:?}");
                    Some(*cur)
                }
            }
            ElementKind::Conjunction(values) => {
                values[sender] = Some(pulse);
                //println!("    I am a conjunction.  My values are now: {values:?}");
                let total = values.iter().fold(Pulse::High, |acc, v| match (acc, v) {
                    (_, None) => acc,
                    (Pulse::Low, _) => Pulse::Low,
                    (Pulse::High, Some(v)) => *v,
                });
                //println!("       That makes {total:?}");
                match total {
                    Pulse::Low => Some(Pulse::High),
                    Pulse::High => Some(Pulse::Low),
                }
            }
        };
        if let Some(pulse) = outpulse {
            self.outputs
                .iter()
                .copied()
                .map(|i| (i, pulse))
                .collect_vec()
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
enum ElementKind {
    #[default]
    Broadcast,
    FlipFlop(Pulse),
    Conjunction(Vec<Option<Pulse>>),
}

impl FromStr for Circuit {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ret = Self::default();

        let mut outputs = vec![];
        for l in s.trim().lines().map(str::trim) {
            let (mut name, output) = l.split_once(" -> ").unwrap();
            outputs.push(output);
            let mut element = Element::default();
            match name.chars().next() {
                Some('%') => {
                    element.kind = ElementKind::FlipFlop(Pulse::Low);
                    name = name.strip_prefix('%').unwrap();
                }
                Some('&') => {
                    element.kind = ElementKind::Conjunction(vec![]);
                    name = name.strip_prefix('&').unwrap();
                }
                _ => {}
            }
            let idx = ret.partnames.len();
            ret.partnames.insert(name.to_string(), idx);
            ret.elements.push(element);
        }

        let dummy_idx = ret.partnames.len();
        ret.partnames.insert("**DUMMY**".to_string(), dummy_idx);
        ret.elements.push(Element::default());

        ret.partnames.insert("rx".to_string(), ret.partnames.len());
        ret.elements.push(Element::default());

        for elem in &mut ret.elements {
            if let ElementKind::Conjunction(inmap) = &mut elem.kind {
                inmap.resize(ret.partnames.len(), None);
            }
        }

        for (idx, outputs) in outputs.into_iter().enumerate() {
            ret.elements[idx].outputs = outputs
                .split(", ")
                .map(|s| {
                    //println!("Consider output `{s}`");
                    let oidx = ret.partnames.get(s).copied().unwrap_or(dummy_idx);
                    if let ElementKind::Conjunction(inmap) = &mut ret.elements[oidx].kind {
                        inmap[idx] = Some(Pulse::Low);
                    }
                    oidx
                })
                .collect();
        }

        Ok(ret)
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"#,
        11687500
    )]

    fn testcase(#[case] input: &str, #[case] goal: usize) {
        let input = Circuit::from_str(input).unwrap();
        println!("{input:?}");
        assert_eq!(part1(&input), goal);
    }
}
