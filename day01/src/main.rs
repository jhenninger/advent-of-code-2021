fn main() {
    let depths: Vec<usize> = include_str!("../input")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    println!(
        "Part 1: {}\nPart 2: {}",
        increments(&depths, 1),
        increments(&depths, 3)
    );
}

fn increments(depths: &[usize], window: usize) -> usize {
    depths
        .windows(window + 1)
        .filter(|w| w[0] < w[window])
        .count()
}
