use aoc2021::*;

#[derive(ParseByRegex, Debug)]
#[regex = r"^ *(?P<cave_a>[^-]+)-(?P<cave_b>.+)$"]
struct RawLink {
    cave_a: String,
    cave_b: String,
}

struct CaveSystem {
    links: HashMap<(bool, String), Vec<(bool, String)>>,
}

impl From<Vec<RawLink>> for CaveSystem {
    fn from(input: Vec<RawLink>) -> Self {
        let mut links: HashMap<(bool, String), Vec<(bool, String)>> = HashMap::new();
        for link in input {
            //println!("Raw Link: {:?}", link);
            let a_upper = (b'A'..=b'Z').contains(&link.cave_a.as_bytes()[0]);
            let b_upper = (b'A'..=b'Z').contains(&link.cave_b.as_bytes()[0]);
            links
                .entry((a_upper, link.cave_a.clone()))
                .or_default()
                .push((b_upper, link.cave_b.clone()));
            links
                .entry((b_upper, link.cave_b))
                .or_default()
                .push((a_upper, link.cave_a));
        }

        Self { links }
    }
}

#[allow(clippy::nonminimal_bool)]
fn try_node(
    input: &CaveSystem,
    all_paths: &mut Vec<Vec<String>>,
    cur_path: &mut Vec<String>,
    step_upper: bool,
    step: String,
    already_doubled: bool,
) {
    if step == "end" {
        cur_path.push(step);
        //println!("Path found: {:?}", cur_path);
        all_paths.push(cur_path.clone());
        cur_path.pop();
        return;
    }
    cur_path.push(step.clone());
    //println!("Current path: {:?} ({})", cur_path, already_doubled);
    for possible_node in input.links.get(&(step_upper, step)).unwrap() {
        if possible_node.1 == "start" {
            // Never repeat "start"
            continue;
        }
        let exists = cur_path.contains(&possible_node.1);
        if possible_node.0 || !exists || (exists && !already_doubled) {
            //println!("Trying: {:?}", possible_node);
            // possible is uppercase or not in path, so try walking to it
            try_node(
                input,
                all_paths,
                cur_path,
                possible_node.0,
                possible_node.1.clone(),
                already_doubled || (!possible_node.0 && exists),
            );
        } else {
            //println!("Skipping node: {:?}", possible_node);
        }
    }
    cur_path.pop();
}

fn find_all_paths(input: &CaveSystem, already_doubled: bool) -> Vec<Vec<String>> {
    let mut all_paths = Vec::new();
    let mut cur_path = vec![];
    try_node(
        input,
        &mut all_paths,
        &mut cur_path,
        false,
        "start".to_string(),
        already_doubled,
    );

    all_paths
}

fn part1(input: &CaveSystem) -> usize {
    // Find all unique paths through the cave system from `start` to `end` without
    // revisiting any lowercase caves.  We can do this fairly easily with recursion.
    find_all_paths(input, true).len()
}

fn part2(input: &CaveSystem) -> usize {
    find_all_paths(input, false).len()
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
