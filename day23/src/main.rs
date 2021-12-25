use std::cmp;

use Pod::*;

fn main() {
    let input_1 = Burrow {
        hallway: [None; 11],
        size: 2,
        rooms: [vec![C, D], vec![A, C], vec![A, D], vec![B, B]],
    };

    let input_2 = Burrow {
        hallway: [None; 11],
        size: 4,
        rooms: [
            vec![C, D, D, D],
            vec![A, B, C, C],
            vec![A, A, B, D],
            vec![B, C, A, B],
        ],
    };

    println!(
        "Part 1: {}\nPart 2: {}",
        solve(input_1, 0).unwrap(),
        solve(input_2, 0).unwrap(),
    );
}

// brute force all possible solutions
fn solve(mut state: Burrow, mut cost: usize) -> Option<usize> {
    // if a pod can move into the right home, do it, this is always correct
    'outer: loop {
        for (i, &pod) in state.hallway.iter().enumerate() {
            let pod = match pod {
                Some(pod) => pod,
                _ => continue,
            };

            let room = pod.room();

            if let Some(steps) = state.can_move(i, room * 2 + 2) {
                let target = &mut state.rooms[room];
                if target.iter().all(|p| p.room() == room) {
                    cost += (steps + state.size - target.len()) * pod.cost();
                    target.push(pod);
                    state.hallway[i] = None;
                    continue 'outer;
                }
            }
        }

        break;
    }

    // check for win

    if state.hallway.iter().all(|e| e.is_none())
        && state
            .rooms
            .iter()
            .enumerate()
            .all(|(i, d)| d.iter().all(|p| p.room() == i))
    {
        return Some(cost);
    }

    // otherwise move pods out of their rooms into all possible slots

    let parking = [0, 1, 3, 5, 7, 9, 10];

    let mut results = Vec::new();

    for (i, door) in state.rooms.iter().enumerate() {
        if door.iter().all(|p| p.room() == i) {
            continue;
        }

        for p in parking {
            if let Some(mut steps) = state.can_move(i * 2 + 2, p) {
                let mut state = state.clone();
                let pod = state.rooms[i].pop().unwrap();
                state.hallway[p] = Some(pod);
                steps += state.size - state.rooms[i].len();
                results.push(solve(state, cost + steps * pod.cost()));
            }
        }
    }

    results.iter().flatten().min().cloned()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pod {
    A,
    B,
    C,
    D,
}

impl Pod {
    fn cost(self) -> usize {
        match self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }

    fn room(self) -> usize {
        match self {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Burrow {
    hallway: [Option<Pod>; 11],
    rooms: [Vec<Pod>; 4],
    size: usize,
}

impl Burrow {
    fn can_move(&self, from: usize, to: usize) -> Option<usize> {
        if from == to {
            return None;
        }

        let min = cmp::min(from, to);
        let max = cmp::max(from, to);

        for i in min..=max {
            if self.hallway[i].is_some() && i != from {
                return None;
            }
        }

        Some(max - min)
    }
}
