use aoc2020::*;

#[derive(ParseByRegex, Clone, Debug)]
#[regex = r"(?P<name>[^:]+): (?P<min1>\d+)-(?P<max1>\d+) or (?P<min2>\d+)-(?P<max2>\d+)"]
struct Rule {
    name: String,
    min1: usize,
    max1: usize,
    min2: usize,
    max2: usize,
}

impl Rule {
    fn contains(&self, n: usize) -> bool {
        (n >= self.min1 && n <= self.max1) || (n >= self.min2 && n <= self.max2)
    }
}
#[derive(Debug, Clone)]
struct Puzzle {
    rules: Vec<Rule>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

impl FromStr for Puzzle {
    type Err = GenError;

    fn from_str(value: &str) -> StdResult<Self, Self::Err> {
        let mut lines = value.trim().lines().map(|s| s.trim()).fuse();
        let rules: Result<Vec<_>> = (&mut lines)
            .take_while(|s| !s.is_empty())
            .map(ParseByRegex::parse_by_regex)
            .collect();
        let rules = rules?;
        assert_eq!(lines.next(), Some("your ticket:"));
        let my_ticket = lines.next().ok_or("No ticket?")?;
        let my_ticket = input_by_split_pat(my_ticket, ",")?;
        assert_eq!(lines.next(), Some(""));
        assert_eq!(lines.next(), Some("nearby tickets:"));
        let nearby_tickets: Result<Vec<_>> = lines.map(|s| input_by_split_pat(s, ",")).collect();
        let nearby_tickets = nearby_tickets?;
        Ok(Self {
            rules,
            my_ticket,
            nearby_tickets,
        })
    }
}

impl Puzzle {
    fn sum_never_matches(&self, ticket: &[usize]) -> usize {
        ticket
            .iter()
            .copied()
            .filter(|n| !self.rules.iter().any(|r| r.contains(*n)))
            .sum()
    }
}

fn part1(input: &Puzzle) -> usize {
    // Ignoring our ticket, sum all the never-matches numbers
    input
        .nearby_tickets
        .iter()
        .map(|t| input.sum_never_matches(t))
        .sum()
}

fn part2(input: &Puzzle) -> usize {
    // find all tickets which are completely possible
    let possibles: Vec<_> = input
        .nearby_tickets
        .iter()
        .enumerate()
        .filter_map(|(n, t)| {
            if input.sum_never_matches(t) == 0 {
                Some(n)
            } else {
                None
            }
        })
        .collect();
    println!("Possible tickets are: {:?}", possibles);
    // For each ticket, we need to determine what rules might apply.
    let all_sets: Vec<Vec<HashSet<usize>>> = possibles
        .into_iter()
        .map(|n| &input.nearby_tickets[n])
        .map(|t| {
            // for the ticket t, return the set of rules which match each value
            t.iter()
                .copied()
                .map(|tval| {
                    input
                        .rules
                        .iter()
                        .enumerate()
                        .filter_map(|(n, r)| if r.contains(tval) { Some(n) } else { None })
                        .collect()
                })
                .collect()
        })
        .collect();
    println!("Possibilities are: {:?}", all_sets);

    // Next we need to transform our list of lists of sets into a list of sets
    // where each new set is the intersection of all equivalent sets.  We can do
    // that by means of folding the sets
    let mut all_sets: Vec<HashSet<usize>> = all_sets
        .into_iter()
        .fold1(|v1, v2| {
            v1.into_iter()
                .zip(v2.into_iter())
                .map(|(a, b)| a.intersection(&b).copied().collect())
                .collect()
        })
        .unwrap();

    // For safety, check no field has no possibilities
    for (n, field) in all_sets.iter().enumerate() {
        println!("Field {} has rule possibilities {:?}", n, field);
        assert!(!field.is_empty());
    }
    // Now we need to repeatedly assign rule indices to ticket positions by looking
    // for fields which can only be one possibility, assigning that, and removing it
    // from other fields
    let mut field_map = HashMap::new();
    while field_map.len() < input.rules.len() {
        // Check for any position in the all_sets which has exactly one element
        // in its set
        println!("So far, found {:?}", field_map);
        println!("Considering possibilities: {:?}", all_sets);
        if let Some(idx) = all_sets
            .iter()
            .enumerate()
            .filter_map(|(n, s)| if s.len() == 1 { Some(n) } else { None })
            .next()
        {
            let val = all_sets[idx].drain().next().unwrap();
            println!("I think position {} maps to rule {}", idx, val);
            field_map.insert(val, idx);
            for set in all_sets.iter_mut() {
                set.remove(&val);
            }
        }
    }
    println!("Final mapping: {:?}", field_map);
    // Now field_map maps rule number to field which it matches, all we need
    // is to find the rules which start 'departure '
    input
        .rules
        .iter()
        .enumerate()
        .filter_map(|(n, r)| {
            if r.name.starts_with("departure ") {
                Some(n)
            } else {
                None
            }
        })
        // Turn those rule indices into field indices
        .map(|i| field_map.get(&i).copied().unwrap())
        // Find those indices in our ticket
        .map(|i| input.my_ticket[i])
        // And multiply them together
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

    // Cheesed input slightly, labelling row departure row for part2 test
    const TEST_INPUT2: &str = r#"class: 0-1 or 4-19
departure row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"#;

    #[test]
    fn testcase1() {
        let input = Puzzle::from_str(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 71);
    }

    #[test]
    fn testcase2() {
        let input = Puzzle::from_str(TEST_INPUT2).unwrap();
        assert_eq!(part2(&input), 11);
    }
}

fn main() -> Result<()> {
    let input: String = read_input(16)?;
    let input = Puzzle::from_str(&input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
