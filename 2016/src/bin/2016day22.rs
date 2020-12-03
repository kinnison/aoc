#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

use regex::Regex;

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
}

impl Node {
    fn new(t_: String) -> Option<Node> {
        lazy_static! {
            static ref NODE_RE: Regex =
                Regex::new("node-x([0-9]+)-y([0-9]+)\\s*([0-9]+)T\\s*([0-9]+)T\\s*([0-9]+)T\\s*")
                    .unwrap();
        }
        if let Some(cap) = NODE_RE.captures(&t_) {
            let x_ = cap.get(1);
            let y_ = cap.get(2);
            let size_ = cap.get(3);
            let used_ = cap.get(4);
            let avail_ = cap.get(5);
            let x: usize = x_.unwrap().as_str().parse().unwrap();
            let y: usize = y_.unwrap().as_str().parse().unwrap();
            let size: usize = size_.unwrap().as_str().parse().unwrap();
            let used: usize = used_.unwrap().as_str().parse().unwrap();
            let avail: usize = avail_.unwrap().as_str().parse().unwrap();
            let ret = Node { x, y, size, used };
            assert_eq!(ret.avail(), avail);
            Some(ret)
        } else {
            None
        }
    }

    fn avail(&self) -> usize {
        assert!(self.size >= self.used);
        self.size - self.used
    }

