use aoc2020::*;

#[derive(Debug, ParseByRegex)]
#[regex = r"(?P<ingredients>[^\(]+)\(contains (?P<allergens>[^\)]+)\)"]
struct Food {
    ingredients: SpacedString,
    allergens: CommaSpacedString,
}

// The allergen map is a list of allergens and the sets of
// foods that allergen might be in
fn build_allergen_map(input: &[Food]) -> HashMap<String, HashSet<String>> {
    let mut ret: HashMap<String, HashSet<String>> = HashMap::new();
    for food in input {
        for allergen in food.allergens.iter() {
            let set = ret.entry(allergen.clone()).or_default();
            if set.is_empty() {
                for ingredient in food.ingredients.iter() {
                    set.insert(ingredient.clone());
                }
            } else {
                let other: HashSet<_> = food.ingredients.iter().cloned().collect();
                set.retain(|e| other.contains(e));
            }
        }
    }
    ret
}

fn part1(input: &[Food]) -> usize {
    let allergenmap = build_allergen_map(input);
    let mut total = 0;
    for food in input {
        for ingredient in food.ingredients.iter() {
            if !allergenmap
                .iter()
                .any(|(_, maybes)| maybes.contains(ingredient))
            {
                total += 1;
            }
        }
    }
    total
}

fn part2(input: &[Food]) -> String {
    let mut allergenmap = build_allergen_map(input);
    let mut dangerous = HashMap::new();
    let mut allergens = Vec::new();
    loop {
        let mut food = None;
        for (allergen, foods) in allergenmap.iter_mut() {
            if foods.is_empty() {
                continue;
            }
            if foods.len() == 1 {
                food = foods.drain().next().map(|f| (allergen.clone(), f));
                break;
            }
        }
        if let Some((allergen, food)) = food {
            for (_, foods) in allergenmap.iter_mut() {
                foods.remove(&food);
            }
            allergens.push(allergen.clone());
            dangerous.insert(allergen, food);
        } else {
            break;
        }
    }
    allergens.sort();
    allergens
        .iter()
        .map(|a| &dangerous[a])
        .map(String::as_str)
        .intersperse(",")
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn testcase2() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), "mxmxvkd,sqjhc,fvjkl");
    }
}

fn main() -> Result<()> {
    let input = read_input_as_vec(21)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
