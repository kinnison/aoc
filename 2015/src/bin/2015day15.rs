use aoc2015::*;

struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn from_str(input: &str) -> Ingredient {
        lazy_static! {
            static ref PARSE: Regex = Regex::new("^([^:]+): capacity (-?[0-9]+), durability (-?[0-9]+), flavor (-?[0-9]+), texture (-?[0-9]+), calories (-?[0-9]+)$").unwrap();
        }
        if let Some(cap) = PARSE.captures(input) {
            let name = cap.get(1).unwrap().as_str().to_owned();
            let capacity = cap.get(2).unwrap().as_str().parse().unwrap();
            let durability = cap.get(3).unwrap().as_str().parse().unwrap();
            let flavor = cap.get(4).unwrap().as_str().parse().unwrap();
            let texture = cap.get(5).unwrap().as_str().parse().unwrap();
            let calories = cap.get(6).unwrap().as_str().parse().unwrap();
            Ingredient {
                name,
                capacity,
                durability,
                flavor,
                texture,
                calories,
            }
        } else {
            panic!("Unable to parse ingredient: {}", input)
        }
    }

    fn dump(&self) {
        println!(
            "{}: capacity {}, durability {}, flavor {}, texture {}, calories {}",
            self.name, self.capacity, self.durability, self.flavor, self.texture, self.calories
        )
    }
}

struct Pantry {
    ingredients: Vec<Ingredient>,
}

impl Pantry {
    fn from_str(input: &str) -> Pantry {
        Pantry {
            ingredients: input.lines().map(Ingredient::from_str).collect(),
        }
    }

    fn score_cookie(&self, recipe: &[i32]) -> i32 {
        assert!(recipe.len() == self.ingredients.len());
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        #[allow(clippy::clippy::needless_range_loop)]
        for i in 0..recipe.len() {
            capacity += recipe[i] * self.ingredients[i].capacity;
            durability += recipe[i] * self.ingredients[i].durability;
            flavor += recipe[i] * self.ingredients[i].flavor;
            texture += recipe[i] * self.ingredients[i].texture;
        }
        if capacity < 0 {
            capacity = 0
        }
        if durability < 0 {
            durability = 0
        }
        if flavor < 0 {
            flavor = 0
        }
        if texture < 0 {
            texture = 0
        }
        capacity * durability * flavor * texture
    }

    fn calories(&self, recipe: &[i32]) -> i32 {
        assert!(recipe.len() == self.ingredients.len());
        recipe
            .iter()
            .copied()
            .zip(self.ingredients.iter().map(|i| i.calories))
            .map(|(r, c)| r * c)
            .sum()
    }

    fn dump(&self) {
        for ing in self.ingredients.iter() {
            ing.dump();
        }
    }
}

fn part1_(input: &Pantry, recipe: &mut Vec<i32>) -> i32 {
    if recipe.len() == input.ingredients.len() {
        let inputs: i32 = recipe.iter().sum();
        if inputs == 100 {
            input.score_cookie(recipe)
        } else {
            0
        }
    } else {
        let used: i32 = recipe.iter().sum();
        let remaining = 100 - used;
        let mut best = std::i32::MIN;
        for amt in 0..=remaining {
            recipe.push(amt);
            let score = part1_(input, recipe);
            recipe.pop();
            if score > best {
                best = score
            }
        }
        best
    }
}

fn part1(input: &Pantry) -> i32 {
    let mut recipe: Vec<i32> = Vec::new();
    part1_(input, &mut recipe)
}

fn part2_(input: &Pantry, recipe: &mut Vec<i32>) -> Option<i32> {
    if recipe.len() == input.ingredients.len() {
        let inputs: i32 = recipe.iter().sum();
        if inputs == 100 {
            if input.calories(recipe) == 500 {
                let score = input.score_cookie(recipe);
                Some(score)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        let used: i32 = recipe.iter().sum();
        let remaining = 100 - used;
        let mut best = None;
        for amt in 0..=remaining {
            recipe.push(amt);
            let score = part2_(input, recipe);
            recipe.pop();
            if score > best {
                best = score
            }
        }
        best
    }
}

fn part2(input: &Pantry) -> i32 {
    let mut recipe: Vec<i32> = Vec::new();
    part2_(input, &mut recipe).unwrap()
}

fn main() -> Result<()> {
    let test_input = Pantry::from_str("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3\n");
    println!("Test pantry:");
    test_input.dump();
    println!("Test 1: {}", part1(&test_input));
    println!("Test 2: {}", part2(&test_input));
    let input = Pantry::from_str(&read_input(15)?);
    println!("Pantry:");
    input.dump();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
