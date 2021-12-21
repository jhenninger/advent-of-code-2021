use std::{cmp, collections::HashMap, mem};

fn main() {
    let input = include_str!("../input").trim();
    let mut input = input
        .lines()
        .map(|l| l.split_ascii_whitespace().last().unwrap().parse().unwrap());

    let player_1 = Player::new(input.next().unwrap());
    let player_2 = Player::new(input.next().unwrap());

    let game = Game { player_1, player_2 };
    let part_1 = play_1(game.clone());

    let wins = play_2(game, &mut HashMap::new());
    let part_2 = cmp::max(wins.0, wins.1);

    println!("Part 1: {}\nPart 2: {}", part_1, part_2);
}

fn play_1(mut game: Game) -> usize {
    let mut rolls = 0;
    loop {
        let mut steps = 0;
        for _ in 0..3 {
            steps += (rolls % 100) + 1;
            rolls += 1;
        }
        game.player_1.advance(steps);
        if game.player_1.score >= 1000 {
            break rolls * game.player_2.score;
        }
        mem::swap(&mut game.player_1, &mut game.player_2);
    }
}

fn play_2(game: Game, mem: &mut HashMap<Game, (usize, usize)>) -> (usize, usize) {
    if game.player_1.score >= 21 {
        return (1, 0);
    }

    if game.player_2.score >= 21 {
        return (0, 1);
    }

    if let Some(wins) = mem.get(&game) {
        return *wins;
    }

    let mut wins = (0, 0);

    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                let mut player_1 = game.player_1.clone();
                player_1.advance(a + b + c);
                let new_wins = play_2(
                    Game {
                        player_1: game.player_2.clone(),
                        player_2: player_1,
                    },
                    mem,
                );
                wins.0 += new_wins.1;
                wins.1 += new_wins.0;
            }
        }
    }

    mem.insert(game, wins);
    wins
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Game {
    player_1: Player,
    player_2: Player,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Player {
    score: usize,
    pos: usize,
}

impl Player {
    fn new(pos: usize) -> Self {
        Player { score: 0, pos }
    }

    fn advance(&mut self, steps: usize) {
        self.pos = (self.pos + steps - 1) % 10 + 1;
        self.score += self.pos;
    }
}
