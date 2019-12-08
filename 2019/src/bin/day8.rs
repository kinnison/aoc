use aoc2019::*;

fn part1(input: &str) -> Result<usize> {
    // Image is 25x6 so layers are 150px each
    assert_eq!(input.len() % 150, 0);
    // Goal is on layer with fewest zeroes, count 1s and 2s, multiply counts
    let mut layers: Vec<_> = input
        .as_bytes()
        .chunks_exact(150)
        .map(|layer| {
            let zeroes = layer.iter().copied().filter(|b| *b == b'0').count();
            let ones = layer.iter().copied().filter(|b| *b == b'1').count();
            let twos = layer.iter().copied().filter(|b| *b == b'2').count();
            (zeroes, ones * twos)
        })
        .collect();
    assert!(!layers.is_empty());
    layers.sort();
    Ok(layers[0].1)
}

fn part2(input: &str) -> Result<String> {
    // Next goal is to composite the layers.
    // 2 is transparent, 0/1 are opaque
    // first layer is in front, subsequent layers are behind
    let front = input.as_bytes()[0..150].to_vec();
    let pixels = input
        .as_bytes()
        .chunks_exact(150)
        .skip(1)
        .fold(front, |pixels, layer| {
            pixels
                .iter()
                .copied()
                .zip(layer.iter().copied())
                .map(|(a, b)| if a == b'2' { b } else { a })
                .collect()
        });
    // Remember that things are 25x6, so now we split the pixels into
    // 6 rows and rerender them
    let rows: Vec<_> = pixels
        .chunks_exact(25)
        .map(|row| std::str::from_utf8(row).unwrap())
        .collect();
    Ok(rows.join("\n").replace('0', " ").replace('1', "X"))
}

fn main() -> Result<()> {
    let input = read_input(8)?;
    let input = input.trim();

    println!("Part 1: {}", part1(input)?);
    println!("Part 2:\n{}", part2(input)?);
    Ok(())
}
