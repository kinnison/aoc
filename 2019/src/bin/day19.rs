use aoc2019::*;

fn beam(
    input: &intcode::VM,
    minx: usize,
    miny: usize,
    maxx: usize,
    maxy: usize,
) -> Result<Vec<(usize, usize)>> {
    let r: StdResult<Vec<_>, _> = (minx..=maxx)
        .flat_map(move |x| {
            (miny..=maxy).filter_map(move |y| {
                let mut vm = input.clone();
                let mut out = Vec::new();
                match vm.full_interpret(&[x as i64, y as i64], &mut out) {
                    Ok(_) => {
                        if out[0] == 0 {
                            None
                        } else {
                            Some(Ok((x as usize, y as usize)))
                        }
                    }
                    Err(e) => Some(Err(e)),
                }
            })
        })
        .collect();
    Ok(r?)
}

fn part1(input: &intcode::VM) -> Result<usize> {
    Ok(beam(input, 0, 0, 49, 49)?.len())
}

fn probe(input: &intcode::VM, x: usize, y: usize) -> Result<bool> {
    let mut vm = input.clone();
    let input = [x as i64, y as i64];
    let mut output = Vec::new();
    vm.full_interpret(&input, &mut output)?;
    assert_eq!(output.len(), 1);
    Ok(output[0] == 1)
}

fn ship_fits(input: &intcode::VM, x: usize, y: usize) -> Result<bool> {
    // We return true if all four points of the ship are in the beam
    Ok(probe(input, x, y)?
        && probe(input, x + 99, y)?
        && probe(input, x, y + 99)?
        && probe(input, x + 99, y + 99)?)
}

fn part2(input: &intcode::VM) -> Result<usize> {
    // Our goal is to find the first spot where a 100x100 object would be
    // entirely caught in the beam we have.  For that to work, we need to
    // find a row which is wide enough that 100 rows down from it there would
    // be a row whose first entry matches the X coordinate we care about
    // and which top row has 100 columns to the right.

    // Let's do this by computing the formulae for the lines
    // Step one, the edges.  Compute that by skimming the 1000th row
    let mut minx = std::usize::MAX;
    let mut maxx = 0;
    for (x, _) in beam(input, 0, 3000, 3000, 3000)? {
        minx = min(minx, x);
        maxx = max(maxx, x);
    }
    assert_ne!(minx, std::usize::MAX);
    assert_ne!(maxx, 3000);

    #[cfg(debug_assertions)]
    println!(
        "On row 3000, the beam is {} wide, starting at {}",
        (maxx - minx) + 1,
        minx
    );

    // We know that the beam originates at 0,0
    // As such, we need only the two points we now have (minx,1000) and (maxx,1000)
    // to tell us the slope formulae of the line
    let min_slope = f64::from(minx as i32) / 3000f64;
    let max_slope = f64::from(maxx as i32) / 3000f64;

    let y = -99.0 * (1.0 + min_slope) / (min_slope - max_slope);
    let x = (max_slope * y) - 99.0;

    let mut base_x = (x.floor()) as usize;
    let mut base_y = (y.floor()) as usize;

    // Now that we've found somewhere the ship definitely fits, try shuffling it up/right
    'shuffle: loop {
        for x in (1..20).rev() {
            for y in (1..20).rev() {
                if ship_fits(input, base_x - x, base_y - y)? {
                    println!("Okay, we can shuffle santa by x-{} y-{}", x, y);
                    base_x -= x;
                    base_y -= y;
                    continue 'shuffle;
                }
            }
        }
        break;
    }

    #[cfg(debug_assertions)]
    {
        // Okay we think we know what we're up to, let's gather a big space and render it
        let points: HashSet<(usize, usize)> =
            beam(input, base_x - 25, base_y - 25, base_x + 125, base_y + 125)?
                .into_iter()
                .collect();

        for y in (base_y - 25)..=(base_y + 125) {
            let in_ship_y = y >= base_y && y <= base_y + 99;
            for x in (base_x - 25)..=(base_x + 125) {
                let in_ship = in_ship_y && x >= base_x && x <= base_x + 99;
                let in_beam = points.contains(&(x, y));
                if !(in_ship || in_beam) {
                    print!(" ");
                } else if !in_ship && in_beam {
                    print!(".");
                } else if in_ship && !in_beam {
                    print!("X");
                } else {
                    print!("-");
                }
            }
            println!();
        }
    }

    // We can be moderately confident, the puzzle calls for x*10_000 + y
    Ok(base_y + (base_x * 10_000))
}

fn main() -> Result<()> {
    let input = read_input(19)?;
    let input = intcode::VM::from_str(&input)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}
