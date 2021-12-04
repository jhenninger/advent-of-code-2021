use std::{num::ParseIntError, str::FromStr};

struct Board {
    board: Vec<Vec<(u32, bool)>>,
}

impl Board {
    fn mark(&mut self, n: u32) -> bool {
        for (y, row) in self.board.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if cell.0 == n {
                    cell.1 = true;
                    return self.board[y].iter().all(|cell| cell.1)
                        || self.board.iter().all(|row| row[x].1);
                }
            }
        }
        false
    }

    fn score(&self) -> u32 {
        self.board
            .iter()
            .flatten()
            .filter(|c| !c.1)
            .map(|c| c.0)
            .sum()
    }
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board = s
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|n| Ok((n.parse()?, false)))
                    .collect()
            })
            .collect::<Result<_, _>>()?;
        Ok(Board { board })
    }
}

fn main() {
    let mut input = include_str!("../input").split("\n\n");

    let numbers: Vec<u32> = input
        .next()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .unwrap();

    let mut winning_order: Vec<(usize, u32, Board)> = input
        .map(|b| {
            let mut board: Board = b.parse().unwrap();
            let (i, &n) = numbers
                .iter()
                .enumerate()
                .find(|(_, &n)| board.mark(n))
                .unwrap();
            (i, n, board)
        })
        .collect();

    winning_order.sort_unstable_by_key(|b| b.0);

    println!(
        "Part 1: {}\nPart 2: {}",
        winning_order.first().map(|b| b.1 * b.2.score()).unwrap(),
        winning_order.last().map(|b| b.1 * b.2.score()).unwrap(),
    );
}
