use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use std::thread;

use std::sync::mpsc;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Isotope {
    Promethium,
    Cobalt,
    Curium,
    Ruthenium,
    Plutonium,
    Elerium,
    Dilithium,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Carry {
    Nothing,
    Generator(Isotope),
    Microchip(Isotope),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Floor {
    gens: HashSet<Isotope>,
    chips: HashSet<Isotope>,
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Floor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut carries = self.all_carries();
        carries.sort();
        carries.hash(state);
    }
}

impl Floor {
    fn new() -> Floor {
        Floor {
            gens: HashSet::new(),
            chips: HashSet::new(),
        }
    }

    fn empty(&self) -> bool {
        self.gens.is_empty() && self.chips.is_empty()
    }

    fn safe(&self) -> bool {
        // Safe if all chips have corresponding generator
        // or if there are no generators
        if self.gens.is_empty() {
            return true;
        }
        for chip in &self.chips {
            if !self.gens.contains(chip) {
                return false;
            }
        }
        true
    }

    fn leave_(&mut self, carry: Carry) {
        match carry {
            Carry::Generator(i) => {
                self.gens.remove(&i);
            }
            Carry::Microchip(i) => {
                self.chips.remove(&i);
            }
            Carry::Nothing => (),
        }
    }

    fn leave(&mut self, carry1: Carry, carry2: Carry) {
        self.leave_(carry1);
        self.leave_(carry2);
    }

    fn arrive_(&mut self, carry: Carry) {
        match carry {
            Carry::Generator(i) => {
                self.gens.insert(i);
            }
            Carry::Microchip(i) => {
                self.chips.insert(i);
            }
            Carry::Nothing => (),
        }
    }

    fn arrive(&mut self, carry1: Carry, carry2: Carry) {
        self.arrive_(carry1);
        self.arrive_(carry2);
    }

    fn all_carries(&self) -> Vec<Carry> {
        let mut ret: Vec<Carry> = Vec::new();
        for gen in &self.gens {
            ret.push(Carry::Generator(*gen));
        }
        for chip in &self.chips {
            ret.push(Carry::Microchip(*chip));
        }
        ret
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    up: bool,
    carry1: Carry,
    carry2: Carry,
}

#[allow(clippy::derive_hash_xor_eq)]
#[derive(Debug, Clone, Eq, Hash)]
struct RTGFacility {
    liftat: usize,
    floors: Vec<Floor>,
}

impl RTGFacility {
    fn new() -> RTGFacility {
        RTGFacility {
            liftat: 0,
            floors: vec![Floor::new(), Floor::new(), Floor::new(), Floor::new()],
        }
    }

    fn finished(&self) -> bool {
        // We're finished if everything is on the top floor
        for i in 0..self.floors.len() - 1 {
            if !self.floors[i].empty() {
                return false;
            }
        }
        true
    }

    fn possible_moves(&self) -> Vec<Move> {
        // Based on where we are, we generate all possible one or two carry
        // moves up and down, and return the facility for them
        let carries: Vec<Carry> = self.floors[self.liftat].all_carries();
        let mut moves: Vec<Move> = Vec::new();
        let dir = self.liftat == 0;
        // No carries, no moves...
        if carries.is_empty() {
            return moves;
        }
        // First up you can carry Nothing, and one of the possible carries
        for carry in &carries {
            moves.push(Move {
                up: dir,
                carry1: Carry::Nothing,
                carry2: *carry,
            });
        }
        // Second of all, you can carry two of the carries...
        if carries.len() > 1 {
            for i in 0..carries.len() - 1 {
                for j in i..carries.len() {
                    moves.push(Move {
                        up: dir,
                        carry1: carries[i],
                        carry2: carries[j],
                    });
                }
            }
        }
        // now if we're not at the bottom or top, duplicate with opposite moves
        if (self.liftat != 0) && (self.liftat != self.floors.len() - 1) {
            // duplicate moves...
            let n = moves.len();
            for i in 0..n {
                let mut m: Move = moves[i];
                m.up = !m.up;
                moves.push(m);
            }
        }
        moves
    }

    fn safe_moves(&self, moves: Vec<Move>) -> Vec<Move> {
        let mut ret: Vec<Move> = Vec::new();
        for m in &moves {
            let mut thisfloor: Floor = self.floors[self.liftat].clone();
            let nextfloor_: usize = if m.up {
                self.liftat + 1
            } else {
                self.liftat - 1
            };
            let mut nextfloor: Floor = self.floors[nextfloor_].clone();
            // A move is possible, *iff* leaving thisfloor is safe
            // and arriving nextfloor is safe
            thisfloor.leave(m.carry1, m.carry2);
            nextfloor.arrive(m.carry1, m.carry2);
            if thisfloor.safe() && nextfloor.safe() {
                ret.push(*m);
            }
        }
        ret
    }

    fn do_move(&mut self, m: Move) {
        let nextfloor_: usize = if m.up {
            self.liftat + 1
        } else {
            self.liftat - 1
        };
        self.floors[self.liftat].leave(m.carry1, m.carry2);
        self.floors[nextfloor_].arrive(m.carry1, m.carry2);
        self.liftat = nextfloor_;
    }

    fn next_state(&self, m: Move) -> RTGFacility {
        let mut ret: RTGFacility = self.clone();
        ret.do_move(m);
        ret
    }

    fn push_branches(&self, targ: &mut HashSet<RTGFacility>) {
        let moves = self.safe_moves(self.possible_moves());
        for m in &moves {
            let nextstate = self.next_state(*m);
            //println!("{:?} -> {:?} -> {:?}", self, m, nextstate);
            targ.insert(nextstate);
        }
    }

    fn push_branches_to(&self, targ: &mpsc::Sender<Option<RTGFacility>>) {
        let moves = self.safe_moves(self.possible_moves());
        for m in &moves {
            let nextstate = self.next_state(*m);
            targ.send(Some(nextstate)).unwrap();
        }
    }
}

impl PartialEq for RTGFacility {
    fn eq(&self, other: &RTGFacility) -> bool {
        (self.liftat == other.liftat)
            && (self.floors.len() == other.floors.len())
            && self
                .floors
                .iter()
                .zip(other.floors.iter())
                .map(|(a, b)| *a == *b)
                .all(|x| x)
    }
}

fn initial_state() -> RTGFacility {
    let mut ret = RTGFacility::new();
    // The first floor contains a promethium generator and a promethium-compatible microchip.
    ret.floors[0].arrive(
        Carry::Generator(Isotope::Promethium),
        Carry::Microchip(Isotope::Promethium),
    );
    // The second floor contains a cobalt generator, a curium generator, a ruthenium generator, and a plutonium generator.
    ret.floors[1].arrive_(Carry::Generator(Isotope::Cobalt));
    ret.floors[1].arrive_(Carry::Generator(Isotope::Curium));
    ret.floors[1].arrive_(Carry::Generator(Isotope::Ruthenium));
    ret.floors[1].arrive_(Carry::Generator(Isotope::Plutonium));
    // The third floor contains a cobalt-compatible microchip, a curium-compatible microchip, a ruthenium-compatible microchip, and a plutonium-compatible microchip.
    ret.floors[2].arrive_(Carry::Microchip(Isotope::Cobalt));
    ret.floors[2].arrive_(Carry::Microchip(Isotope::Curium));
    ret.floors[2].arrive_(Carry::Microchip(Isotope::Ruthenium));
    ret.floors[2].arrive_(Carry::Microchip(Isotope::Plutonium));
    // The fourth floor contains nothing relevant.
    ret
}

#[derive(Debug)]
struct Solver {
    seen: HashMap<RTGFacility, usize>,
    branches: HashSet<RTGFacility>,
}

static WORKER_COUNT: usize = 7;

impl Solver {
    fn new(fac: RTGFacility) -> Solver {
        let mut ret = Solver {
            seen: HashMap::new(),
            branches: HashSet::new(),
        };
        fac.push_branches(&mut ret.branches);
        ret.seen.insert(fac, 0);
        ret
    }

    fn step_branches(&mut self, depth: usize) {
        /*
        let mut branches : HashSet<RTGFacility> = HashSet::new();
        for branch in &self.branches {
            if ! self.seen.contains_key(branch) {
                branches.insert(branch.clone());
            }
        }
         */
        let mut branches = HashSet::new();
        println!(
            "At depth {}, {} branches to consider",
            depth,
            self.branches.len()
        );
        let (restx, resrx) = mpsc::channel();
        let mut threads = Vec::new();
        let mut worktxs = Vec::new();

        for _i in 0..WORKER_COUNT {
            let (worktx, workrx) = mpsc::channel::<Option<RTGFacility>>();
            worktxs.push(worktx);
            let restx: mpsc::Sender<Option<RTGFacility>> = restx.clone();
            threads.push(thread::spawn(move || {
                loop {
                    let msg = workrx.recv().unwrap();
                    match msg {
                        None => break,
                        Some(facility) => facility.push_branches_to(&restx),
                    }
                }
                //                println!("Thread finished!");
                restx.send(None).unwrap();
            }));
        }
        //        println!("Insert work...");
        let mut finished = 0;
        let mut worker = 0;
        for branch in self.branches.drain() {
            self.seen.insert(branch.clone(), depth);
            worktxs[worker].send(Some(branch)).unwrap();
            worker += 1;
            if worker == WORKER_COUNT {
                worker = 0;
            }
            let rc = resrx.try_recv();
            match rc {
                Err(_) => {}
                Ok(None) => finished += 1,
                Ok(Some(fac)) => {
                    if !self.seen.contains_key(&fac) {
                        branches.insert(fac);
                    }
                }
            }
        }
        //        println!("Insert ends...");
        worktxs.iter_mut().for_each(|w| w.send(None).unwrap());
        //        println!("Work sent, gather results...");
        while finished != WORKER_COUNT {
            let branch = resrx.recv().unwrap();
            match branch {
                None => {
                    finished += 1;
                    //                    println!("We have {} results, {} threads done", self.branches.len(), finished);
                }
                Some(fac) => {
                    if !self.seen.contains_key(&fac) {
                        branches.insert(fac);
                    }
                }
            }
        }
        //        println!("Join threads...");
        for worker in threads {
            worker.join().unwrap();
        }
        self.branches = branches;
    }

    fn finished(&self) -> bool {
        // We're finished if any of self.branches is finished
        self.branches.iter().map(RTGFacility::finished).any(|b| b)
    }

    fn solve(&mut self) -> usize {
        // Run the branch stepper until we have finished, and then
        // return the depth we reached
        let mut depth = 1;
        loop {
            if self.finished() {
                break;
            }
            assert!(!self.branches.is_empty());
            self.step_branches(depth);
            depth += 1;
        }
        depth
    }
}

fn problem1() -> usize {
    let mut solver = Solver::new(initial_state());
    solver.solve()
}

fn problem2() -> usize {
    let mut base = initial_state();
    base.floors[0].arrive(
        Carry::Generator(Isotope::Elerium),
        Carry::Microchip(Isotope::Elerium),
    );
    base.floors[0].arrive(
        Carry::Generator(Isotope::Dilithium),
        Carry::Microchip(Isotope::Dilithium),
    );
    let mut solver = Solver::new(base);
    solver.solve()
}

fn main() {
    println!("Problem 1: {}", problem1());
    println!("Problem 2: {}", problem2());
}
