use aoc2018::*;

/* Critical insight here is that the regex is a regex, *NOT* a path
 * generator.  As such, we don't have to worry at all about the
 * alternations generating maximal path counts.
 *
 * Given that, we can simply walk the regex, generating an adjacency matrix
 * for the rooms, giving each room its distance from the start as its value.
 */
fn parse_input(input: &str) -> Result<HashMap<(i32, i32), usize>> {
    let mut distmap = HashMap::new();
    distmap.insert((0, 0), 0); // Distance from start point is zero
    let mut positions = Vec::new();
    let mut x = 0;
    let mut y = 0;
    if !input.starts_with('^') {
        return Err("Input regex didn't start with ^".into());
    }
    if !input.ends_with('$') {
        return Err("Input regex didn't end with $".into());
    }
    for ch in input[1..input.len() - 1].chars() {
        match ch {
            '(' => positions.push((x, y)),
            ')' => {
                let (x_, y_) = positions.pop().ok_or("Input regex closed too often")?;
                x = x_;
                y = y_;
            }
            '|' => {
                let (x_, y_) = positions[positions.len() - 1];
                x = x_;
                y = y_;
            }
            'N' | 'S' | 'E' | 'W' => {
                let newx = match ch {
                    'N' | 'S' => x,
                    'W' => x - 1,
                    'E' => x + 1,
                    _ => return Err("Wha?".into()),
                };
                let newy = match ch {
                    'N' => y - 1,
                    'S' => y + 1,
                    'E' | 'W' => y,
                    _ => return Err("Wha?".into()),
                };
                let curdist = distmap[&(x, y)];
                let newpos = distmap.entry((newx, newy)).or_insert(curdist + 1);
                *newpos = min(*newpos, curdist + 1);
                x = newx;
                y = newy;
            }
            _ => return Err(format!("Unknown input character: '{}'", ch).into()),
        }
    }
    Ok(distmap)
}

fn part1(input: &str) -> Result<usize> {
    let parsed = parse_input(input)?;
    Ok(parsed
        .values()
        .max()
        .cloned()
        .ok_or("Something went wrong!")?)
}

fn part2(input: &str) -> Result<usize> {
    let parsed = parse_input(input)?;
    Ok(parsed.values().filter(|&v| *v >= 1000).count())
}

static TEST_INPUT: &[(&str, usize)] = &[
    (r"^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$", 23),
    (
        r"^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
        31,
    ),
];

fn main() -> Result<()> {
    for test in TEST_INPUT {
        assert_eq!(test.1, part1(test.0)?);
    }
    let input = read_input(20)?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}
