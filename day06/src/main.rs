fn main() {
    let input = include_str!("../input").trim();
    let mut timers = [0usize; 9];

    for fish in input.split(',') {
        let timer: usize = fish.parse().unwrap();
        timers[timer] += 1;
    }

    println!(
        "Part 1: {}\nPart 2: {}",
        generations(80, &mut timers),
        generations(256 - 80, &mut timers),
    );
}

fn generations(n: usize, timers: &mut [usize]) -> usize {
    for _ in 0..n {
        timers.rotate_left(1);
        timers[6] += timers[8];
    }
    timers.iter().sum()
}
