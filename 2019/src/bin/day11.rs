use aoc2019::*;

struct PaintingBot {
    prog: intcode::VM,
    posx: i32,
    posy: i32,
    panels: HashMap<(i32, i32), bool>,
    facing: Facing,
}

impl PaintingBot {
    fn new(vm: intcode::VM) -> Self {
        Self {
            prog: vm,
            posx: 0,
            posy: 0,
            panels: HashMap::new(),
            facing: Facing::Up,
        }
    }

    fn paint_position(&mut self, posx: i32, posy: i32, colour: bool) {
        *self.panels.entry((posx, posy)).or_insert(false) = colour;
    }

    fn run_to_completion(&mut self) -> Result<usize> {
        let mut pending_input = None;
        let mut awaiting_paint = true;
        loop {
            use intcode::VMState::*;
            let cursor = (self.posx, self.posy);
            let next_input = pending_input.take();
            match self.prog.interpreter_step(next_input)? {
                Runnable => {}
                WaitingOnInput => {
                    let panel_white = *self.panels.entry(cursor).or_insert(false);
                    let panel_num = if panel_white { 1 } else { 0 };
                    pending_input = Some(panel_num);
                }
                GaveOutput(o) => {
                    if awaiting_paint {
                        // Painting a panel
                        let entry = self.panels.entry(cursor).or_insert(false);
                        *entry = o == 1;
                        awaiting_paint = false;
                    } else {
                        // Movement
                        self.facing = if o == 0 {
                            self.facing.rotate_left()
                        } else {
                            self.facing.rotate_right()
                        };
                        let (newx, newy) = self.facing.move_by(cursor);
                        self.posx = newx;
                        self.posy = newy;
                        awaiting_paint = true;
                    }
                }
                Halted => break,
            }
        }
        Ok(self.panels.len())
    }

    fn display_map(&self) -> Vec<String> {
        let (minx, maxx, miny, maxy) = self.panels.keys().fold(
            (std::i32::MAX, std::i32::MIN, std::i32::MAX, std::i32::MIN),
            |(minx, maxx, miny, maxy), pos| {
                (
                    min(minx, pos.0),
                    max(maxx, pos.0),
                    min(miny, pos.1),
                    max(maxy, pos.1),
                )
            },
        );
        let mut ret = Vec::new();
        for y in miny..=maxy {
            let mut row = String::new();
            for x in minx..=maxx {
                if self.panels.get(&(x, y)).copied().unwrap_or(false) {
                    row.push('X');
                } else {
                    row.push(' ');
                }
            }
            ret.push(row);
        }
        ret
    }
}

fn part1(input: &intcode::VM) -> Result<usize> {
    let mut bot = PaintingBot::new(input.clone());
    bot.run_to_completion()
}

fn part2(input: &intcode::VM) -> Result<String> {
    let mut bot = PaintingBot::new(input.clone());
    bot.paint_position(0, 0, true);
    bot.run_to_completion()?;
    Ok(bot.display_map().join("\n"))
}

fn main() -> Result<()> {
    let input = read_input(11)?;
    let input = input.trim();
    let input = intcode::VM::from_str(input)?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2:\n{}", part2(&input)?);
    Ok(())
}
