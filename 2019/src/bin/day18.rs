use aoc2019::*;

#[derive(Debug, Clone, Copy)]
enum CellKind {
    Open,
    Wall,
    Key(u8),
    Door(u8),
}

impl TryFrom<char> for CellKind {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: char) -> Result<Self> {
        use CellKind::*;
        match value {
            '#' => Ok(Wall),
            '@' | '.' => Ok(Open),
            _ if value >= 'a' && value <= 'z' => Ok(Key(KEY_OFFSET + (value as u8) - b'a')),
            _ if value >= 'A' && value <= 'Z' => Ok(Door(KEY_OFFSET + (value as u8) - b'A')),
            _ => Err(format!("Unknown maze character: {:?}", value).into()),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Default, Copy, Clone, Hash)]
struct KeySet([bool; 30]);
static KEY_OFFSET: u8 = 4;

impl fmt::Display for KeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..30 {
            write!(f, "{}", if self.0[i] { 'X' } else { '_' })?;
        }
        write!(f, "]")
    }
}

impl From<[bool; 30]> for KeySet {
    fn from(value: [bool; 30]) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for KeySet {
    type Target = [bool; 30];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for KeySet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn can_reach(keys_held: &KeySet, doors: &KeySet) -> bool {
    for (key, door) in keys_held
        .iter()
        .skip(1)
        .copied()
        .zip(doors.iter().skip(1).copied())
    {
        if door && !key {
            return false;
        }
    }
    true
}

// impl CellKind {
//     fn passable(self, keys_held: &KeySet) -> bool {
//         use CellKind::*;
//         match self {
//             Wall => false,
//             Open => true,
//             Key(_) => true,
//             Door(v) => keys_held[v as usize],
//         }
//     }
// }

#[derive(Debug, Clone)]
struct Maze {
    width: usize,
    height: usize,
    biggest_key: u8,
    key_locations: Vec<(usize, usize)>,
    cells: Vec<CellKind>,
}

impl Maze {
    fn new(s: &str) -> Result<Self> {
        let mut cells = Vec::new();
        let mut width = 0;
        let mut height = 0;
        let mut key_locations = Vec::new();
        key_locations.resize(30, (0, 0));
        let mut keys_found = 0;
        let mut biggest_key = 0;
        for l in s.lines() {
            for (col, ch) in l.chars().enumerate() {
                if ch == '@' {
                    key_locations[keys_found] = (col, height);
                    if keys_found == 0 {
                        key_locations[1] = key_locations[0];
                        key_locations[2] = key_locations[1];
                        key_locations[3] = key_locations[2];
                    }
                    keys_found += 1;
                }
                let cell = CellKind::try_from(ch)?;
                if let CellKind::Key(k) = cell {
                    biggest_key = max(biggest_key, k);
                    key_locations[k as usize] = (col, height);
                }
                cells.push(cell);
            }
            width = l.len();
            height += 1;
        }

        Ok(Self {
            width,
            height,
            cells,
            biggest_key,
            key_locations,
        })
    }

    fn cell_at(&self, x: usize, y: usize) -> CellKind {
        self.cells[x + (y * self.width)]
    }

    fn find_shortest_route(
        &self,
        start: (usize, usize),
        target: (usize, usize),
    ) -> (usize, KeySet) {
        // We want to do a breadth-first search from key to target_key
        // noting any doors we pass through.  We want the shortest route
        // If on our route we pass by another key, we ignore that
        let mut best_len = std::usize::MAX;
        let mut best_doors = KeySet::default();

        let mut visited: HashMap<(usize, usize), usize> = (0..self.width)
            .flat_map(|x| {
                (0..self.height).filter_map(move |y| match self.cell_at(x, y) {
                    CellKind::Wall => Some((x, y)),
                    _ => None,
                })
            })
            .map(|k| (k, 0))
            .collect();

        // To do the BFS, we store a set of tip,length,doors and we trim whenever
        // length is longer than best_len, and we set best_len and best_doors
        // when we find the target key
        let mut tips: Vec<((usize, usize), usize, KeySet)> = vec![(start, 0, KeySet::default())];
        visited.insert(start, 0);
        while !tips.is_empty() {
            let old_tips: Vec<_> = tips.drain(..).collect();
            'tip: for (loc, pathlen, doors) in old_tips {
                let newlen = pathlen + 1;
                if newlen > best_len {
                    // We've gone on too long, best abandon this route
                    continue 'tip;
                }
                for newloc in &surrounds(loc) {
                    if *newloc == target {
                        // We have found our way to the target
                        if newlen < best_len {
                            // And it was the best pathway
                            best_len = newlen;
                            best_doors = doors;
                        }
                        continue 'tip;
                    }
                    let mut newdoors = doors;
                    if let CellKind::Door(d) = self.cell_at(newloc.0, newloc.1) {
                        newdoors[d as usize] = true;
                    }
                    match visited.entry(*newloc) {
                        ve @ Entry::Vacant(_) => {
                            ve.or_insert(newlen);
                            let new_tip = (*newloc, newlen, newdoors);
                            tips.push(new_tip);
                        }
                        Entry::Occupied(_) => {
                            // We've been to this cell before, and since it's
                            // not possible to be here faster because all tips
                            // advance at the same rate, we do nothing here.
                        }
                    }
                }
            }
        }

        (best_len, best_doors)
    }

    #[allow(clippy::too_many_arguments)]
    fn collect_dfs(
        routes: &HashMap<u8, HashMap<u8, (usize, KeySet)>>,
        current_key: u8,
        max_key: u8,
        mut keys_held: KeySet,
        goal_set: &KeySet,
        pathlen: usize,
        best_path: &mut usize,
        trimmings: &mut HashMap<(u8, KeySet), usize>,
    ) {
        // If our current state is worse than the trimmings, then abort
        if let Some(depth) = trimmings.get(&(current_key, keys_held)) {
            if pathlen >= *depth {
                // We were already here before, sooner
                return;
            }
        }
        trimmings.insert((current_key, keys_held), pathlen);
        // We're doing a depth-first search from current_key toward goal_set
        let cur_keys = keys_held;
        for key in (KEY_OFFSET..=max_key)
            // Remove keys which we already have
            .filter(|k| !cur_keys[*k as usize])
            // Now remove keys which are not reachable from this location
            .filter(|k| can_reach(&cur_keys, &routes[&current_key][k].1))
        {
            // key is a candidate
            let route = &routes[&current_key][&key];
            let newpath = pathlen + route.0;
            if newpath < *best_path {
                // And it's potentially still shorter than the best path
                keys_held[key as usize] = true;
                if keys_held == *goal_set {
                    // We've found a path
                    *best_path = newpath
                } else {
                    // We need to recurse
                    Maze::collect_dfs(
                        routes, key, max_key, keys_held, goal_set, newpath, best_path, trimmings,
                    );
                }
                // And relinquish the key to proceed
                keys_held[key as usize] = false;
            }
        }
    }

    fn collect_keys(&self) -> Result<usize> {
        // Since we *MUST* visit every key, that means we need to route
        // from any key to any other key, that's 27*26 routes though
        // obviously the fastest route from a->b is the same b->a
        // We can then analyse that reduced dataset to determine the
        // fastest way to collect all the keys
        let mut routes: HashMap<u8, HashMap<u8, (usize, KeySet)>> = HashMap::new();
        for key in 0..=self.biggest_key {
            for target_key in key + 1..=self.biggest_key {
                let route = self.find_shortest_route(
                    self.key_locations[key as usize],
                    self.key_locations[target_key as usize],
                );
                *routes
                    .entry(key)
                    .or_default()
                    .entry(target_key)
                    .or_default() = route;
                *routes
                    .entry(target_key)
                    .or_default()
                    .entry(key)
                    .or_default() = route;
            }
        }

        //println!("Routes: {:#?}", routes);

        // We've found the routes, now we need to find the best route from
        // the start (key 0) to the point of holding all the keys
        // We cannot do that with a BFS of possible routes because we run out
        // of RAM and need factorial time.
        let mut best_len = std::usize::MAX;
        let target_keys = {
            let mut keys = KeySet::default();
            for i in KEY_OFFSET..=self.biggest_key {
                keys[i as usize] = true;
            }
            keys
        };
        // As such we do a depth-first search, trimming whenever we exceed
        // best_len
        Maze::collect_dfs(
            &routes,
            0,
            self.biggest_key,
            KeySet::default(),
            &target_keys,
            0,
            &mut best_len,
            &mut HashMap::new(),
        );

        Ok(best_len)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        static MAP: &str = r"
#########
#b.A.@.a#
#########";
        let maze = Maze::new(MAP.trim()).expect("Unable to parse maze");
        println!("Maze: {:?}", maze);
        assert_eq!(maze.collect_keys().expect("Unable to collect keys?"), 8);
    }
    #[test]
    fn test_2() {
        static MAP: &str = r"
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        let maze = Maze::new(MAP.trim()).expect("Unable to parse maze");
        println!("Maze: {:?}", maze);
        assert_eq!(maze.collect_keys().expect("Unable to collect keys?"), 86);
    }
}

fn part1(input: &Maze) -> Result<usize> {
    input.collect_keys()
}

fn main() -> Result<()> {
    let input = read_input(18)?;
    let input = Maze::new(&input)?;

    println!("Part 1: {}", part1(&input)?);
    Ok(())
}
