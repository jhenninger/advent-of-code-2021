fn main() {
    let mut positions: Vec<i32> = include_str!("../input")
        .trim()
        .split(',')
        .map(|p| p.parse().unwrap())
        .collect();
    positions.sort_unstable();

    let median = positions[positions.len() / 2];
    let part_1: i32 = positions.iter().map(|p| (median - p).abs()).sum();

    let mean = positions.iter().sum::<i32>() / positions.len() as i32;
    let part_2: i32 = (mean..=mean + 1)
        .map(|target| {
            positions
                .iter()
                .map(|p| {
                    let n = (target - p).abs();
                    n * (n + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap();

    println!("Part 1: {}\nPart 2: {}", part_1, part_2);
}
