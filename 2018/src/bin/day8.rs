use aoc2018::*;

struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn from_iter(iter: &mut Iterator<Item = &usize>) -> Option<Node> {
        let n_children = *iter.next()?;
        let n_metadata = *iter.next()?;
        let mut children = Vec::new();
        let mut metadata = Vec::new();
        for _i in 0..n_children {
            children.push(Node::from_iter(iter)?);
        }
        for _i in 0..n_metadata {
            metadata.push(*iter.next()?);
        }
        Some(Node { children, metadata })
    }

    fn sum_metadata(&self) -> usize {
        let metasum: usize = self.metadata.iter().cloned().sum();
        let kidsum: usize = self.children.iter().map(|n| n.sum_metadata()).sum();
        metasum + kidsum
    }
    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.sum_metadata()
        } else {
            self.metadata
                .iter()
                .cloned()
                .filter(|&n| n > 0 && n <= self.children.len())
                .map(|n| self.children[n - 1].value())
                .sum()
        }
    }
}

fn part1(input: &[usize]) -> Result<usize> {
    let tree = Node::from_iter(&mut input.iter()).ok_or("Unable to parse tree")?;
    Ok(tree.sum_metadata())
}

fn part2(input: &[usize]) -> Result<usize> {
    let tree = Node::from_iter(&mut input.iter()).ok_or("Unable to parse tree")?;
    Ok(tree.value())
}

fn main() -> Result<()> {
    let test_input: Result<Vec<usize>> = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"
        .split_whitespace()
        .map(|s| Ok(s.parse()?))
        .collect();
    let test_input = test_input?;
    let input: Result<Vec<usize>> = read_input(8)?
        .split_whitespace()
        .map(|s| Ok(s.parse()?))
        .collect();
    let input = input?;
    println!("Test 1: {}", part1(&test_input)?);
    println!("Part 1: {}", part1(&input)?);
    println!("Test 2: {}", part2(&test_input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}
