use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

type Pos = (i32, i32);

fn main() {
    let input = include_str!("../input").trim();

    let map: HashMap<Pos, u32> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c.to_digit(10).unwrap()))
        })
        .collect();

    let target = target_pos(&map);
    let part_1 = dijkstra(&map, target);

    let mut big_map = HashMap::new();

    for ((x, y), v) in &map {
        for xm in 0..5 {
            for ym in 0..5 {
                big_map.insert(
                    (x + xm * (target.0 + 1), y + ym * (target.1 + 1)),
                    (v + (xm + ym) as u32 - 1) % 9 + 1,
                );
            }
        }
    }

    let part_2 = dijkstra(&big_map, target_pos(&big_map));

    println!("Part 1: {}\nPart 2: {}", part_1, part_2);
}

fn target_pos(map: &HashMap<Pos, u32>) -> Pos {
    let max_x = map.keys().map(|k| k.0).max().unwrap();
    let max_y = map.keys().map(|k| k.1).max().unwrap();
    (max_x, max_y)
}

fn dijkstra(map: &HashMap<Pos, u32>, target: Pos) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut dist = HashMap::new();

    // maximum laziness, no custom type necessary :D!
    heap.push((Reverse(0), (0, 0)));

    while let Some((Reverse(cost), pos)) = heap.pop() {
        if pos == target {
            return cost;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next = (pos.0 + dx, pos.1 + dy);
            if let Some(risk) = map.get(&next) {
                let next_cost = cost + risk;
                if next_cost < *dist.get(&next).unwrap_or(&u32::MAX) {
                    heap.push((Reverse(next_cost), next));
                    dist.insert(next, next_cost);
                }
            }
        }
    }

    unreachable!();
}
