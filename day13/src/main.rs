use std::{collections::HashSet, mem};

type Paper = HashSet<(i32, i32)>;

fn main() {
    let input = include_str!("../input").trim();

    let (dots, folds) = input.split_once("\n\n").unwrap();

    let mut paper = HashSet::new();

    for line in dots.lines() {
        let (x, y) = line.split_once(',').unwrap();
        paper.insert((x.parse().unwrap(), y.parse().unwrap()));
    }

    let mut next: HashSet<(i32, i32)> = HashSet::new();
    let mut part_1 = None;

    for fold in folds.lines() {
        let (axis, pos) = fold
            .strip_prefix("fold along ")
            .unwrap()
            .split_once('=')
            .unwrap();

        let pos: i32 = pos.parse().unwrap();

        let (fx, fy) = match axis {
            "y" => (i32::MAX, pos),
            "x" => (pos, i32::MAX),
            _ => panic!(),
        };

        for (x, y) in paper.drain() {
            if x > fx {
                next.insert((2 * fx - x, y));
            } else if y > fy {
                next.insert((x, 2 * fy - y));
            } else {
                next.insert((x, y));
            }
        }

        mem::swap(&mut paper, &mut next);
        part_1.get_or_insert_with(|| paper.len());
    }

    println!("Part 1: {}\nPart 2:", part_1.unwrap());
    print(&paper);
}

fn print(paper: &Paper) {
    let max_x = paper.iter().map(|p| p.0).max().unwrap();
    let max_y = paper.iter().map(|p| p.1).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if paper.contains(&(x, y)) {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!();
    }
}
