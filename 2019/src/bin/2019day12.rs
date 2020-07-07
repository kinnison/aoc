use aoc2019::*;

#[derive(ParseByRegex, Copy, Clone, Debug, Default, PartialEq, Hash, Eq)]
#[regex = r"<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>"]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct Moon {
    pos: Position,
    vel: Position,
}

impl Moon {
    fn new(pos: Position) -> Self {
        Self {
            pos,
            vel: Position::default(),
        }
    }

    fn adjust_velocites(&mut self, other: &mut Moon) {
        match self.pos.y.cmp(&other.pos.y) {
            Ordering::Less => {
                self.vel.y += 1;
                other.vel.y -= 1;
            }
            Ordering::Greater => {
                self.vel.y -= 1;
                other.vel.y += 1;
            }
            Ordering::Equal => {}
        }
        match self.pos.y.cmp(&other.pos.y) {
            Ordering::Less => {
                self.vel.y += 1;
                other.vel.y -= 1;
            }
            Ordering::Greater => {
                self.vel.y -= 1;
                other.vel.y += 1;
            }
            Ordering::Equal => {}
        }
        match self.pos.z.cmp(&other.pos.z) {
            Ordering::Less => {
                self.vel.z += 1;
                other.vel.z -= 1;
            }
            Ordering::Greater => {
                self.vel.z -= 1;
                other.vel.z += 1;
            }
            Ordering::Equal => {}
        }
    }

    fn apply_velocity(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }

    fn energy(&self) -> i32 {
        (self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs())
            * (self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs())
    }
}

fn part1(input: &[Moon]) -> i32 {
    let mut moons = input.to_owned();
    for _ in 0..1000 {
        for moon1 in 0..input.len() - 1 {
            for moon2 in moon1 + 1..input.len() {
                let (left, right) = moons.split_at_mut(moon2);
                left[moon1].adjust_velocites(&mut right[0]);
            }
        }
        moons.iter_mut().for_each(|moon| moon.apply_velocity());
    }
    moons.iter().map(|m| m.energy()).sum()
}

fn part2(input: &[Moon]) -> usize {
    let mut moons = input.to_owned();
    let mut steps = 0;
    let mut xcount = 0;
    let mut ycount = 0;
    let mut zcount = 0;
    loop {
        if xcount == 0
            && moons
                .iter()
                .zip(input.iter())
                .all(|(a, b)| a.pos.x == b.pos.x && a.vel.x == b.vel.x)
        {
            xcount = steps;
        }

        if ycount == 0
            && moons
                .iter()
                .zip(input.iter())
                .all(|(a, b)| a.pos.y == b.pos.y && a.vel.y == b.vel.y)
        {
            ycount = steps;
        }

        if zcount == 0
            && moons
                .iter()
                .zip(input.iter())
                .all(|(a, b)| a.pos.z == b.pos.z && a.vel.z == b.vel.z)
        {
            zcount = steps;
        }

        if (xcount != 0) && (ycount != 0) && (zcount != 0) {
            break;
        }

        steps += 1;
        for moon1 in 0..input.len() - 1 {
            for moon2 in moon1 + 1..input.len() {
                let (left, right) = moons.split_at_mut(moon2);
                left[moon1].adjust_velocites(&mut right[0]);
            }
        }
        moons.iter_mut().for_each(|moon| moon.apply_velocity());
    }
    // Each of x, y, and z repeated, so the lcm of all those repeats
    // should be the repeat for all three...
    lcm(lcm(xcount, ycount), zcount)
}

fn main() -> Result<()> {
    let input: Vec<Position> = read_input_as_vec(12)?;
    let input: Vec<Moon> = input.into_iter().map(Moon::new).collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}
