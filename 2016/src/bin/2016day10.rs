#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone, Copy)]
enum Goal {
    Unknown,
    Output(usize),
    Bot(usize),
}

#[derive(Debug)]
struct Bot {
    holding1: usize,
    holding2: usize,
    holdingc: usize,
    lowgoal: Goal,
    highgoal: Goal,
    lastcomp1: usize,
    lastcomp2: usize,
}

impl Default for Bot {
    fn default() -> Self {
        Bot::new()
    }
}

impl Bot {
    fn new() -> Bot {
        Bot {
            holding1: 0,
            holding2: 0,
            holdingc: 0,
            lowgoal: Goal::Unknown,
            highgoal: Goal::Unknown,
            lastcomp1: 0,
            lastcomp2: 0,
        }
    }

    fn give(&mut self, n: usize) {
        if self.holdingc == 2 {
            panic!("Unable to be given another value!");
        }
        if self.holdingc == 1 {
            if n > self.holding1 {
                self.holding2 = n;
            } else {
                self.holding2 = self.holding1;
                self.holding1 = n;
            }
            self.holdingc = 2;
        } else {
            self.holding1 = n;
            self.holdingc = 1;
        }
    }

    fn set_goals(&mut self, lowgoal: Goal, highgoal: Goal) {
        self.lowgoal = lowgoal;
        self.highgoal = highgoal;
    }
}

#[derive(Debug)]
struct RoomState {
    bots: HashMap<usize, Bot>,
    bins: HashMap<usize, usize>,
}

static NO_BOT: usize = 9999999;

impl RoomState {
    fn settle(&mut self) {
        /* While there are bots with two things held, run them */
        loop {
            let mut chosen: usize = NO_BOT;
            let mut lowgoal: Goal = Goal::Unknown;
            let mut highgoal: Goal = Goal::Unknown;
            let mut holding1: usize = 0;
            let mut holding2: usize = 0;
            for (botn, bot) in &self.bots {
                if bot.holdingc == 2 {
                    chosen = *botn;
                    lowgoal = bot.lowgoal;
                    highgoal = bot.highgoal;
                    holding1 = bot.holding1;
                    holding2 = bot.holding2;
                    break;
                }
            }
            if chosen == NO_BOT {
                break;
            }
            /* chosen bot is holding 2 items, can we do something? */
            match lowgoal {
                Goal::Output(bin) => {
                    let _prev: usize = *self.bins.get(&bin).unwrap();
                    self.bins.insert(bin, holding1);
                }
                Goal::Bot(target) => {
                    let targbot = self.bots.get_mut(&target).unwrap();
                    targbot.give(holding1);
                }
                _ => unreachable!(),
            }
            match highgoal {
                Goal::Output(bin) => {
                    let _prev: usize = *self.bins.get(&bin).unwrap();
                    self.bins.insert(bin, holding2);
                }
                Goal::Bot(target) => {
                    let targbot = self.bots.get_mut(&target).unwrap();
                    targbot.give(holding2);
                }
                _ => unreachable!(),
            }
            /* Tidy the bot */
            let mut bot = self.bots.get_mut(&chosen).unwrap();
            bot.lastcomp1 = bot.holding1;
            bot.lastcomp2 = bot.holding2;
            bot.holdingc = 0;
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Give(usize, usize),
    Prog(usize, Goal, Goal),
}

impl Instruction {
    fn new(l: &str) -> Instruction {
        lazy_static! {
            static ref GIVE_RE: Regex = Regex::new("value ([0-9]+) goes to bot ([0-9]+)").unwrap();
            static ref PROG_RE: Regex = Regex::new(
                "bot ([0-9]+) gives low to (bot|output) ([0-9]+) and high to (bot|output) ([0-9]+)"
            )
            .unwrap();
        }
        if let Some(cap) = GIVE_RE.captures_iter(l).next() {
            let val_ = cap.get(1).unwrap();
            let bot_ = cap.get(2).unwrap();
            let val: usize = val_.as_str().parse().unwrap();
            let bot: usize = bot_.as_str().parse().unwrap();
            Instruction::Give(val, bot)
        } else {
            let cap = PROG_RE.captures_iter(l).next().unwrap();
            let bot_ = cap.get(1).unwrap();
            let what1 = cap.get(2).unwrap();
            let n1_ = cap.get(3).unwrap();
            let what2 = cap.get(4).unwrap();
            let n2_ = cap.get(5).unwrap();
            let bot: usize = bot_.as_str().parse().unwrap();
            let n1: usize = n1_.as_str().parse().unwrap();
            let n2: usize = n2_.as_str().parse().unwrap();
            let goal1 = if what1.as_str().len() == 3 {
                Goal::Bot(n1)
            } else {
                Goal::Output(n1)
            };
            let goal2 = if what2.as_str().len() == 3 {
                Goal::Bot(n2)
            } else {
                Goal::Output(n2)
            };
            Instruction::Prog(bot, goal1, goal2)
        }
    }
}

fn load_state() -> RoomState {
    let f = File::open("day10.input").unwrap();
    let reader = BufReader::new(f);
    let mut bots: HashMap<usize, Bot> = HashMap::new();
    let mut bins: HashMap<usize, usize> = HashMap::new();
    let ensure_bot = |bots: &mut HashMap<usize, Bot>, n: usize| {
        bots.entry(n).or_default();
    };
    let ensure_bin = |bins: &mut HashMap<usize, usize>, n: usize| {
        bins.entry(n).or_insert(0);
    };
    for line_ in reader.lines() {
        let line = line_.unwrap();
        let instr = Instruction::new(&line);
        match instr {
            Instruction::Give(val, bot) => {
                ensure_bot(&mut bots, bot);
                let bot = bots.get_mut(&bot).unwrap();
                bot.give(val);
            }
            Instruction::Prog(bot, lowgoal, highgoal) => {
                ensure_bot(&mut bots, bot);
                match lowgoal {
                    Goal::Unknown => panic!("Unknown goal!"),
                    Goal::Bot(bot2) => ensure_bot(&mut bots, bot2),
                    Goal::Output(bin) => ensure_bin(&mut bins, bin),
                }
                match highgoal {
                    Goal::Unknown => panic!("Unknown goal!"),
                    Goal::Bot(bot2) => ensure_bot(&mut bots, bot2),
                    Goal::Output(bin) => ensure_bin(&mut bins, bin),
                }
                let bot = bots.get_mut(&bot).unwrap();
                bot.set_goals(lowgoal, highgoal);
            }
        }
    }

    RoomState { bots, bins }
}

fn problem1() -> usize {
    let mut room = load_state();
    room.settle();
    for (botn, bot) in &room.bots {
        if bot.lastcomp1 == 17 && bot.lastcomp2 == 61 {
            return *botn;
        }
    }
    NO_BOT
}

fn problem2() -> usize {
    let mut room = load_state();
    room.settle();
    let o0 = *room.bins.get(&(0 as usize)).unwrap();
    let o1 = *room.bins.get(&(1 as usize)).unwrap();
    let o2 = *room.bins.get(&(2 as usize)).unwrap();
    o0 * o1 * o2
}

fn main() {
    println!("Problem 1: {}", problem1());
    println!("Problem 2: {}", problem2());
}
