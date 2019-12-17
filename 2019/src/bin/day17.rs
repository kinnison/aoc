use aoc2019::*;

struct Scaffold {
    map: Vec<bool>,
    width: usize,
    height: usize,
    botpos: (i32, i32),
    botfacing: Facing,
}

#[derive(Debug)]
enum Instruction {
    TurnLeft,
    TurnRight,
    Go(usize),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;
        match self {
            TurnLeft => write!(f, "L"),
            TurnRight => write!(f, "R"),
            Go(n) => write!(f, "{}", n),
        }
    }
}

impl Scaffold {
    fn load(mut vm: intcode::VM) -> Result<Self> {
        let mut width = std::usize::MAX; // Start huge in case bot is on line 1
        let mut asciimap = Vec::new();
        let mut botpos = (0, 0);
        let mut botfacing = Facing::Up;

        vm.full_interpret(&[], &mut asciimap)?;

        let mut map = Vec::new();
        for ch in asciimap.into_iter().map(|v| (v as u8) as char) {
            match ch {
                '\n' => {
                    if width == std::usize::MAX {
                        width = map.len();
                    }
                }
                '.' => {
                    map.push(false);
                }
                '#' => {
                    map.push(true);
                }
                '^' | '<' | '>' | 'v' => {
                    botpos = ((map.len() % width) as i32, (map.len() / width) as i32);
                    map.push(true);
                    botfacing = match ch {
                        '^' => Facing::Up,
                        '<' => Facing::Left,
                        '>' => Facing::Right,
                        'v' => Facing::Down,
                        _ => unreachable!(),
                    };
                }
                _ => return Err(format!("Unknown map character: {}", ch).into()),
            }
        }

        let height = map.len() / width;

        Ok(Self {
            map,
            width,
            height,
            botpos,
            botfacing,
        })
    }

    fn is_scaffolding(&self, x: usize, y: usize) -> bool {
        self.map[x + (y * self.width)]
    }

    fn safe_to_go(&self, bp: (i32, i32), bf: Facing) -> bool {
        let nbp = bf.move_by(bp);
        if nbp.0 < 0 || nbp.1 < 0 || nbp.0 >= (self.width as i32) || nbp.1 >= (self.height as i32) {
            // Not safe, fell off an edge
            false
        } else {
            self.map[(nbp.0 + (nbp.1 * self.width as i32)) as usize]
        }
    }

    fn explore_fully(&self) -> Vec<Instruction> {
        let mut curpos = self.botpos;
        let mut curfacing = self.botfacing;
        // to_visit is the set of coordinates we've not visited
        // which we have to.  Our exploration algorithm is always
        // to go as far as possible before turning once.
        // We start at the end of one scaffold line, maybe pointing
        // at the line, maybe not.
        let mut res = Vec::new();
        loop {
            let mut dist = 0;
            while self.safe_to_go(curpos, curfacing) {
                dist += 1;
                curpos = curfacing.move_by(curpos);
            }
            if dist > 0 {
                res.push(Instruction::Go(dist));
            }
            // Attempt to turn left or right, if we can't do either then
            // we've reached the end and we stop
            if self.safe_to_go(curpos, curfacing.rotate_left()) {
                // We can turn left
                res.push(Instruction::TurnLeft);
                curfacing = curfacing.rotate_left();
            } else if self.safe_to_go(curpos, curfacing.rotate_right()) {
                // We can turn right
                res.push(Instruction::TurnRight);
                curfacing = curfacing.rotate_right();
            } else {
                // We cannot turn either way, we've reached the end
                break;
            }
        }
        res
    }
}

fn part1(input: &intcode::VM) -> Result<usize> {
    let scaff = Scaffold::load(input.clone())?;
    let mut alignments = 0;
    for x in 1..scaff.width - 1 {
        for y in 1..scaff.height - 1 {
            if scaff.is_scaffolding(x, y)
                && scaff.is_scaffolding(x - 1, y)
                && scaff.is_scaffolding(x + 1, y)
                && scaff.is_scaffolding(x, y - 1)
                && scaff.is_scaffolding(x, y + 1)
            {
                // Intersection found
                alignments += x * y;
            }
        }
    }

    Ok(alignments)
}

fn render_instruction_sequence(instrs: &[Instruction]) -> String {
    instrs.iter().map(|i| format!("{}", i)).join(",")
}

fn find_largest_subsequence(s: &str) -> impl Iterator<Item = String> {
    let mut subcounts: HashMap<String, usize> = HashMap::new();
    for sub in (8..21)
        .flat_map(|w| s.as_bytes().windows(w))
        .filter(|s| s[s.len() - 1] != b',' && s[0] != b',')
        .filter(|s| s.iter().all(|b| *b != b'A' && *b != b'B' && *b != b'C'))
        .map(|s| String::from_utf8(s.to_vec()).unwrap())
    {
        *subcounts.entry(sub).or_default() += 1;
    }
    let mut allsubs: Vec<String> = subcounts
        .into_iter()
        .filter_map(|(k, v)| if v > 1 { Some(k) } else { None })
        .collect();
    allsubs.sort_by_cached_key(|s| 400 - s.len());
    allsubs.into_iter()
}

fn find_workable_subset(prog: &str) -> (String, String, String, String) {
    for seq_a in find_largest_subsequence(prog) {
        let prog = prog.replace(&seq_a, "A");
        for seq_b in find_largest_subsequence(&prog) {
            let prog = prog.replace(&seq_b, "B");
            for seq_c in find_largest_subsequence(&prog) {
                let prog = prog.replace(&seq_c, "C");
                if prog.len() <= 20 {
                    // Found a candidate, go for it
                    return (prog, seq_a, seq_b, seq_c);
                }
            }
        }
    }
    unreachable!()
}

fn part2(input: &intcode::VM) -> Result<usize> {
    let scaff = Scaffold::load(input.clone())?;
    let instructions = scaff.explore_fully();
    let full_seq = render_instruction_sequence(&instructions);
    let (prog, seq_a, seq_b, seq_c) = find_workable_subset(&full_seq);
    //println!("Program: {}", full_seq);
    //println!("Turns into:");
    //println!("main: {}", prog);
    //println!("A: {}", seq_a);
    //println!("B: {}", seq_b);
    //println!("C: {}", seq_c);
    let mut newbot = input.clone();
    newbot.poke(0, 2)?;
    let botin: Vec<_> = format!("{}\n{}\n{}\n{}\nn\n", prog, seq_a, seq_b, seq_c)
        .bytes()
        .map(|b| b as i64)
        .collect();
    let mut botout = Vec::new();
    newbot.full_interpret(&botin, &mut botout)?;
    //println!("Output: {:?}", botout);

    Ok(botout[botout.len() - 1] as usize)
}

fn main() -> Result<()> {
    let input = read_input(17)?;
    let input = intcode::VM::from_str(&input)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);

    Ok(())
}
