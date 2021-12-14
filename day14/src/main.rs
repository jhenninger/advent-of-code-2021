use std::{collections::HashMap, mem};

fn main() {
    let input = include_str!("../input").trim();

    let (polymer, rules) = input.split_once("\n\n").unwrap();

    let rules: HashMap<(u8, u8), u8> = rules
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();
            let from = from.as_bytes();
            ((from[0], from[1]), to.as_bytes()[0])
        })
        .collect();

    println!(
        "Part 1: {}\nPart 2: {}",
        solve(polymer.as_bytes(), &rules, 10),
        solve(polymer.as_bytes(), &rules, 40),
    )
}

fn solve(polymer: &[u8], rules: &HashMap<(u8, u8), u8>, iterations: usize) -> usize {
    let mut pairs: HashMap<(u8, u8), usize> = HashMap::new();

    for pair in polymer.windows(2) {
        *pairs.entry((pair[0], pair[1])).or_default() += 1;
    }

    let mut counts: HashMap<u8, usize> = HashMap::new();

    for &b in polymer {
        *counts.entry(b).or_default() += 1;
    }

    let mut next = HashMap::new();

    for _ in 0..iterations {
        for (pair, count) in pairs.drain() {
            let insert = rules[&pair];
            *next.entry((pair.0, insert)).or_default() += count;
            *next.entry((insert, pair.1)).or_default() += count;
            *counts.entry(insert).or_default() += count;
        }

        mem::swap(&mut next, &mut pairs);
    }

    counts.values().max().unwrap() - counts.values().min().unwrap()
}
