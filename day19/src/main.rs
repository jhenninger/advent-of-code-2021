use std::{
    cmp,
    collections::{HashSet, VecDeque},
    mem,
};

fn main() {
    let input = include_str!("../input").trim();
    let mut scanners = VecDeque::new();

    for s in input.split("\n\n") {
        let mut scanner = Scanner {
            beacons: HashSet::new(),
        };
        for l in s.lines().skip(1) {
            let mut coords = l.split(',').map(|x| x.parse().unwrap());
            let beacon = Beacon(
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            );
            scanner.beacons.insert(beacon);
        }

        scanners.push_back(scanner);
    }

    let mut positions = vec![(0, 0, 0)];
    let mut zero = scanners.pop_front().unwrap();

    // pick any scanner as absolute zero
    // for this scanner, check with other scanners
    // look at a pair of points of both scanners, assume they are the same
    // this gets us the assumed pos of scanner 2
    // using these coords, transform the points in scanner 2
    // check if at least 12 are the same

    'outer: while let Some(mut other) = scanners.pop_front() {
        for z in &zero.beacons {
            for i in 0..24 {
                for o in &other.beacons {
                    // o and z is our pair of points

                    // this is the assumed pos of the other scanner
                    let offset = (z.0 - o.0, z.1 - o.1, z.2 - o.2);

                    let transformed: HashSet<_> = other
                        .beacons
                        .iter()
                        .map(|b| Beacon(offset.0 + b.0, offset.1 + b.1, offset.2 + b.2))
                        .collect();

                    let intersection = zero.beacons.intersection(&transformed).count();

                    if intersection >= 12 {
                        // add all transformed beacons to zero
                        // we are done with other
                        for b in transformed {
                            zero.beacons.insert(b);
                        }
                        positions.push(offset);
                        continue 'outer;
                    }
                }
                other.rotate(i + 1);
            }
        }
        // if we arrive here, we have not found a match, so we add other back to the queue
        scanners.push_back(other);
    }

    dbg!(zero.beacons.len());

    let mut max = 0;
    for a in &positions {
        for b in &positions {
            let dist = (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs();
            max = cmp::max(dist, max)
        }
    }

    dbg!(max);
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Beacon(i32, i32, i32);

impl Beacon {
    fn rotate(&mut self, i: usize) {
        // this sucks and should be possible without i

        // rotate around axis 0
        mem::swap(&mut self.1, &mut self.2);
        self.2 *= -1;

        if i % 8 == 0 {
            // rotate around axis 2
            mem::swap(&mut self.0, &mut self.1);
            self.0 *= -1;
        } else if i % 4 == 0 {
            // rotate around axis 1
            mem::swap(&mut self.0, &mut self.2);
            self.2 *= -1;
        }
    }
}

#[derive(Debug)]
struct Scanner {
    beacons: HashSet<Beacon>,
}

impl Scanner {
    fn rotate(&mut self, i: usize) {
        let mut next = HashSet::new();
        for mut beacon in self.beacons.drain() {
            beacon.rotate(i);
            next.insert(beacon);
        }
        self.beacons = next;
    }
}
