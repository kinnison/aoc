use aoc2015::*;

const WEAPONS: [(&str, i32, i32, i32); 5] = [
    ("Dagger", 8, 4, 0),
    ("Shortsword", 10, 5, 0),
    ("Warhammer", 25, 6, 0),
    ("Longsword", 40, 7, 0),
    ("Greataxe", 74, 8, 0),
];

const ARMORS: [(&str, i32, i32, i32); 5] = [
    ("Leather", 13, 0, 1),
    ("Chainmail", 31, 0, 2),
    ("Splintmail", 53, 0, 3),
    ("Bandedmail", 75, 0, 4),
    ("Platemail", 102, 0, 5),
];

const RINGS: [(&str, i32, i32, i32); 6] = [
    ("Damage +1", 25, 1, 0),
    ("Damage +2", 50, 2, 0),
    ("Damage +3", 100, 3, 0),
    ("Defense +1", 20, 0, 1),
    ("Defense +2", 40, 0, 2),
    ("Defense +3", 80, 0, 3),
];

#[derive(Debug)]
struct Combos {
    weapon: usize,
    armor: usize,
    ring1: usize,
    ring2: usize,
}

impl Combos {
    fn new() -> Combos {
        Combos {
            weapon: 0,
            armor: 0,
            ring1: 0,
            ring2: 0,
        }
    }

    fn finished(&self) -> bool {
        self.weapon == 6
    }

    fn increment(&mut self) {
        if self.weapon == 0 {
            // Initial state...
            self.weapon = 1;
            return;
        }
        if self.ring2 < 6 {
            self.ring2 += 1;
            return;
        } else {
            self.ring2 = 0;
        }
        if self.ring1 < 6 {
            self.ring1 += 1;
            return;
        } else {
            self.ring1 = 0;
        }
        if self.armor < 5 {
            self.armor += 1;
            return;
        } else {
            self.armor = 0;
        }
        self.weapon += 1;
    }
}

struct Player {
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Iterator for Combos {
    type Item = Player;

    fn next(&mut self) -> Option<Player> {
        loop {
            self.increment();
            if self.finished() {
                break;
            };
            if self.ring2 > self.ring1 {
                break;
            };
            if self.ring2 == self.ring1 && self.ring1 == 0 {
                break;
            }
        }
        if self.finished() {
            None
        } else {
            Some(Player::new(
                self.weapon - 1,
                if self.armor == 0 {
                    None
                } else {
                    Some(self.armor - 1)
                },
                if self.ring1 == 0 {
                    None
                } else {
                    Some(self.ring1 - 1)
                },
                if self.ring2 == 0 {
                    None
                } else {
                    Some(self.ring2 - 1)
                },
            ))
        }
    }
}

impl Player {
    fn new(
        weapon: usize,
        armor: Option<usize>,
        ring1: Option<usize>,
        ring2: Option<usize>,
    ) -> Player {
        let mut totcost = WEAPONS[weapon].1;
        let mut totdamg = WEAPONS[weapon].2;
        let mut totarmr = WEAPONS[weapon].3;
        print!("Wielding {}", WEAPONS[weapon].0);
        if let Some(armor) = armor {
            let armor = ARMORS[armor];
            totcost += armor.1;
            totdamg += armor.2;
            totarmr += armor.3;
            print!(" wearing {}", armor.0);
        } else {
            print!(" while naked");
        }
        if let Some(ring) = ring1 {
            let ring = RINGS[ring];
            totcost += ring.1;
            totdamg += ring.2;
            totarmr += ring.3;
            print!(" with {}", ring.0);
        } else {
            print!(" left hand unadorned");
        }
        if let Some(ring) = ring2 {
            let ring = RINGS[ring];
            totcost += ring.1;
            totdamg += ring.2;
            totarmr += ring.3;
            print!(" and {}", ring.0);
        } else {
            print!(" right hand unadorned");
        }
        println!(
            " costs {} for {} damage and {} armor",
            totcost, totdamg, totarmr
        );
        Player {
            cost: totcost,
            damage: totdamg,
            armor: totarmr,
        }
    }
}

struct Boss {
    hp: i32,
    damage: i32,
    armor: i32,
}

impl Boss {
    fn from_str(input: &str) -> Boss {
        let values: Vec<i32> = input
            .lines()
            .flat_map(|s| s.split_whitespace())
            .map(|s| s.parse())
            .filter(|p| p.is_ok())
            .map(|o| o.unwrap())
            .collect();
        assert!(values.len() == 3);
        Boss {
            hp: values[0],
            damage: values[1],
            armor: values[2],
        }
    }
}

fn player_wins(player: &Player, boss: &Boss, playerhp: i32) -> bool {
    let mut playerhp = playerhp;
    let mut bosshp = boss.hp;
    loop {
        let mut playerhit = player.damage - boss.armor;
        if playerhit < 1 {
            playerhit = 1
        }
        let mut bosshit = boss.damage - player.armor;
        if bosshit < 1 {
            bosshit = 1
        }
        // Hit the boss
        if bosshp <= playerhit {
            return true;
        }
        bosshp -= playerhit;
        // Boss hits you
        if playerhp <= bosshit {
            return false;
        }
        playerhp -= bosshit;
    }
}

fn test_fight() {
    let testplayer = Player {
        cost: 0,
        damage: 5,
        armor: 5,
    };
    let testboss = Boss {
        hp: 12,
        damage: 7,
        armor: 2,
    };
    println!(
        "Player wins in test scenario: {}",
        player_wins(&testplayer, &testboss, 8),
    );
}

fn part1(boss: &Boss, players: &[Player]) -> i32 {
    let mut leastcost = std::i32::MAX;
    for player in players {
        if player_wins(player, boss, 100) && player.cost < leastcost {
            leastcost = player.cost;
        }
    }
    leastcost
}

fn part2(boss: &Boss, players: &[Player]) -> i32 {
    let mut mostcost = std::i32::MIN;
    for player in players {
        if !player_wins(player, boss, 100) && player.cost > mostcost {
            mostcost = player.cost;
        }
    }
    mostcost
}

fn main() -> Result<()> {
    let input = Boss::from_str(&read_input(21)?);
    let players: Vec<Player> = Combos::new().collect();
    test_fight();
    println!("Part 1: {}", part1(&input, &players));
    println!("Part 2: {}", part2(&input, &players));
    Ok(())
}
