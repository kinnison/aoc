use aoc2018::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum DamageKind {
    Fire,
    Cold,
    Slashing,
    Radiation,
    Bludgeoning,
}

use self::DamageKind::*;

impl DamageKind {
    fn parse(input: &str) -> Result<DamageKind> {
        Ok(match input {
            "fire" => Fire,
            "cold" => Cold,
            "slashing" => Slashing,
            "radiation" => Radiation,
            "bludgeoning" => Bludgeoning,
            _ => return Err(format!("Unable to parse damage kind: {}", input).into()),
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Side {
    Immune,
    Infect,
}

use self::Side::*;

#[derive(Clone, Debug)]
struct ArmyGroup {
    side: Side,
    units: usize,
    hp: usize,
    weaknesses: HashSet<DamageKind>,
    immunities: HashSet<DamageKind>,
    damage: usize,
    damagekind: DamageKind,
    initiative: usize,
}

impl ArmyGroup {
    fn effective_power(&self) -> usize {
        self.units * self.damage
    }

    fn target_phase_cmp(&self, b: &ArmyGroup) -> std::cmp::Ordering {
        let aep = self.effective_power();
        let bep = b.effective_power();
        if aep == bep {
            b.initiative.cmp(&self.initiative)
        } else {
            bep.cmp(&aep)
        }
    }

    fn calc_damage(&self, other: &ArmyGroup) -> usize {
        if other.immunities.contains(&self.damagekind) {
            0
        } else if other.weaknesses.contains(&self.damagekind) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }
}

#[allow(clippy::cognitive_complexity)]
fn run_fight(input: &[ArmyGroup], immuneboost: usize) -> Option<(Side, usize)> {
    let mut fighters: Vec<ArmyGroup> = input.to_vec();
    for f in fighters.iter_mut() {
        if f.side == Immune {
            f.damage += immuneboost;
        }
    }
    let mut initiative_idx: Vec<usize> = (0..fighters.len()).collect();
    initiative_idx.sort_by_key(|&n| fighters[n].initiative);
    initiative_idx.reverse();
    let mut damage_last_phase = 1;
    let winner = loop {
        if damage_last_phase == 0 {
            // We've probably hit a stalemate, give up
            if cfg!(debug_assertions) {
                dump_armies(&fighters);
            }
            return None;
        }
        if cfg!(debug_assertions) {
            println!("Immune System:");
            let mut ctr = 0;
            let mut prn = 0;
            for (i, f) in fighters.iter().enumerate() {
                if f.side == Immune {
                    ctr += 1;
                    if f.units > 0 {
                        println!("Group {} (at {}) contains {} units with {} effective power, {} initiative", ctr, i, f.units, f.effective_power(), f.initiative);
                        prn += 1;
                    }
                }
            }
            if prn == 0 {
                println!("No units remain")
            }
            println!("Infection:");
            let mut ctr = 0;
            let mut prn = 0;
            for (i, f) in fighters.iter().enumerate() {
                if f.side == Infect {
                    ctr += 1;
                    if f.units > 0 {
                        println!("Group {} (at {}) contains {} units with {} effective power, {} initiative", ctr, i, f.units, f.effective_power(), f.initiative);
                        prn += 1;
                    }
                }
            }
            if prn == 0 {
                println!("No units remain")
            }
        }
        if fighters
            .iter()
            .filter(|f| f.side == Immune)
            .map(|f| f.units)
            .sum::<usize>()
            == 0
        {
            break Infect; // No more immune system
        }
        if fighters
            .iter()
            .filter(|f| f.side == Infect)
            .map(|f| f.units)
            .sum::<usize>()
            == 0
        {
            break Immune; // No more infection
        }

        // Target selection phase
        let mut targeting_ord: Vec<usize> = (0..fighters.len()).collect();
        targeting_ord.sort_by(|&a, &b| fighters[a].target_phase_cmp(&fighters[b]));
        let mut targets: Vec<Option<usize>> = Vec::new();
        targets.resize(fighters.len(), None);
        let mut targetted: Vec<Option<usize>> = Vec::new();
        targetted.resize(fighters.len(), None);
        for &gid in &targeting_ord {
            if fighters[gid].units == 0 {
                // Group is dead, no action
                continue;
            }
            let mut possible_targets: Vec<usize> = (0..fighters.len())
                .filter(|&n| {
                    fighters[gid].side != fighters[n].side && // Opposing forces
                    fighters[n].units > 0 && // Not dead yet
                    targetted[n].is_none() && // Not targetted yet
                    fighters[gid].calc_damage(&fighters[n]) > 0 // Would damage at all
                })
                .collect();
            possible_targets.sort_by(|&a, &b| {
                let admg = fighters[gid].calc_damage(&fighters[a]);
                let bdmg = fighters[gid].calc_damage(&fighters[b]);
                if admg == bdmg {
                    fighters[b].target_phase_cmp(&fighters[a])
                } else {
                    admg.cmp(&bdmg)
                }
            });
            if cfg!(debug_assertions) {
                for &poss in possible_targets.iter() {
                    println!(
                        "{:?} group at {} would deal group at {} {} damage",
                        fighters[gid].side,
                        gid,
                        poss,
                        fighters[gid].calc_damage(&fighters[poss])
                    );
                }
            }
            let target = possible_targets.iter().last().cloned();
            targets[gid] = target;
            if let Some(target) = target {
                assert!(fighters[gid].calc_damage(&fighters[target]) > 0);
                targetted[target] = Some(gid);
                if cfg!(debug_assertions) {
                    println!(
                        "{:?} group at {} would deal group at {} {} damage  (CHOSEN)",
                        fighters[gid].side,
                        gid,
                        target,
                        fighters[gid].calc_damage(&fighters[target])
                    );
                }
            }
        }
        // Attack phase is carried out purely on initiative ordering
        damage_last_phase = 0;
        for &grp in initiative_idx.iter() {
            if fighters[grp].units == 0 {
                // Group already defeated, abadon shop
                if cfg!(debug_assertions) {
                    println!(
                        "{:?} group at {} does nothing, they are dead!",
                        fighters[grp].side, grp
                    );
                }
                continue;
            }
            if let Some(target) = targets[grp] {
                let dmg = fighters[grp].calc_damage(&fighters[target]);
                let killed: usize = min(dmg / fighters[target].hp, fighters[target].units);
                fighters[target].units -= killed;
                damage_last_phase += killed;
                if cfg!(debug_assertions) {
                    println!(
                        "{:?} group at {} attacks group at {} killing {} units of {} hp each by doing {} damage", 
                        fighters[grp].side, grp, target, killed, fighters[target].hp, dmg
                    );
                    if fighters[target].units == 0 {
                        println!("That group is defeated!");
                    }
                }
            }
        }
    };

    Some((winner, fighters.iter().map(|f| f.units).sum()))
}

fn parse_line(input: &str, side: Side) -> Result<ArmyGroup> {
    // Input of the form:
    // 1117 units each with 5042 hit points (weak to slashing; immune to fire, radiation, bludgeoning) with an attack that does 44 fire damage at initiative 15
    lazy_static! {
        static ref PARSE: Regex = Regex::new(r"^(\d+) units each with (\d+) hit points(?: \(([^\)]+)\))? with an attack that does (\d+) ([^ ]+) damage at initiative (\d+)$").unwrap();
    }

    let caps = PARSE
        .captures(input)
        .ok_or_else(|| format!("Unable to parse input line: {}", input))?;

    let mut ret = ArmyGroup {
        side,
        units: caps.get(1).ok_or("No units?")?.as_str().parse()?,
        hp: caps.get(2).ok_or("No hp?")?.as_str().parse()?,
        immunities: HashSet::new(),
        weaknesses: HashSet::new(),
        damage: caps.get(4).ok_or("No damage?")?.as_str().parse()?,
        damagekind: DamageKind::parse(caps.get(5).ok_or("No damage kind?")?.as_str())?,
        initiative: caps.get(6).ok_or("No initiative?")?.as_str().parse()?,
    };

    // Now we need to handle the weak/immune bits
    if let Some(modifiers) = caps.get(3).map(|v| v.as_str()) {
        // modifiers should be a string of the form:
        // weak to xxx, yyy; immune to zzz, bar
        for section in modifiers.split(';') {
            let section: String = section
                .trim()
                .chars()
                .filter(|&c| c != ';' && c != ',')
                .collect();
            let mut words = section.split_whitespace().fuse();
            let is_immunity = match words.next() {
                Some("immune") => true,
                Some("weak") => false,
                Some(eh) => return Err(format!("Unknown modifer kind: {}", eh).into()),
                None => return Err(format!("What?! {}", section).into()),
            };
            words.next(); // skip 'to'
            for w in words {
                if is_immunity {
                    ret.immunities.insert(DamageKind::parse(w)?);
                } else {
                    ret.weaknesses.insert(DamageKind::parse(w)?);
                }
            }
        }
    }
    Ok(ret)
}

fn parse_armies(input: &str) -> Result<Vec<ArmyGroup>> {
    let mut ret = Vec::new();
    let mut side = Immune;
    for l in input.trim().lines() {
        match l {
            "" => {}
            "Immune System:" => side = Immune,
            "Infection:" => side = Infect,
            _ => ret.push(parse_line(l, side)?),
        }
    }
    Ok(ret)
}

static TEST_INPUT: &str = r"
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
";

fn dump_armies(input: &[ArmyGroup]) {
    for grp in input.iter() {
        println!("{:?}: {} units, {} hp, weak to {:?}, immune to {:?}, attacks with {} {:?} damage, initiative {} ",
         grp.side, grp.units,grp.hp, grp.weaknesses, grp.immunities, grp.damage, grp.damagekind, grp.initiative);
    }
}

fn part2(input: &[ArmyGroup]) -> usize {
    let mut min_boost = 0;
    let mut max_boost = 1_000_000;
    let mut lastvalue = 0;
    while (max_boost - min_boost) > 1 {
        let boost = (min_boost + max_boost) / 2;
        if let Some((winner, value)) = run_fight(input, boost) {
            if winner == Infect {
                min_boost = boost;
            } else {
                max_boost = boost;
                lastvalue = value;
            }
        } else {
            // Stalemate, so bring up at least this far
            min_boost = boost;
        }
    }
    lastvalue
}

fn main() -> Result<()> {
    let test_input = parse_armies(TEST_INPUT)?;
    if cfg!(debug_assertions) {
        dump_armies(&test_input);
    }
    println!("Test 1: {:?}", run_fight(&test_input, 0));
    assert_eq!(run_fight(&test_input, 1570), Some((Immune, 51)));
    println!("Test 2: {:?}", part2(&test_input));
    let input = parse_armies(&read_input(24)?)?;
    if cfg!(debug_assertions) {
        dump_armies(&input);
    }
    println!("Part 1: {:?}", run_fight(&input, 0));
    println!("Part 2: {:?}", part2(&input));
    Ok(())
}
