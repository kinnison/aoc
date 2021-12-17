use aoc2021::*;

#[derive(ParseByRegex, Debug, Copy, Clone)]
#[regex = r"^target area: x=(?P<minx>-?\d+)\.\.(?P<maxx>-?\d+), y=(?P<miny>-?\d+)\.\.(?P<maxy>-?\d+)$"]
struct Conditions {
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
}

impl Conditions {
    fn xvels(&self) -> impl Iterator<Item = i32> {
        // We are looking for min..=max velocities which *could* land within the target area
        // since the distance a probe travels horizontally is a triangle number, we want the
        // min/max triangle numbers which land in the target area
        // This means we want the set of triangle numbers whose values are *within* the given range.
        let mut min_tri = 0;
        while triangle(min_tri) < self.minx {
            min_tri += 1;
        }
        let mut max_tri = min_tri;
        while triangle(max_tri + 1) <= self.maxx {
            max_tri += 1;
        }
        min_tri..=max_tri
    }

    fn max_height(&self, mut xvel: i32, mut yvel: i32) -> Option<i32> {
        // What is the max height reached, *if* this combo hits the target
        // We make the assumption that the xvel *will* result in a hit if the yvel is good
        let mut xpos = 0;
        let mut ypos = 0;
        let mut maxy = 0;
        loop {
            xpos += xvel;
            if xvel > 0 {
                xvel -= 1;
            }
            if xvel < 0 {
                xvel += 1;
            }
            ypos += yvel;
            maxy = max(maxy, ypos);
            yvel -= 1;
            // Check if we're in the area
            if (self.minx..=self.maxx).contains(&xpos) && (self.miny..=self.maxy).contains(&ypos) {
                break Some(maxy);
            }
            // Check if we've fallen below our target
            if ypos < self.miny && yvel < 1 {
                break None;
            }
        }
    }
}
fn part1(input: Conditions) -> i32 {
    // Seek the best y velocity which hits the target
    input
        .xvels()
        .flat_map(|xvel| {
            (input.miny..-input.miny).flat_map(move |yvel| input.max_height(xvel, yvel))
        })
        .max()
        .unwrap()
}
fn part2(input: Conditions) -> usize {
    // We're not after optimum heights this time, so we need to try any X velocity which might
    // eventually reach the range after any number of steps at all, so this time let's use
    // any xvel from 1 below min optimal (just reaches) to maxx (needs only 1 step)
    (input.xvels().next().unwrap() - 1..=input.maxx)
        .flat_map(|xvel| {
            (input.miny..-input.miny).flat_map(move |yvel| input.max_height(xvel, yvel))
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"target area: x=20..30, y=-10..-5"#;

    #[test]
    fn testcase1() {
        let input = Conditions::parse_by_regex(TEST_INPUT.trim()).unwrap();
        assert_eq!(input.max_height(7, 2), Some(3));
        assert_eq!(input.max_height(9, 0), Some(0));
        assert_eq!(input.max_height(6, 3), Some(6));
        assert_eq!(input.max_height(17, -4), None);
        assert_eq!(part1(input), 45);
    }

    #[test]
    fn testcase2() {
        let input = Conditions::parse_by_regex(TEST_INPUT.trim()).unwrap();
        assert_eq!(part2(input), 112);
    }
}

fn main() -> Result<()> {
    let input = read_input(17)?;
    let input = Conditions::parse_by_regex(input.trim())?;
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
    Ok(())
}
