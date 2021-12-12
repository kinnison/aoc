use aoc2021::*;

#[derive(ParseByRegex, Debug)]
#[regex = r"^ *(?P<cave_a>[^-]+)-(?P<cave_b>.+)$"]
struct RawLink {
    cave_a: String,
    cave_b: String,
}

struct CaveSystem {
    links: HashMap<String, Vec<(bool, String)>>,
}

impl From<Vec<RawLink>> for CaveSystem {
    fn from(input: Vec<RawLink>) -> Self {
        let mut links: HashMap<String, Vec<(bool, String)>> = HashMap::new();
        for link in input {
            //println!("Raw Link: {:?}", link);
            let a_upper = (b'A'..=b'Z').contains(&link.cave_a.as_bytes()[0]);
            let b_upper = (b'A'..=b'Z').contains(&link.cave_b.as_bytes()[0]);
            links
                .entry(link.cave_a.clone())
                .or_default()
                .push((b_upper, link.cave_b.clone()));
            links
                .entry(link.cave_b)
                .or_default()
                .push((a_upper, link.cave_a));
        }

        Self { links }
    }
}

#[allow(clippy::nonminimal_bool)]
fn try_node<'cave>(
    input: &'cave CaveSystem,
    cur_path: &mut Vec<&'cave str>,
    step: &'cave str,
    already_doubled: bool,
) -> usize {
    if step == "end" {
        return 1;
    }
    let mut count = 0;
    cur_path.push(step);
    for possible_node in input.links.get(step).unwrap() {
        let next_step = possible_node.1.as_str();
        if next_step == "start" {
            // Never repeat "start"
            continue;
        }
        let exists = cur_path.contains(&next_step);
        if possible_node.0 || !exists || (exists && !already_doubled) {
            // possible is uppercase or not in path, so try walking to it
            count += try_node(
                input,
                cur_path,
                next_step,
                already_doubled || (!possible_node.0 && exists),
            );
        }
    }
    cur_path.pop();
    count
}

fn count_all_paths(input: &CaveSystem, already_doubled: bool) -> usize {
    let mut cur_path = vec![];
    try_node(input, &mut cur_path, "start", already_doubled)
}

fn part1(input: &CaveSystem) -> usize {
    // Find all unique paths through the cave system from `start` to `end` without
    // revisiting any lowercase caves.  We can do this fairly easily with recursion.
    count_all_paths(input, true)
}

fn part2(input: &CaveSystem) -> usize {
    count_all_paths(input, false)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUTS: &[(&str, usize, usize)] = &[
        (
            r#"start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end"#,
            10,
            36,
        ),
        (
            r#"dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc"#,
            19,
            103,
        ),
        (
            r#"fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW"#,
            226,
            3509,
        ),
    ];

    #[test]
    fn testcase1() {
        for (input, paths, _) in TEST_INPUTS.iter().cloned() {
            let input = input_as_vec(input).unwrap();
            let input = input.into();
            assert_eq!(part1(&input), paths);
        }
    }

    #[test]
    fn testcase2() {
        for (input, _, paths) in TEST_INPUTS.iter().cloned() {
            let input = input_as_vec(input).unwrap();
            let input = input.into();
            assert_eq!(part2(&input), paths);
        }
    }
}

fn main() -> Result<()> {
    let input: Vec<RawLink> = read_input_as_vec(12)?;
    let input = input.into();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
