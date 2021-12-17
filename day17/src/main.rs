use std::ops::RangeInclusive;

struct Area {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

impl Area {
    fn part_1(&self) -> i32 {
        self.y.start() * (self.y.start() + 1) / 2
    }

    fn part_2(&self) -> i32 {
        let mut count = 0;

        for vx in 1..=*self.x.end() {
            for vy in *self.y.start()..=self.y.start().abs() {
                if self.hits(vx, vy) {
                    count += 1;
                }
            }
        }

        count
    }

    fn hits(&self, mut vx: i32, mut vy: i32) -> bool {
        let mut x = 0;
        let mut y = 0;

        while x <= *self.x.end() && y >= *self.y.start() {
            x += vx;
            y += vy;
            vx -= vx.signum();
            vy -= 1;

            if self.x.contains(&x) && self.y.contains(&y) {
                return true;
            }
        }

        false
    }
}

fn main() {
    let area = Area {
        x: 288..=330,
        y: -96..=-50,
    };

    println!("Part 1: {}\nPart 2: {}", area.part_1(), area.part_2());
}
