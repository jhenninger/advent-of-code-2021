use std::collections::HashSet;

fn main() {
    let input = include_str!("../input").trim();

    let (part_1, part_2): (u32, u32) = input.lines().fold((0, 0), |acc, l| {
        let (patterns, output) = l.split_once(" | ").unwrap();

        let one: HashSet<char> = patterns
            .split_whitespace()
            .find(|s| s.len() == 2)
            .map(|s| s.chars().collect())
            .unwrap();

        let four: HashSet<char> = patterns
            .split_whitespace()
            .find(|s| s.len() == 4)
            .map(|s| s.chars().collect())
            .unwrap();

        let (p1, p2) = output.split_whitespace().fold((0, 0), |(p1, p2), d| {
            let segments: HashSet<char> = d.chars().collect();
            
            // identify each digit by its segment count and its intersection with of 1 and 4
            let digit = match (
                segments.len(),
                segments.intersection(&four).count(),
                segments.intersection(&one).count(),
            ) {
                (2, _, _) => 1,
                (3, _, _) => 7,
                (4, _, _) => 4,
                (7, _, _) => 8,
                (5, 2, _) => 2,
                (5, 3, 1) => 5,
                (5, 3, 2) => 3,
                (6, 4, _) => 9,
                (6, 3, 1) => 6,
                (6, 3, 2) => 0,
                _ => panic!(),
            };

            (p1 + [1, 4, 7, 8].contains(&digit) as u32, p2 * 10 + digit)
        });

        (acc.0 + p1, acc.1 + p2)
    });

    println!("Part 1: {}\nPart 2: {}", part_1, part_2);
}
