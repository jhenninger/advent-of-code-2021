fn main() {
    let mut lines = include_str!("../input").lines().peekable();
    let bits = lines.peek().unwrap().len();
    let numbers: Vec<_> = lines.map(|l| u32::from_str_radix(l, 2).unwrap()).collect();

    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&numbers, bits),
        part_2(&numbers, bits),
    );
}

fn part_1(numbers: &[u32], bits: usize) -> u32 {
    let gamma = (0..bits).fold(0, |acc, pos| acc | most_common_bit(numbers, pos) << pos);
    gamma * (!gamma & (1 << bits) - 1)
}

fn part_2(numbers: &[u32], bits: usize) -> u32 {
    let oxygen = bit_criteria(numbers, bits, false);
    let co2 = bit_criteria(numbers, bits, true);
    oxygen * co2
}

fn most_common_bit(numbers: &[u32], pos: usize) -> u32 {
    let ones = numbers.iter().filter(|&n| n & (1 << pos) != 0).count();
    (ones >= numbers.len() - ones) as u32
}

fn bit_criteria(numbers: &[u32], bits: usize, least: bool) -> u32 {
    let mut numbers = numbers.to_vec();
    let mut pos = bits;
    while numbers.len() != 1 {
        pos -= 1;
        let target_bit = most_common_bit(&numbers, pos) ^ least as u32;
        numbers.retain(|n| (n >> pos) & 1 == target_bit);
    }
    numbers[0]
}
