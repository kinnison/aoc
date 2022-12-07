use aoc2022::*;

use std::path::{Path, PathBuf};

#[derive(ParseByRegex)]
enum InputLine {
    #[regex = r"\$ cd /"]
    ChdirRoot,
    #[regex = r"\$ cd \.\."]
    ChdirUp,
    #[regex = r"\$ cd (.+)"]
    Chdir(String),
    #[regex = r"\$ ls"]
    List,
    #[regex = r"dir (.+)"]
    DirEntry(String),
    #[regex = r"(\d+) (.+)"]
    FileEntry(usize, String),
}

fn make_dirmap(input: &[InputLine]) -> (HashMap<PathBuf, usize>, usize) {
    let mut dirmap = HashMap::new();
    dirmap.insert(PathBuf::from("/"), 0usize);

    let mut cwd = PathBuf::from("/");
    let mut paths = vec![cwd.clone()];
    for entry in input {
        match entry {
            InputLine::ChdirRoot => {
                cwd = PathBuf::from("/");
                paths = vec![cwd.clone()];
            }
            InputLine::ChdirUp => {
                cwd.pop();
                paths.pop();
            }
            InputLine::Chdir(d) => {
                cwd.push(d);
                paths.push(cwd.clone());
            }
            InputLine::List => {}
            InputLine::DirEntry(_) => {}
            InputLine::FileEntry(sz, _) => {
                for dir in &paths {
                    *(dirmap.entry(dir.clone()).or_default()) += *sz;
                }
            }
        }
    }
    let root_size = *dirmap.get(&paths[0]).unwrap();
    (dirmap, root_size)
}

fn part1(input: &[InputLine]) -> usize {
    make_dirmap(input)
        .0
        .into_iter()
        .map(|(_, s)| s)
        .filter(|&s| s <= 100000)
        .sum()
}

const NEEDED_SPACE: usize = 30000000;
const TOTAL_SPACE: usize = 70000000;
fn part2(input: &[InputLine]) -> usize {
    let (dirmap, root_size) = make_dirmap(input);
    let avail = TOTAL_SPACE - root_size;
    dirmap
        .into_iter()
        .map(|(_, s)| s)
        .filter(|sz| (*sz + avail) >= NEEDED_SPACE)
        .sorted()
        .next()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 95437);
    }

    #[test]
    fn testcase2() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 24933642);
    }
}

fn main() -> Result<()> {
    let input: Vec<InputLine> = read_input_as_vec(7)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
