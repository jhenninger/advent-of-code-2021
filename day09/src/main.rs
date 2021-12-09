use std::collections::{HashMap, HashSet};

type Point = (i32, i32);

fn main() {
    let input = include_str!("../input").trim();

    let mut map: HashMap<Point, u32> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, height) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), height.to_digit(10).unwrap());
        }
    }

    let lows: Vec<(Point, u32)> = map
        .iter()
        .filter(|(&pos, &h)| neighbors(&map, pos).all(|(_, n)| h < n))
        .map(|(&p, &h)| (p, h))
        .collect();

    let part_1: u32 = lows.iter().map(|(_, h)| h + 1).sum();

    let mut basins: Vec<usize> = lows.iter().map(|&(p, _)| basin_size(&map, p)).collect();
    basins.sort_unstable();

    let part_2: usize = basins.iter().rev().take(3).product();

    println!("Part 1: {}\nPart 2: {}", part_1, part_2);
}

fn neighbors<T: Copy>(
    map: &HashMap<Point, T>,
    (x, y): Point,
) -> impl Iterator<Item = (Point, T)> + '_ {
    [(0, 1), (1, 0), (-1, 0), (0, -1)]
        .into_iter()
        .filter_map(move |(dx, dy)| {
            let n = (x + dx, y + dy);
            map.get(&n).map(|&t| (n, t))
        })
}

fn basin_size(map: &HashMap<Point, u32>, start: Point) -> usize {
    let mut stack = Vec::new();
    let mut seen = HashSet::new();
    stack.push(start);
    seen.insert(start);

    while let Some(next) = stack.pop() {
        for (n, h) in neighbors(map, next) {
            if h != 9 && !seen.contains(&n) {
                seen.insert(n);
                stack.push(n);
            }
        }
    }

    seen.len()
}
