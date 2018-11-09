use aoc2015::*;

fn part1(input: &Value) -> f64 {
    match input {
        Value::Number(n) => n.as_f64().unwrap(),
        Value::Array(ref v) => v.iter().map(part1).sum(),
        Value::Object(ref m) => m.values().map(part1).sum(),
        _ => 0.0,
    }
}

fn part2(red: &Value, input: &Value) -> f64 {
    match input {
        Value::Number(n) => n.as_f64().unwrap(),
        Value::Array(ref v) => v.iter().map(|v| part2(red, v)).sum(),
        Value::Object(ref m) => {
            if m.values().find(|&v| v == red).is_some() {
                0.0
            } else {
                m.values().map(|v| part2(red, v)).sum()
            }
        }
        _ => 0.0,
    }
}

fn main() -> Result<()> {
    let input = read_input(12)?;
    let input = serde_json::from_str(&input)?;
    println!("Part 1: {}", part1(&input));
    let red = Value::String("red".to_owned());
    println!("Part 2: {}", part2(&red, &input));
    Ok(())
}
