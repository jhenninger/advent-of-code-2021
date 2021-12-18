use std::{iter::Sum, ops::Add};

#[derive(Clone)]
struct Number(Part, Part);

#[derive(Clone)]
enum Part {
    Number(Box<Number>),
    Literal(u32),
}

impl Part {
    fn leftmost(&mut self) -> &mut u32 {
        match self {
            Part::Literal(n) => n,
            Part::Number(n) => n.0.leftmost(),
        }
    }

    fn rightmost(&mut self) -> &mut u32 {
        match self {
            Part::Literal(n) => n,
            Part::Number(n) => n.1.rightmost(),
        }
    }

    fn explode(&mut self, depth: usize) -> (bool, Option<u32>, Option<u32>) {
        match self {
            Part::Literal(_) => (false, None, None),
            Part::Number(n) => {
                if depth == 4 {
                    match **n {
                        Number(Part::Literal(left), Part::Literal(right)) => {
                            *self = Part::Literal(0);
                            (true, Some(left), Some(right))
                        }
                        _ => panic!("expected two literals"),
                    }
                } else {
                    n.explode(depth)
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Part::Literal(n) if *n >= 10 => {
                let num = Number(
                    Part::Literal(*n / 2),
                    Part::Literal((*n as f32 / 2.0).ceil() as u32),
                );
                *self = Part::Number(Box::new(num));
                true
            }
            Part::Number(n) => n.split(),
            _ => false,
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Part::Number(n) => n.magnitude(),
            Part::Literal(n) => *n,
        }
    }
}

impl Number {
    fn reduce(&mut self) {
        while self.explode(0).0 || self.split() {}
    }

    fn explode(&mut self, depth: usize) -> (bool, Option<u32>, Option<u32>) {
        let (changed, left, mut right) = self.0.explode(depth + 1);

        if let Some(value) = right.take() {
            *self.1.leftmost() += value;
        }

        if changed {
            return (true, left, right);
        }

        let (changed, mut left, right) = self.1.explode(depth + 1);

        if let Some(value) = left.take() {
            *self.0.rightmost() += value;
        }

        return (changed, left, right);
    }

    fn split(&mut self) -> bool {
        self.0.split() || self.1.split()
    }

    fn magnitude(&self) -> u32 {
        3 * self.0.magnitude() + 2 * self.1.magnitude()
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new = Number(Part::Number(Box::new(self)), Part::Number(Box::new(rhs)));
        new.reduce();
        new
    }
}

impl Sum for Number {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b)
            .unwrap_or(Number(Part::Literal(0), Part::Literal(0)))
    }
}

fn main() {
    let input = include_str!("../input").trim();

    let numbers: Vec<Number> = input.lines().map(parse_number).collect();

    let part_1 = numbers.iter().cloned().sum::<Number>().magnitude();

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

fn parse_number(s: &str) -> Number {
    let s = &s[1..s.len() - 1];

    let mut depth = 0;
    let (left, right) = s
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

    Number(parse_part(left), parse_part(right))
}

fn parse_part(s: &str) -> Part {
    if s.starts_with('[') {
        Part::Number(Box::new(parse_number(s)))
    } else {
        Part::Literal(s.parse().unwrap())
    }
}
