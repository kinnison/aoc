use aoc2015::*;

struct Boss {
    hp: i32,
    dmg: i32,
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
        assert!(values.len() == 2);
        Boss {
            hp: values[0],
            dmg: values[1],
        }
    }
}

#[derive(Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    fn castable(&self, game: &GameState) -> bool {
        if self.cost() > game.mana {
            false
        } else {
            match self {
                Spell::MagicMissile => true,
                Spell::Drain => true,
                Spell::Shield => game.shield == 0,
                Spell::Poison => game.poison == 0,
                Spell::Recharge => game.recharge == 0,
            }
        }
    }

    fn cast(&self, game: &GameState) -> GameState {
        assert!(self.castable(game));
        let mut ret = *game;
        ret.mana -= self.cost();
        ret.manaspent += self.cost();
        match self {
            Spell::MagicMissile => ret.bosshp -= 4,
            Spell::Drain => {
                ret.bosshp -= 2;
                ret.playerhp += 2
            }
            Spell::Shield => ret.shield = 6,
            Spell::Poison => ret.poison = 6,
            Spell::Recharge => ret.recharge = 5,
        }
        ret
    }
}

#[derive(Copy, Clone)]
struct GameState {
    mana: i32,
    playerhp: i32,
    bosshp: i32,
    bossdmg: i32,
    shield: usize,
    poison: usize,
    recharge: usize,
    manaspent: i32,
    hardmode: bool,
}

impl GameState {
    fn new(boss: &Boss, hard: bool) -> GameState {
        GameState {
            mana: 500,
            playerhp: 50,
            bosshp: boss.hp,
            bossdmg: boss.dmg,
            shield: 0,
            poison: 0,
            recharge: 0,
            manaspent: 0,
            hardmode: hard,
        }
    }

    fn print(&self) {
        println!(
            "Player: hp={} mana={} manaspent={}{}",
            self.playerhp,
            self.mana,
            self.manaspent,
            if self.hardmode { " **HARD MODE**" } else { "" }
        );
        println!("Boss: hp={} dmg={}", self.bosshp, self.bossdmg);
        if self.shield > 0 || self.poison > 0 || self.recharge > 0 {
            print!("Effects active:");
            if self.shield > 0 {
                print!(" shield={}", self.shield);
            }
            if self.poison > 0 {
                print!(" poison={}", self.poison);
            }
            if self.recharge > 0 {
                print!(" recharge={}", self.recharge);
            }
            println!();
        }
        println!();
    }

    fn run_timers(&mut self) {
        if self.shield > 0 {
            self.shield -= 1;
        }
        if self.poison > 0 {
            self.poison -= 1;
            self.bosshp -= 3;
        }
        if self.recharge > 0 {
            self.recharge -= 1;
            self.mana += 101;
        }
    }

    fn boss_turn(&self) -> Either<GameState, Option<i32>> {
        let mut ret = *self;
        ret.run_timers();
        if ret.bosshp > 0 {
            // Boss can deal some damage
            if ret.shield > 0 {
                let attack = if ret.bossdmg > 7 { ret.bossdmg - 7 } else { 1 };
                ret.playerhp -= attack;
            } else {
                ret.playerhp -= ret.bossdmg;
            }
        }
        if let Some(result) = self.game_result() {
            Right(result)
        } else {
            Left(ret)
        }
    }

    fn game_result(&self) -> Option<Option<i32>> {
        if self.bosshp <= 0 {
            Some(Some(self.manaspent))
        } else if self.playerhp <= 0 {
            Some(None)
        } else {
            None
        }
    }

    fn player_turn(&self) -> Either<Vec<GameState>, Option<i32>> {
        // Player has spell choices, they die only if they cannot make any turn
        let mut basegame = *self;
        // In hard mode, we wallop the player now for 1 point
        if self.hardmode {
            basegame.playerhp -= 1;
            // Despite the instructions saying "Before any effects apply"
            // If we check for death here, we don't get an answer because
            // we die before the boss can from an active poison.
            //
            // Sigh.
            //
            // if let Some(result) = basegame.game_result() {
            //    return Right(result);
            // }
        }
        basegame.run_timers();
        let mut ret = Vec::new();
        if let Some(result) = basegame.game_result() {
            return Right(result);
        }

        if Spell::MagicMissile.castable(&basegame) {
            ret.push(Spell::MagicMissile.cast(&basegame));
        }
        if Spell::Drain.castable(&basegame) {
            ret.push(Spell::Drain.cast(&basegame));
        }
        if Spell::Poison.castable(&basegame) {
            ret.push(Spell::Poison.cast(&basegame));
        }
        if Spell::Shield.castable(&basegame) {
            ret.push(Spell::Shield.cast(&basegame));
        }
        if Spell::Recharge.castable(&basegame) {
            ret.push(Spell::Recharge.cast(&basegame));
        }

        if !ret.is_empty() {
            Left(ret)
        } else {
            Right(None) // Player dies, can't cast any spells
        }
    }
}

fn full_turn(input: &GameState) -> Vec<Either<GameState, Option<i32>>> {
    // A full turn is a player turn, then a boss turn on each of the
    // player turns.  The result of a full turn is a list of games
    // where is game is either a new state for a new turn, or else an
    // indication of if the boss or player won.
    match input.player_turn() {
        Either::Left(mut moves) => {
            // The player was not immediately dead, for each resulting game
            // state in moves, turn it into a result if appropriate
            moves
                .drain(..)
                .map(|gs| {
                    if let Some(result) = gs.game_result() {
                        Right(result)
                    } else {
                        Left(gs)
                    }
                })
                .collect()
        }
        Either::Right(result) => {
            // The player is either dead or has won, so let's report that
            vec![Right(result)]
        }
    }
    .drain(..)
    .map(|pm| {
        match pm {
            Left(gs) => {
                // There is a gamestate, so make a boss move...
                gs.boss_turn()
            }
            Right(res) => Right(res),
        }
    })
    .collect()
}

fn play_game(base: &GameState) -> Vec<Option<i32>> {
    println!("\n\nPlaying a game, initial state is:");
    base.print();
    let mut results = HashSet::new();
    let mut turns = full_turn(&base);
    //let mut goes = 0;
    while !turns.is_empty() {
        //println!("************************************************************************");
        //println!("Gathered {} turns", turns.len());
        let curs: Vec<Either<GameState, Option<i32>>> = turns.drain(..).collect();
        for gs in curs {
            match gs {
                Left(game) => {
                    turns.extend(full_turn(&game));
                }
                Right(result) => {
                    results.insert(result);
                }
            }
        }
        //goes += 1;
        //println!(
        //    "At the end of {} goes we have {} things to think about and {} results",
        //    goes,
        //    turns.len(),
        //    results.len()
        //);
    }
    results.into_iter().collect()
}

fn part1(boss: &Boss) -> i32 {
    // Find minimum mana needed to kill boss...
    play_game(&GameState::new(&boss, false))
        .iter()
        .map(|o| o.unwrap_or(std::i32::MAX))
        .min()
        .unwrap_or(std::i32::MAX)
}

fn part2(boss: &Boss) -> i32 {
    // Find minimum mana needed to kill boss when in hard mode...
    play_game(&GameState::new(&boss, true))
        .iter()
        .map(|o| o.unwrap_or(std::i32::MAX))
        .min()
        .unwrap_or(std::i32::MAX)
}

fn main() -> Result<()> {
    let input = Boss::from_str(&read_input(22)?);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
