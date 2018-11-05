use aoc2015::*;
use std::io::Result;

fn part1(input: &Vec<String>) -> usize {
    fn is_nice(input: &str) -> bool {
        // It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
        if input
            .chars()
            .filter(|c| (*c == 'a') || (*c == 'e') || (*c == 'i') || (*c == 'o') || (*c == 'u'))
            .count()
            < 3
        {
            return false;
        }
        // It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
        let mut found_pair = false;

        // It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
        for pair in input.as_bytes().windows(2) {
            match pair {
                b"ab" => return false,
                b"cd" => return false,
                b"pq" => return false,
                b"xy" => return false,
                _ => {}
            }
            if pair[0] == pair[1] {
                found_pair = true;
            }
        }

        return found_pair;
    }

    input.iter().filter(|s| is_nice(s)).count()
}

fn part2(input: &Vec<String>) -> usize {
    fn is_nice(input: &str) -> bool {
        // It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
        let mut got_pair = false;
        for i in 0..(input.len() - 2) {
            let pair1 = [input.as_bytes()[i], input.as_bytes()[i + 1]];
            for pair2 in input.as_bytes()[i + 2..].windows(2) {
                if pair1 == pair2 {
                    got_pair = true;
                    break;
                }
            }
        }

        // It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
        if !got_pair {
            return false;
        }
        for trip in input.as_bytes().windows(3) {
            if trip[0] == trip[2] {
                return true;
            }
        }

        false
    }
    input.iter().filter(|s| is_nice(s)).count()
}

fn main() -> Result<()> {
    let input: Vec<String> = read_input(5)?.lines().map(|s| s.to_owned()).collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
