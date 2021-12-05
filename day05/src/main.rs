use std::{iter::successors, collections::HashMap};

struct Line {
    p0: (i32, i32),
    p1: (i32, i32),
}

impl Line {
    fn is_diagonal(&self) -> bool {
        self.p0.0 != self.p1.0 && self.p0.1 != self.p1.1
    }

    fn points(&self) -> impl Iterator<Item = (i32, i32)> {
        let step_x = (self.p1.0 - self.p0.0).signum();
        let step_y = (self.p1.1 - self.p0.1).signum();
        let end_x = self.p1.0;
        let end_y = self.p1.1;
        successors(Some((self.p0.0, self.p0.1)), move |&(x, y)| {
            if x == end_x && y == end_y {
                None
            } else {
                Some((x + step_x, y + step_y))
            }
        })
    }
}

fn main() {
    let input = include_str!("../input");

    let (diagonal, lateral): (Vec<Line>, Vec<Line>) = input
        .lines()
        .map(|l| {
            let mut points = l
                .split(" -> ")
                .flat_map(|p| p.split(",").map(|n| n.parse().unwrap()));
            Line {
                p0: (points.next().unwrap(), points.next().unwrap()),
                p1: (points.next().unwrap(), points.next().unwrap()),
            }
        })
        .partition(|l| l.is_diagonal());

    let mut grid: HashMap<(i32, i32), usize> = HashMap::new();

    for p in lateral.iter().flat_map(|l| l.points()) {
        *grid.entry(p).or_default() += 1;
    }

    let part_1 = grid.values().filter(|&&c| c >= 2).count();

    for p in diagonal.iter().flat_map(|l| l.points()) {
        *grid.entry(p).or_default() += 1;
    }

    let part_2 = grid.values().filter(|&&c| c >= 2).count();

    println!("Part 1: {}\nPart 2: {}", part_1, part_2);
}
