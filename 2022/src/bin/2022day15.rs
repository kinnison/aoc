use aoc2022::*;

#[derive(ParseByRegex, Clone, Copy, Debug)]
#[regex = r"Sensor at x=(?P<x>-?\d+), y=(?P<y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)"]
struct Sensor {
    x: i32,
    y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

fn coalesce_ranges(mut ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    ranges.sort_unstable();
    let mut ret = Vec::new();
    ret.push(ranges[0]);

    for current in ranges.into_iter().skip(1) {
        let j = ret.len() - 1;
        if current.0 >= ret[j].0 && current.0 <= ret[j].1 {
            ret[j].1 = max(current.1, ret[j].1);
        } else {
            ret.push(current);
        }
    }

    ret
}

fn part1(input: &[Sensor], check_y: i32) -> usize {
    let mut vacant = Vec::new();

    for sensor in input {
        let sensor_manhat = sensor.x.abs_diff(sensor.beacon_x) + sensor.y.abs_diff(sensor.beacon_y);
        let y_dist = check_y.abs_diff(sensor.y);
        if sensor_manhat > y_dist {
            let width = sensor_manhat - y_dist;
            let start_x = sensor.x - width as i32;
            let end_x = sensor.x + width as i32;
            vacant.push((start_x, end_x));
            vacant = coalesce_ranges(vacant);
        }
    }

    // At this point, we have vacant ranges in the vacant vector, but we need
    // to count them up, and subtract any beacons on our line which are inside
    // any of the ranges
    let beacons_on_line: HashSet<_> = input
        .iter()
        .flat_map(|sensor| {
            if sensor.beacon_y == check_y {
                Some(sensor.beacon_x)
            } else {
                None
            }
        })
        .collect();
    //println!("Vacant ranges: {:?}", vacant);
    //println!("Beacons on the line: {:?}", beacons_on_line);
    let beacons_in_ranges = beacons_on_line
        .into_iter()
        .filter(|x| vacant.iter().any(|(s, e)| (s <= x) && (e >= x)))
        .count();
    //println!("Found {beacons_in_ranges} beacons overlapping");
    vacant
        .into_iter()
        .map(|(s, e)| (e - s + 1) as usize)
        .sum::<usize>()
        - beacons_in_ranges
}

fn part2(input: &[Sensor], range: i32) -> u64 {
    // To determine tuning, what we want to do is find for each row if there
    // is any gap at all... If the sensors cover the entire row, there cannot be
    // a missing beacon in that gap
    for y in 0..=range {
        let mut cover = Vec::new();
        for sensor in input {
            let sensor_manhat =
                sensor.x.abs_diff(sensor.beacon_x) + sensor.y.abs_diff(sensor.beacon_y);
            let y_dist = y.abs_diff(sensor.y);
            if sensor_manhat > y_dist {
                let width = sensor_manhat - y_dist;
                let start_x = max(0, sensor.x - width as i32);
                let end_x = min(range, sensor.x + width as i32);
                cover.push((start_x, end_x));
                cover = coalesce_ranges(cover);
            }
        }
        // cover is a set of ranges covering 0..=range
        // if it has more than one range in it, we have a gap
        if cover.len() > 1 {
            let missing_x = (cover[0].1 + 1) as u64;
            //println!("Found missing beacon at x={missing_x} y={y}");
            return (missing_x * 4_000_000) + (y as u64);
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

    #[test]
    fn testcase1() {
        let input: Vec<Sensor> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(dbg!(&input), 10), 26);
    }

    #[test]
    fn testcase2() {
        let input: Vec<Sensor> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input, 20), 56_000_011);
    }
}

pub fn main() -> Result<()> {
    let input: Vec<Sensor> = read_input_as_vec(15)?;
    println!("Part 1: {}", part1(&input, 2_000_000));
    println!("Part 2: {}", part2(&input, 4_000_000));
    Ok(())
}
