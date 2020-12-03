extern crate either;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

use either::*;

struct Parser {
    content: Vec<char>,
    curpos: usize,
    finished: bool,
}

impl Parser {
    fn new(content: &str) -> Parser {
        Parser {
            content: content.chars().collect(),
            curpos: 0,
            finished: false,
        }
    }

    fn peek(&mut self) -> char {
        if self.finished {
            panic!("Already finished!")
        } else {
            self.content[self.curpos]
        }
    }

    fn next(&mut self) {
        if !self.finished {
            self.curpos += 1;
            self.finished = self.curpos >= self.content.len();
        }
    }

    fn eat_pling(&mut self) {
        self.next();
        self.next();
    }

    fn eat_garbage(&mut self) -> usize {
        self.next();
        let mut count = 0;
        loop {
            let ch = self.peek();
            if ch == '!' {
                self.eat_pling();
                continue;
            }
            if ch == '>' {
                break;
            }
            self.next();
            count = count + 1;
        }
        self.next();
        count
    }
}

struct Group {
    elems: Vec<Either<String, Group>>,
    garbage: usize,
}

impl Group {
    fn parse_from(parser: &mut Parser, depth: usize) -> Group {
        assert!(parser.peek() == '{');
        parser.next();
        let mut elems: Vec<Either<String, Group>> = Vec::new();
        let mut accum: String = String::new();
        let mut garbage: usize = 0;
        while parser.peek() != '}' {
            let ch = parser.peek();
            if ch == '!' {
                parser.eat_pling();
                continue;
            }
            if ch == '<' {
                garbage += parser.eat_garbage();
                continue;
            }
            if ch == '}' {
                break;
            }
            if ch == ',' {
                elems.push(Left(accum));
                accum = String::new();
                parser.next();
                continue;
            }
            if ch == '{' {
                elems.push(Right(Group::parse_from(parser, depth + 1)));
                continue;
            }
            accum.push(ch);
            parser.next();
        }
        parser.next();
        elems.push(Left(accum));
        Group {
            elems: elems,
            garbage: garbage,
        }
    }

    fn parse(input: &str) -> Group {
        let mut parser = Parser::new(input);
        Group::parse_from(&mut parser, 0)
    }

    fn score_group(&self, depth: usize) -> usize {
        let mut total = depth;
        for elem in &self.elems {
            if let Right(ref grp) = *elem {
                total += grp.score_group(depth + 1);
            }
        }
        total
    }

    fn count_groups(&self) -> usize {
        let mut total = 1;
        for elem in &self.elems {
            if let Right(ref grp) = *elem {
                total += grp.count_groups();
            }
        }
        total
    }

    fn total_garbage(&self) -> usize {
        let mut total = self.garbage;
        for elem in &self.elems {
            if let Right(ref grp) = *elem {
                total += grp.total_garbage();
            }
        }
        total
    }
}

fn load_instructions() -> Group {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let line = freader.lines().next().unwrap().unwrap();
    Group::parse(&line)
}

fn run_test(input: &str, grps: usize, score: usize) {
    let grp = Group::parse(input);
    assert!(grp.count_groups() == grps);
    assert!(grp.score_group(1) == score);
}

fn main() {
    run_test("{}", 1, 1);
    run_test("{{{}}}", 3, 6);
    run_test("{{},{}}", 3, 5);
    run_test("{{{},{},{{}}}}", 6, 16);
    run_test("{<{},{},{{}}>}", 1, 1);
    run_test("{<a>,<a>,<a>,<a>}", 1, 1);
    run_test("{{<a>},{<a>},{<a>},{<a>}}", 5, 9);
    run_test("{{<!>},{<!>},{<!>},{<a>}}", 2, 3);
    run_test("{{<ab>},{<ab>},{<ab>},{<ab>}}", 5, 9);
    run_test("{{<!!>},{<!!>},{<!!>},{<!!>}}", 5, 9);
    run_test("{{<a!>},{<a!>},{<a!>},{<ab>}}", 2, 3);
    let input = load_instructions();
    println!("Problem 1: {}", input.score_group(1));
    println!("Problem 2: {}", input.total_garbage());
}
