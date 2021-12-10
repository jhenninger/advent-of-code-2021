fn main() {
    let input = include_str!("../input");

    let mut part_1 = 0;

    let mut scores: Vec<u64> = input
        .lines()
        .filter_map(|line| {
            let mut stack = Vec::new();

            for c in line.chars() {
                let points = match c {
                    '(' | '[' | '{' | '<' => {
                        stack.push(c);
                        0
                    }
                    ')' if stack.pop() != Some('(') => 3,
                    ']' if stack.pop() != Some('[') => 57,
                    '}' if stack.pop() != Some('{') => 1197,
                    '>' if stack.pop() != Some('<') => 25137,
                    _ => 0,
                };

                if points != 0 {
                    part_1 += points;
                    return None;
                }
            }

            let score = stack.iter().rev().fold(0, |acc, c| {
                let points = match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
                5 * acc + points
            });

            Some(score)
        })
        .collect();

    scores.sort_unstable();
    let part_2 = scores[scores.len() / 2];

    println!("Part 1: {}\nPart 2: {}", part_1, part_2);
}
