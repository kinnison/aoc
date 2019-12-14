use aoc2019::*;

#[derive(ParseByRegex, Debug, Clone)]
#[regex = r"(?P<count>\d+) (?P<chemical>[A-Z]+)"]
struct Reagent {
    chemical: String,
    count: usize,
}

#[derive(Debug, Clone)]
struct Reaction {
    inputs: Vec<Reagent>,
    output: Reagent,
}

#[derive(ParseByRegex, Debug)]
#[regex = r"(?P<inputs>[^=]+) => (?P<output>.+)"]
struct InputReaction {
    inputs: String,
    output: Reagent,
}

impl TryFrom<InputReaction> for Reaction {
    type Error = Box<dyn std::error::Error>;

    fn try_from(input: InputReaction) -> Result<Self> {
        let inputs: Vec<Reagent> = line_as_list(input.inputs)?;
        Ok(Self {
            inputs,
            output: input.output,
        })
    }
}

type ChemicalSoup = HashMap<String, usize>;

struct SoupyReaction {
    output: String,
    count: usize,
    inputs: ChemicalSoup,
}

impl From<Reaction> for SoupyReaction {
    fn from(input: Reaction) -> Self {
        let output = input.output.chemical;
        let count = input.output.count;
        let inputs = input
            .inputs
            .into_iter()
            .map(|reagent| (reagent.chemical, reagent.count))
            .collect();
        Self {
            output,
            count,
            inputs,
        }
    }
}

type ReactionPool = HashMap<String, SoupyReaction>;

fn load_data(s: &str) -> Result<ReactionPool> {
    let inputs: Vec<InputReaction> = input_as_vec(s)?;
    let input_count = inputs.len();
    let reactions: Result<Vec<Reaction>> = inputs.into_iter().map(TryFrom::try_from).collect();
    let ret: ReactionPool = reactions?
        .into_iter()
        .map(SoupyReaction::from)
        .map(|r| (r.output.clone(), r))
        .collect();
    assert_eq!(ret.len(), input_count);
    Ok(ret)
}

fn run_reactor(reactions: &ReactionPool, fuel: usize) -> Result<usize> {
    let mut factory: ChemicalSoup = HashMap::new();
    let mut spares: ChemicalSoup = HashMap::new();

    // Our goal is to make one fuel, so that's the factory's target
    factory.insert("FUEL".to_owned(), fuel);
    // We keep working backwards producing something the factory needs
    // until we've run back to ORE only
    while let Some(target) = factory
        .iter()
        .filter_map(|(k, v)| {
            if k.as_str() != "ORE" && *v > 0 {
                Some(k)
            } else {
                None
            }
        })
        .next()
        .cloned()
    {
        let wanted = factory[&target];
        let reaction = &reactions[&target];
        // To make the target, we need some number of the reaction applied
        //println!("Applying reaction to make {} of {}", wanted, target);
        // How many of the reaction do we need?
        let mut multiplier = (wanted + reaction.count - 1) / reaction.count;
        while (reaction.count * multiplier) < wanted {
            multiplier += 1;
        }
        //println!("multiplier: {}", multiplier);
        let spare = (reaction.count * multiplier) - wanted;
        //println!(
        //    "Need to run {} reactions each making {}, leaving {} spare",
        //    multiplier, reaction.count, spare,
        //);
        // First, remove the goal from the factory and add spares to the
        // spares holder
        factory.remove(&target);
        *spares.entry(target.clone()).or_default() += spare;
        // Now add multiplied up reagent set to the factory
        for (reagent, eachcount) in reaction.inputs.iter() {
            //println!(
            //    "Adding a need for {} of {}",
            //    eachcount * multiplier,
            //    reagent
            //);
            *factory.entry(reagent.clone()).or_default() += eachcount * multiplier;
        }
        // Next, reduce spares onto the factory
        for (chemical, count) in spares.iter_mut() {
            if *count > 0 {
                let wanted = factory.get(chemical).copied().unwrap_or_default();
                if wanted > 0 {
                    let xfer = min(*count, wanted);
                    //println!("Transferring {} of {} from the spares", wanted, chemical);
                    *count -= xfer;
                    *factory.entry(chemical.clone()).or_default() -= xfer;
                }
            }
        }
    }

    // We're done, so there's a required ORE count
    Ok(factory["ORE"])
}

fn part1(input: &ReactionPool) -> Result<usize> {
    run_reactor(input, 1)
}

static TRILLION: usize = 1_000_000_000_000;

fn part2(input: &ReactionPool) -> Result<usize> {
    let cost1 = run_reactor(input, 1)?;
    // Ignoring leftovers, we can definitely create:
    let mut fuel_count = TRILLION / cost1;
    let mut addend = 1000000;
    loop {
        let ore = run_reactor(input, fuel_count + addend)?;
        if ore >= TRILLION {
            if addend > 1 {
                addend = addend / 10;
                continue;
            } else {
                break;
            }
        }
        //let remaining = TRILLION - ore;
        //println!("Maybe coult add {} goes?", remaining / cost1);
        fuel_count += addend;
    }
    Ok(fuel_count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        static INPUT: &str = r"
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        let input = load_data(INPUT).expect("Unable to parse input reactions");
        assert_eq!(part1(&input).expect("Factory exploded?"), 31);
    }

    #[test]
    fn test_2() {
        static INPUT: &str = r"
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let input = load_data(INPUT).expect("Unable to parse input reactions");
        assert_eq!(part1(&input).expect("Factory exploded?"), 165);
    }

    #[test]
    fn test_3() {
        static INPUT: &str = r"
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        let input = load_data(INPUT).expect("Unable to parse input reactions");
        assert_eq!(part1(&input).expect("Factory exploded?"), 2210736);
        assert_eq!(part2(&input).expect("Factory exploded?"), 460664);
    }
}

fn main() -> Result<()> {
    let input = read_input(14)?;
    let input = load_data(&input)?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}
