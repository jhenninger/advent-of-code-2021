use std::{collections::HashSet, mem};

fn main() {
    let input = include_str!("../input").trim();
    let (enhancement, image) = input.split_once("\n\n").unwrap();

    let enhancement: Vec<bool> = enhancement.chars().map(|c| c == '#').collect();

    assert!(!enhancement[0] || !enhancement[enhancement.len() - 1], "∞");

    let mut image: HashSet<(i32, i32)> = image
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((x as i32, y as i32)),
                _ => None,
            })
        })
        .collect();

    let mut part_1 = 0;

    let mut next = HashSet::new();

    for i in 0..50 {
        if i == 2 {
            part_1 = image.len();
        }

        let min_x = image.iter().map(|p| p.0).min().unwrap();
        let max_x = image.iter().map(|p| p.0).max().unwrap();
        let min_y = image.iter().map(|p| p.1).min().unwrap();
        let max_y = image.iter().map(|p| p.1).max().unwrap();

        for y in min_y - 1..=max_y + 1 {
            for x in min_x - 1..=max_x + 1 {
                let mut idx = 0;
                for ny in y - 1..=y + 1 {
                    for nx in x - 1..=x + 1 {
                        let outside_and_active = enhancement[0]
                            && i % 2 == 1
                            && (nx < min_x || nx > max_x || ny < min_y || ny > max_y);
                        idx = idx << 1 | (outside_and_active || image.contains(&(nx, ny))) as usize;
                    }
                }

                if enhancement[idx] {
                    next.insert((x, y));
                }
            }
        }

        mem::swap(&mut image, &mut next);
        next.clear();
    }

    println!("Part 1: {}\nPart 2: {}", part_1, image.len());
}
