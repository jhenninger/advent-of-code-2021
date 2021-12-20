use std::{collections::HashSet, mem};

fn main() {
    let input = include_str!("../input");

    let (a, b) = input.split_once("\n\n").unwrap();

    let e: Vec<_> = a.chars().map(|c| c == '#').collect();

    let mut img: HashSet<(i32, i32)> = b
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut next = HashSet::new();

    for i in 0..50 {
        let min_x = img.iter().map(|p| p.0).min().unwrap();
        let max_x = img.iter().map(|p| p.0).max().unwrap();
        let min_y = img.iter().map(|p| p.1).min().unwrap();
        let max_y = img.iter().map(|p| p.1).max().unwrap();

        for y in min_y-1..=max_y+1 {
            for x in min_x-1..=max_x+1 {
                let mut idx = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = x + dx;
                        let ny = y + dy;
                        idx <<= 1;
                        idx |= ((i % 2 == 1
                            && (nx < min_x || nx > max_x || ny < min_y || ny > max_y))
                            || img.contains(&(x + dx, y + dy)))
                            as usize;
                    }
                }

                if e[idx] {
                    next.insert((x, y));
                }
            }
        }

        mem::swap(&mut img, &mut next);
        next.clear();
    }

    dbg!(img.len());
}