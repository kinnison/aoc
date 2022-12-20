use aoc2022::*;

struct Decryptor {
    values: Vec<i64>,
    indexes: Vec<usize>,
}

impl Decryptor {
    fn new(nums: &[i64], key: i64) -> Self {
        Self {
            values: nums.iter().copied().map(|n| n * key).collect(),
            indexes: (0..nums.len()).collect(),
        }
    }

    fn shunt(&mut self, idx: usize) {
        // To shunt a single value during a mix, we need to know what index it comes from
        // and to which index it needs to move
        let val = self.values[idx];
        if val == 0 {
            return;
        }
        let numidx = self.indexes[idx];
        self.values.remove(idx);
        self.indexes.remove(idx);

        let dest_index = {
            let n = ((idx as i64) + val).rem_euclid(self.values.len() as i64) as usize;
            if n == 0 {
                self.values.len()
            } else {
                n
            }
        };

        self.values.insert(dest_index, val);
        self.indexes.insert(dest_index, numidx);
    }

    fn mix(&mut self) {
        for pos in 0..self.values.len() {
            let loc = self
                .indexes
                .iter()
                .enumerate()
                .find(|(_, v)| **v == pos)
                .unwrap()
                .0;
            self.shunt(loc);
        }
    }

    fn key(&self, input_zeropos: usize) -> i64 {
        let zero_index = self
            .indexes
            .iter()
            .enumerate()
            .find(|(_, v)| **v == input_zeropos)
            .unwrap()
            .0;
        assert_eq!(self.values[zero_index], 0);
        let wrap = |n| (zero_index + n) % self.values.len();
        self.values[wrap(1000)] + self.values[wrap(2000)] + self.values[wrap(3000)]
    }
}

fn part1(input: &[i64]) -> i64 {
    let mut dec = Decryptor::new(input, 1);
    dec.mix();
    // Find zero in the inputs.
    let input_zeropos = input.iter().enumerate().find(|(_, v)| **v == 0).unwrap().0;
    dec.key(input_zeropos)
}

fn part2(input: &[i64]) -> i64 {
    let mut dec = Decryptor::new(input, 811589153);
    for _ in 0..10 {
        dec.mix();
    }
    // Find zero in the inputs.
    let input_zeropos = input.iter().enumerate().find(|(_, v)| **v == 0).unwrap().0;
    dec.key(input_zeropos)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
1
2
-3
3
-2
0
4"#;

    #[test]
    fn test_shunt() {
        let mut dec = Decryptor::new(&[1, 2, -3, 3, -2, 0, 4], 1);
        for (idx, target) in &[
            (0, [2, 1, -3, 3, -2, 0, 4]), // 1
            (0, [1, -3, 2, 3, -2, 0, 4]), // 2
            (1, [1, 2, 3, -2, -3, 0, 4]), // -3
            (2, [1, 2, -2, -3, 0, 3, 4]), // 3
            (2, [1, 2, -3, 0, 3, 4, -2]), // -2
            (3, [1, 2, -3, 0, 3, 4, -2]), // 0
            (5, [1, 2, -3, 4, 0, 3, -2]), // 4
        ] {
            println!("Shunting {} in {:?}", dec.values[*idx], dec.values);
            dec.shunt(*idx);
            assert_eq!(dec.values, target);
        }
    }

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn testcase2() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 1623178306);
    }
}

pub fn main() -> Result<()> {
    let input = read_input_as_vec(20)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
