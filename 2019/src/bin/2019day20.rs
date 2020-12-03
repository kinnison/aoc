use aoc2019::*;

#[derive(Debug, Copy, Clone)]
enum CellKind {
    Blocking,
    Passable,
    PortalLetter(u8),
}
use CellKind::*;

impl From<char> for CellKind {
    fn from(value: char) -> CellKind {
        match value {
            '#' | ' ' => Blocking,
            '.' => Passable,
            _ => PortalLetter(value as u8),
        }
    }
}

impl CellKind {
    fn get_letter(self) -> Option<u8> {
        match self {
            PortalLetter(l) => Some(l),
            _ => None,
        }
    }

    fn is_open(self) -> bool {
        matches!(self, Passable)
    }
}

#[derive(Debug, Clone)]
struct Maze {
    width: usize,
    height: usize,
    grid: Vec<CellKind>,
    portal_pairing: HashMap<(usize, usize), (usize, usize)>,
    start: (usize, usize),
    finish: (usize, usize),
}

static START: u16 = (b'A' as u16) << 8 | (b'A' as u16);
static FINISH: u16 = (b'Z' as u16) << 8 | (b'Z' as u16);

impl FromStr for Maze {
    type Err = Box<dyn std::error::Error>;

    fn from_str(value: &str) -> Result<Maze> {
        let width = value.lines().next().ok_or("No lines?")?.len();
        let height = value.lines().count();

        let grid: Vec<_> = value
            .chars()
            .filter_map(|c| {
                if c != '\n' {
                    Some(CellKind::from(c))
                } else {
                    None
                }
            })
            .collect();

        // We need to find the portals and for each portal, find its adjacent
        // spot. Portals are always oriented top-down or left-right but their
        // open spot may be on either end.
        let mut portals: HashMap<u16, (usize, usize)> = HashMap::new();
        let mut portal_pairing: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut start = None;
        let mut finish = None;

        for x in 0..width - 1 {
            for y in 0..height - 1 {
                if let Some(first) = grid[x + (y * width)].get_letter() {
                    let mut portal: u16 = (first as u16) << 8;
                    let open_spot;
                    // Let's try looking down first
                    if let Some(second) = grid[x + ((y + 1) * width)].get_letter() {
                        portal |= second as u16;
                        if y > 0 && grid[x + ((y - 1) * width)].is_open() {
                            open_spot = (x, y - 1);
                        } else {
                            open_spot = (x, y + 2);
                        }
                    } else if let Some(second) = grid[x + 1 + (y * width)].get_letter() {
                        // It wasn't down so it must be right
                        portal |= second as u16;
                        if x > 0 && grid[x - 1 + (y * width)].is_open() {
                            open_spot = (x - 1, y);
                        } else {
                            open_spot = (x + 2, y);
                        }
                    } else {
                        // Neither down nor right, so must have been up/left
                        // which means we already did it
                        continue;
                    }
                    // At this point we know the portal and the open spot
                    // if the portal is START or FINISH, note it
                    if portal == START {
                        start = Some(open_spot);
                    } else if portal == FINISH {
                        finish = Some(open_spot);
                    } else {
                        // It's not start or finish, so if we already know this
                        // portal, we have a pair, otherwise remember this for
                        // the future
                        match portals.entry(portal) {
                            Entry::Vacant(ve) => {
                                ve.insert(open_spot);
                            }
                            Entry::Occupied(oe) => {
                                let paired = *oe.get();
                                portal_pairing.insert(open_spot, paired);
                                portal_pairing.insert(paired, open_spot);
                            }
                        }
                    }
                }
            }
        }

        let start = start.ok_or("No start location found")?;
        let finish = finish.ok_or("No finish location found")?;

        Ok(Maze {
            width,
            height,
            grid,
            start,
            finish,
            portal_pairing,
        })
    }
}

impl Maze {
    fn cell_at(&self, pos: (usize, usize)) -> CellKind {
        self.grid[pos.0 + (pos.1 * self.width)]
    }
    fn going_up(&self, pos: (usize, usize)) -> bool {
        // pos is the target of the portal jump.
        // If the target of the jump is near the edge of the maze then we're
        // going down, otherwise we're going up
        !(pos.0 == 2 || pos.1 == 2 || pos.0 == (self.width - 3) || pos.1 == (self.height - 3))
    }
    fn shortest_route(&self, using_levels: bool) -> usize {
        // Breadth first wander through the maze, using the portals
        // from self.start until self.finish
        let mut best_length = std::usize::MAX;

        let mut tips: Vec<_> = vec![(self.start, 0, 0)];
        let mut shortest_here: HashMap<(usize, usize, usize), usize> = HashMap::new();
        while !tips.is_empty() {
            let old_tips: Vec<_> = tips.drain(..).collect();
            for (pos, level, pathlen) in old_tips {
                if pathlen >= best_length {
                    // No point continuing this tip, we already beat it
                    continue;
                }
                if pos == self.finish && level == 0 {
                    // We made it!
                    best_length = pathlen;
                    continue;
                }
                match shortest_here.entry((pos.0, pos.1, level)) {
                    Entry::Vacant(ve) => {
                        ve.insert(pathlen);
                    }
                    Entry::Occupied(mut oe) => {
                        if *oe.get() > pathlen {
                            oe.insert(pathlen);
                        } else {
                            continue;
                        }
                    }
                }
                for adj in surrounds(pos).iter().copied() {
                    if self.cell_at(adj).is_open() {
                        tips.push((adj, level, pathlen + 1));
                    }
                }
                if let Some(matching) = self.portal_pairing.get(&pos) {
                    // We're passing through a portal, which means we need
                    // to know if the new level is up or down
                    let newlevel = if using_levels {
                        if self.going_up(*matching) {
                            if level == 0 {
                                continue;
                            }
                            level - 1
                        } else {
                            level + 1
                        }
                    } else {
                        level
                    };
                    tips.push((*matching, newlevel, pathlen + 1));
                }
            }
        }

        best_length
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        static MAZE: &str = r"
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";
        let maze = Maze::from_str(&MAZE[1..]).expect("Unable to parse maze");
        println!("{:?}", maze);
        assert_eq!(maze.shortest_route(false), 23);
        assert_eq!(maze.shortest_route(true), 26);
    }

    #[test]
    fn test_2() {
        static MAZE: &str = r"
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";
        let maze = Maze::from_str(&MAZE[1..]).expect("Unable to parse maze");
        println!("{:?}", maze);
        assert_eq!(maze.shortest_route(false), 58);
    }

    #[test]
    fn test_3() {
        static MAZE: &str = r"
             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";
        let maze = Maze::from_str(&MAZE[1..]).expect("Unable to parse maze");
        assert_eq!(maze.shortest_route(true), 396);
    }
}

fn part1(input: &Maze) -> usize {
    input.shortest_route(false)
}

fn part2(input: &Maze) -> usize {
    input.shortest_route(true)
}

fn main() -> Result<()> {
    let input = read_input(20)?;
    let input = Maze::from_str(&input)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
