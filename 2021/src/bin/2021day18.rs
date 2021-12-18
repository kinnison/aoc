use std::io;

use aoc2021::*;

#[derive(Debug, Clone)]
enum SFValue {
    N(i32),
    P(Box<(SFValue, SFValue)>),
}

impl SFValue {
    fn value_from_chars<T>(chars: &mut Peekable<T>) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        if chars.peek() == Some(&b'[') {
            Self::pair_from_chars(chars)
        } else {
            let n = chars.next().unwrap();
            Ok(SFValue::N((n - b'0') as i32))
        }
    }

    fn pair_from_chars<T>(chars: &mut Peekable<T>) -> Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        assert_eq!(chars.next(), Some(b'['));
        let v1 = Self::value_from_chars(chars)?;
        assert_eq!(chars.next(), Some(b','));
        let v2 = Self::value_from_chars(chars)?;
        assert_eq!(chars.next(), Some(b']'));
        Ok(SFValue::P(Box::new((v1, v2))))
    }

    fn ival(&self) -> i32 {
        match self {
            SFValue::N(n) => *n,
            _ => unreachable!(),
        }
    }

    fn split(&mut self) -> bool {
        if matches!(self, SFValue::N(_)) {
            let val = self.ival();
            if val > 9 {
                //println!("Splitting {}", val);
                let lval = val >> 1;
                let rval = val - lval;
                *self = SFValue::P(Box::new((SFValue::N(lval), SFValue::N(rval))));
                true
            } else {
                false
            }
        } else {
            match self {
                SFValue::P(b) => {
                    if !b.0.split() {
                        b.1.split()
                    } else {
                        true
                    }
                }
                _ => false,
            }
        }
    }

    fn add_to_leftmost(&mut self, n: i32) {
        match self {
            SFValue::N(sn) => *sn += n,
            SFValue::P(b) => b.0.add_to_leftmost(n),
        }
    }

    fn add_to_rightmost(&mut self, n: i32) {
        match self {
            SFValue::N(sn) => *sn += n,
            SFValue::P(b) => b.1.add_to_rightmost(n),
        }
    }

    fn internal_explode(&mut self, depth: i32) -> Option<(SFValue, SFValue)> {
        // if is number, we don't explode
        if matches!(self, SFValue::N(_)) {
            return None;
        }
        // Is a pair
        if depth == 4 {
            // We explode
            println!("At depth == 4, explode {}", self);
            let old = std::mem::replace(self, SFValue::N(0));
            let (left, right) = match old {
                SFValue::P(b) => *b,
                _ => unreachable!(),
            };
            return Some((left, right));
        }
        match self {
            SFValue::P(b) => {
                // Depth less than 4, if our left explodes, we propagate rightward
                if let Some((left, right)) = b.0.internal_explode(depth + 1) {
                    // To prop rightward, add to leftmost on our right...
                    //println!("Leftmost exploded, adding {} to {}", right.ival(), b.1);
                    b.1.add_to_leftmost(right.ival());
                    //println!("Giving {}", b.1);
                    // and return with left propagation to do
                    return Some((left, SFValue::N(0)));
                }
                // else if our right explodes, we propagate leftward
                if let Some((left, right)) = b.1.internal_explode(depth + 1) {
                    // To prop leftward, add to rightmost on our left
                    //println!("Rightmost exploded, adding {} to {}", left.ival(), b.0);
                    b.0.add_to_rightmost(left.ival());
                    // and return with right propatation to do
                    //println!("Giving {}", b.0);
                    return Some((SFValue::N(0), right));
                }
                // neither exploded
                //println!("No explosion here");
                None
            }
            _ => unreachable!(),
        }
    }

    fn reduce(&mut self) {
        loop {
            //println!("Try to explode {}", self);
            if matches!(self.internal_explode(0), None) {
                // We didn't explode, can we split?
                //println!("Try to split {}", self);
                if !self.split() {
                    // Nope, we're reduced
                    //println!("Finished");
                    break;
                }
            }
        }
    }

    fn magnitude(&self) -> i32 {
        match self {
            SFValue::N(n) => *n,
            SFValue::P(b) => (3 * b.0.magnitude()) + (2 * b.1.magnitude()),
        }
    }

    fn make_pair(left: SFValue, right: SFValue) -> SFValue {
        SFValue::P(Box::new((left, right)))
    }

    fn add_to(self, right: SFValue) -> SFValue {
        let mut ret = Self::make_pair(self, right);
        ret.reduce();
        ret
    }
}

impl fmt::Display for SFValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SFValue::N(n) => write!(f, "{}", *n),
            SFValue::P(b) => write!(f, "[{},{}]", b.0, b.1),
        }
    }
}

impl FromStr for SFValue {
    type Err = io::Error;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self::pair_from_chars(&mut input.trim().bytes().peekable()).unwrap())
    }
}

fn part1(input: &[SFValue]) -> i32 {
    let summed = input
        .iter()
        .cloned()
        .fold1(|acc, val| {
            print!("{} + {} = ", acc, val);
            let ret = acc.add_to(val);
            println!("{}", ret);
            ret
        })
        .unwrap();
    summed.magnitude()
}

fn part2(input: &[SFValue]) -> i32 {
    (0..input.len())
        .flat_map(|left| {
            (0..input.len()).flat_map(move |right| {
                if left == right {
                    None
                } else {
                    let val = input[left].clone();
                    let mut val = val.add_to(input[right].clone());
                    val.reduce();
                    Some(val.magnitude())
                }
            })
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"#;

    #[test]
    fn testcase1() {
        let input: Vec<SFValue> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 4140);
    }

    #[test]
    fn testcase2() {
        let input: Vec<SFValue> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 3993);
    }

    #[test]
    fn reduce1() {
        let mut input = SFValue::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
        input.reduce();
        let output = format!("{}", input);
        assert_eq!(output, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    }

    #[test]
    fn reduce2() {
        let mut input = SFValue::from_str(
            "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
        )
        .unwrap();
        input.reduce();
        let output = format!("{}", input);
        assert_eq!(
            output,
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        )
    }
}

fn main() -> Result<()> {
    let input: Vec<SFValue> = read_input_as_vec(18)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
