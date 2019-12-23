use aoc2019::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum NodeAction {
    ReadPacket,
    WritePacket,
}

#[derive(Debug)]
struct NotWork {
    vms: Vec<intcode::VM>,
    last_actions: Vec<NodeAction>,
    queues: Vec<VecDeque<(i64, i64)>>,
    odd_packets: VecDeque<(usize, i64, i64)>,
}

use intcode::VMState::*;

impl NotWork {
    fn new(input: &intcode::VM) -> Self {
        let vms = (0..50).map(|_| input.clone()).collect();
        let queues = (0..50).map(|_| VecDeque::new()).collect();
        let last_actions = (0..50).map(|_| NodeAction::ReadPacket).collect();
        Self {
            vms,
            last_actions,
            queues,
            odd_packets: VecDeque::new(),
        }
    }

    /// Send boot signal to all the VMs
    fn boot(&mut self) -> Result<()> {
        for i in 0..self.vms.len() {
            self.last_actions[i] = match self.vms[i].interpreter_step(Some(i as i64))? {
                Runnable => unreachable!(),
                Halted => panic!("Machine halted immediately after boot?"),
                WaitingOnInput => NodeAction::ReadPacket,
                GaveOutput(nnum) => {
                    self.read_packet(i, nnum as usize)?;
                    NodeAction::WritePacket
                }
            }
        }
        Ok(())
    }

    fn read_packet(&mut self, mach: usize, dest: usize) -> Result<()> {
        // The spec says that a machine outputting a packet *will* output
        // the X and Y next cleanly
        if let GaveOutput(x) = self.vms[mach].interpreter_step(None)? {
            if let GaveOutput(y) = self.vms[mach].interpreter_step(None)? {
                // Received packet x,y for node dest
                if dest < self.vms.len() {
                    self.queues[dest].push_back((x, y));
                } else {
                    self.odd_packets.push_back((dest, x, y));
                }
                Ok(())
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    /// Run a single timeslice through the network
    /// Returns the Y value of any packet sent to node 255
    /// or None if this slice yielded no value
    fn run_slice(&mut self) -> Result<Option<(i64, i64)>> {
        for i in 0..self.vms.len() {
            let nodeaction = if self.last_actions[i] == NodeAction::ReadPacket {
                // This machine would like to read a packet
                let rest = if let Some((x, y)) = self.queues[i].pop_front() {
                    // Send this packet
                    if let WaitingOnInput = self.vms[i].interpreter_step(Some(x))? {
                        y
                    } else {
                        panic!("Machine {} read X but failed to ask for Y", i);
                    }
                } else {
                    // Provide no packet
                    -1
                };
                self.vms[i].interpreter_step(Some(rest))?
            } else {
                // This machine last wrote a packet
                self.vms[i].interpreter_step(None)?
            };
            self.last_actions[i] = match nodeaction {
                Runnable => unreachable!(),
                Halted => panic!("Machine {} halted at runtime?", i),
                WaitingOnInput => NodeAction::ReadPacket,
                GaveOutput(nnum) => {
                    self.read_packet(i, nnum as usize)?;
                    NodeAction::WritePacket
                }
            };
        }

        // If there's any odd packets, pop them off one by one and if one is
        // a packet for 255, yield the Y value

        while let Some((node, x, y)) = self.odd_packets.pop_front() {
            if node == 255 {
                return Ok(Some((x, y)));
            }
        }

        Ok(None)
    }

    fn run_with_nat(&mut self) -> Result<i64> {
        let mut nat_packet = (-1, -1);
        let mut last_packet = (0, 0);
        let mut all_reading = 0;
        loop {
            match self.run_slice()? {
                None => {
                    // The NAT needs to check for idle state?
                    if self
                        .last_actions
                        .iter()
                        .all(|a| *a == NodeAction::ReadPacket)
                    {
                        all_reading += 1;
                    } else {
                        all_reading = 0;
                    }
                    // We're idle if we've been all_reading for at least some
                    // number of cycles and all the queues are empty
                    if all_reading >= 10 && self.queues.iter().all(|q| q.is_empty()) {
                        // Idle, so enqueue the nat packet for node zero
                        // except if we did this before and the y value repeats
                        // the yield that instead
                        if nat_packet.1 == last_packet.1 {
                            break Ok(nat_packet.1);
                        }
                        self.queues[0].push_back(nat_packet);
                        last_packet = nat_packet;
                    }
                }
                Some(natval) => nat_packet = natval,
            }
        }
    }
}

fn part1(input: &intcode::VM) -> Result<i64> {
    let mut notwork = NotWork::new(input);
    notwork.boot()?;
    loop {
        if let Some((_x, y)) = notwork.run_slice()? {
            break Ok(y);
        }
    }
}

fn part2(input: &intcode::VM) -> Result<i64> {
    let mut notwork = NotWork::new(input);
    notwork.boot()?;
    notwork.run_with_nat()
}

fn main() -> Result<()> {
    let input = read_input(23)?;
    let input = intcode::VM::from_str(&input)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}
