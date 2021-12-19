use std::{iter::Sum, ops::Add};
use SnailNumber::*;

#[derive(Debug, Clone)]
enum SnailNumber {
    Pair(Box<SnailNumber>, Box<SnailNumber>),
    Literal(u32),
}

impl SnailNumber {
    fn explode(&mut self, depth: usize) -> (bool, Option<u32>, Option<u32>) {
        match self {
            Literal(_) => (false, None, None),
            Pair(left, right) if depth == 4 => {
                let result = (true, left.literal(), right.literal());
                *self = Literal(0);
                result
            }
            Pair(left, right) => {
                let (change, l, mut r) = left.explode(depth + 1);
                if let Some(v) = r.take() {
                    right.add_left(v);
                }

                if change {
                    return (change, l, r);
                }

                let (change, mut l, r) = right.explode(depth + 1);

                if let Some(v) = l.take() {
                    left.add_right(v);
                }

                (change, l, r)
            }
        }
    }

    fn literal(&self) -> Option<u32> {
        match self {
            Literal(v) => Some(*v),
            _ => None,
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Pair(left, right) => left.split() || right.split(),
            Literal(n) if *n >= 10 => {
                *self = Pair(
                    Box::new(Literal(*n / 2)),
                    Box::new(Literal((*n as f32 / 2.0).ceil() as u32)),
                );
                true
            }
            _ => false,
        }
    }

    fn add_left(&mut self, value: u32) {
        match self {
            Literal(n) => *n += value,
            Pair(left, _) => left.add_left(value),
        }
    }

    fn add_right(&mut self, value: u32) {
        match self {
            Literal(n) => *n += value,
            Pair(_, right) => right.add_right(value),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Literal(n) => *n,
            Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl Add for SnailNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = Pair(Box::new(self), Box::new(rhs));
        while sum.explode(0).0 || sum.split() {}
        sum
    }
}

impl Sum for SnailNumber {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or(Literal(0))
    }
}

fn main() {
    let input = include_str!("../input").trim();
    let numbers: Vec<SnailNumber> = input.lines().map(parse).collect();

    let part_1 = numbers.iter().cloned().sum::<SnailNumber>().magnitude();

    let part_2 = numbers
        .iter()
        .flat_map(|a| {
            numbers
                .iter()
                .map(move |b| (a.clone() + b.clone()).magnitude())
        })
        .max()
        .unwrap();

    println!("Part 1: {}\nPart 2: {}", part_1, part_2);
}

fn parse(s: &str) -> SnailNumber {
    if s.starts_with('[') {
        let mut depth = 0;
        let (left, right) = s[1..s.len() - 1]
            .split_once(|c| match c {
                '[' => {
                    depth += 1;
                    false
                }
                ']' => {
                    depth -= 1;
                    false
                }
                ',' if depth == 0 => true,
                _ => false,
            })
            .unwrap();
        Pair(Box::new(parse(left)), Box::new(parse(right)))
    } else {
        Literal(s.parse().unwrap())
    }
}
