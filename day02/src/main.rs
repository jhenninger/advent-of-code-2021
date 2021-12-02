fn main() {
    let input = include_str!("../input");

    let mut x = 0;
    let mut y1 = 0;
    let mut y2 = 0;

    for line in input.lines() {
        let (dir, change) = line.split_once(' ').unwrap();
        let change: i64 = change.parse().unwrap();

        match dir {
            "forward" => {
                x += change;
                y2 += y1 * change;
            }
            "down" => y1 += change,
            "up" => y1 -= change,
            _ => panic!(),
        }
    }

    println!("Part 1: {}\nPart 2: {}", x * y1, x * y2);
}