    #[allow(clippy::clippy::if_same_then_else, clippy::needless_bool)]
    fn viable_with(&self, other: &Node) -> bool {
        // A node is 'Viable' with another node iff:
        // self is not empty
        // self != other
        // self.used <= other.avail()
        if self.used == 0 {
            false
        } else if self.x == other.x && self.y == other.y {
            false
        } else if self.used > other.avail() {
            false
        } else {
            true
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum NS {
    Empty,
    Used,
    Goal,
    Blocking,
}

#[derive(Debug)]
struct Grid {
    nodes: Vec<Node>,
    states: Vec<Vec<NS>>,
    empty_x: usize,
    empty_y: usize,
    max_x: usize,
    max_y: usize,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

static ARBITRARY_THRESHOLD: usize = 100;

impl Grid {
    fn new() -> Grid {
        Grid {
            nodes: Vec::new(),
            states: Vec::new(),
            empty_x: 0,
            empty_y: 0,
            max_x: 0,
            max_y: 0,
        }
    }

    fn node(&self, x: usize, y: usize) -> &Node {
        for i in &self.nodes {
            if i.x == x && i.y == y {
                return i;
            }
        }
        panic!("Unable to find node x{} y{}", x, y);
    }

    fn node_idx(&self, x: usize, y: usize) -> usize {
        for (idx, ref i) in self.nodes.iter().enumerate() {
            if i.x == x && i.y == y {
                return idx;
            }
        }
        panic!("Unable to find node x{} y{}", x, y);
    }

    fn setup_states(&mut self) {
        let max_x = self.nodes.iter().map(|n| n.x).fold(0, std::cmp::max);
        let max_y = self.nodes.iter().map(|n| n.y).fold(0, std::cmp::max);
        let mut empty_x: usize = 0;
        let mut empty_y: usize = 0;
        for x in 0..(max_x + 1) {
            let mut col: Vec<NS> = Vec::new();
            for y in 0..(max_y + 1) {
                let node = self.node(x, y);
                // In the example, it's only worth considering nodes
                // as empty, used, (used as goal), or blocking
                if node.used == 0 {
                    col.push(NS::Empty);
                    empty_x = x;
                    empty_y = y;
                } else if node.used > ARBITRARY_THRESHOLD {
                    col.push(NS::Blocking);
                } else {
                    col.push(NS::Used);
                }
            }
            self.states.push(col);
        }
        // Finally we know the data we want is in the top left
        // x == max_x, y == 0
        assert_eq!(self.states[max_x][0], NS::Used);
        self.states[max_x][0] = NS::Goal;
        self.max_x = max_x;
        self.max_y = max_y;
        self.empty_x = empty_x;
        self.empty_y = empty_y;
        println!(
            "At end of grid generation, max_x={}, max_y={}, empty_x={}, empty_y={}",
            self.max_x, self.max_y, self.empty_x, self.empty_y
        );
    }

    fn viable_pairs(&self) -> Vec<(usize, usize)> {
        let mut ret: Vec<(usize, usize)> = Vec::new();
        for i in 0..self.nodes.len() {
            for j in 0..self.nodes.len() {
                if self.nodes[i].viable_with(&self.nodes[j]) {
                    ret.push((i, j));
                }
            }
        }
        ret
    }

    fn show_grid(&self) {
        print!("    ");
        for x in 0..(self.max_x + 1) {
            if (x % 5) == 0 {
                print!("|");
            } else {
                print!(" ");
            }
        }
        println!();
        for y in 0..(self.max_y + 1) {
            print!("{:3} ", y);
            for x in 0..(self.max_x + 1) {
                match self.states[x][y] {
                    NS::Empty => {
                        print!("_");
                    }
                    NS::Used => {
                        print!(".");
                    }
                    NS::Goal => {
                        print!("G");
                    }
                    NS::Blocking => {
                        print!("#");
                    }
                }
            }
            println!();
        }
    }

    // Moves are always from the perspective of the empty data space
    // a move is possible if the emptyness can move from where it is
    // to where it's going.
    fn move_possible(&self, mv: Move) -> bool {
        if !match mv {
            Move::Up => self.empty_y > 0,
            Move::Down => self.empty_y < self.max_y,
            Move::Left => self.empty_x > 0,
            Move::Right => self.empty_x < self.max_x,
        } {
            return false;
        }
        match mv {
            Move::Up => self
                .node(self.empty_x, self.empty_y - 1)
                .viable_with(self.node(self.empty_x, self.empty_y)),
            Move::Down => self
                .node(self.empty_x, self.empty_y + 1)
                .viable_with(self.node(self.empty_x, self.empty_y)),
            Move::Left => self
                .node(self.empty_x - 1, self.empty_y)
                .viable_with(self.node(self.empty_x, self.empty_y)),
            Move::Right => self
                .node(self.empty_x + 1, self.empty_y)
                .viable_with(self.node(self.empty_x, self.empty_y)),
        }
    }

    fn do_move(&mut self, mv: Move) {
        assert!(self.move_possible(mv));
        let empty_idx = self.node_idx(self.empty_x, self.empty_y);
        let target_idx = match mv {
            Move::Up => self.node_idx(self.empty_x, self.empty_y - 1),
            Move::Down => self.node_idx(self.empty_x, self.empty_y + 1),
            Move::Left => self.node_idx(self.empty_x - 1, self.empty_y),
            Move::Right => self.node_idx(self.empty_x + 1, self.empty_y),
        };
        assert!(self.nodes[empty_idx].used == 0);
        assert!(self.nodes[target_idx].used != 0);
        self.nodes[empty_idx].used = self.nodes[target_idx].used;
        self.nodes[target_idx].used = 0;
        self.states[self.empty_x][self.empty_y] =
            self.states[self.nodes[target_idx].x][self.nodes[target_idx].y];
        self.states[self.nodes[target_idx].x][self.nodes[target_idx].y] = NS::Empty;
        match mv {
            Move::Up => self.empty_y -= 1,
            Move::Down => self.empty_y += 1,
            Move::Left => self.empty_x -= 1,
            Move::Right => self.empty_x += 1,
        }
    }
}

fn load_grid() -> Grid {
    let infile = File::open("day22.input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = Grid::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        if let Some(node) = Node::new(line) {
            ret.nodes.push(node);
        }
    }
    ret.setup_states();
    ret
}

fn problem1() -> usize {
    let grid = load_grid();

    grid.viable_pairs().len()
}

fn problem2() -> usize {
    let mut grid = load_grid();
    // My grid appears to be mostly movable, with a blocking line of
    // nodes preventing a straight line move from the empty to the goal
    // of walking Empty up to the top
    let mut movecount: usize = 0;
    println!("Initial grid:");
    grid.show_grid();
    grid.do_move(Move::Up);
    movecount += 1;
    grid.do_move(Move::Up);
    movecount += 1;
    // Now we move left as long as we can't move up
    while !grid.move_possible(Move::Up) {
        grid.do_move(Move::Left);
        movecount += 1;
    }
    // Now, while we can move up...
    while grid.move_possible(Move::Up) {
        grid.do_move(Move::Up);
        movecount += 1;
    }
    // And now while we can move right...
    while grid.move_possible(Move::Right) {
        grid.do_move(Move::Right);
        movecount += 1;
    }
    // Finally the sequence of operations to move the
    // goal left is...
    while grid.states[0][0] != NS::Goal {
        grid.do_move(Move::Down);
        movecount += 1;
        grid.do_move(Move::Left);
        movecount += 1;
        grid.do_move(Move::Left);
        movecount += 1;
        grid.do_move(Move::Up);
        movecount += 1;
        grid.do_move(Move::Right);
        movecount += 1;
    }
    println!("Final grid:");
    grid.show_grid();
    movecount
}

fn main() {
    println!("Problem 1: {}", problem1());
    println!("Problem 2: {}", problem2());
}
