use std::cmp;

fn main() {
    let input = include_str!("../input").trim();

    let mut grid = Grid::new();

    for line in input.lines() {
        let mut parts = line.split([' ', ','].as_slice());

        let instr = parts.next().unwrap();

        let mut ranges = parts.map(|p| {
            let (from, to) = p[2..].split_once("..").unwrap();
            (from.parse().unwrap(), to.parse::<i32>().unwrap() + 1)
        });

        let cube = Cube {
            x: ranges.next().unwrap(),
            y: ranges.next().unwrap(),
            z: ranges.next().unwrap(),
        };

        if instr == "on" {
            grid.add(cube);
        } else {
            grid.remove(&cube);
        }
    }

    let init_region = Cube {
        x: (-50, 51),
        y: (-50, 51),
        z: (-50, 51),
    };

    println!(
        "Part 1: {}\nPart 2: {}",
        grid.intersection(&init_region).len(),
        grid.len(),
    );
}

struct Grid {
    cubes: Vec<Cube>,
}

impl Grid {
    fn new() -> Self {
        Grid { cubes: Vec::new() }
    }

    fn add(&mut self, cube: Cube) {
        self.remove(&cube);
        self.cubes.push(cube);
    }

    fn remove(&mut self, cube: &Cube) {
        self.cubes = self.cubes.drain(..).flat_map(|c| c.clip(cube)).collect();
    }

    fn intersection(&mut self, cube: &Cube) -> Self {
        Grid {
            cubes: self
                .cubes
                .iter()
                .filter_map(|c| c.intersection(&cube))
                .collect(),
        }
    }

    fn len(&self) -> usize {
        self.cubes.iter().map(|c| c.len()).sum()
    }
}

struct Cube {
    // end of range is exclusive
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl Cube {
    fn intersection(&self, other: &Self) -> Option<Self> {
        let intersection = Cube {
            x: (cmp::max(self.x.0, other.x.0), cmp::min(self.x.1, other.x.1)),
            y: (cmp::max(self.y.0, other.y.0), cmp::min(self.y.1, other.y.1)),
            z: (cmp::max(self.z.0, other.z.0), cmp::min(self.z.1, other.z.1)),
        };

        match intersection.len() {
            0 => None,
            _ => Some(intersection),
        }
    }

    // removes other from self, returns leftovers of self
    fn clip(self, other: &Self) -> Vec<Self> {
        let i = match self.intersection(other) {
            Some(i) => i,
            None => return vec![self],
        };

        // create 26 cubes that touch the "hole" carved by the intersection, the cube in the very center
        // when seen from above, we create the cubes of each layer in the following order:

        // y
        // | 7 8 9
        // | 4 5 6
        // | 1 2 3
        // +------ x

        // z is the layer axis of the cube, facing towards the camera

        [
            // BOTTOM LAYER
            // 1 corner
            Cube {
                x: (self.x.0, i.x.0),
                y: (self.y.0, i.y.0),
                z: (self.z.0, i.z.0),
            },
            // 2 edge
            Cube {
                x: (i.x.0, i.x.1),
                y: (self.y.0, i.y.0),
                z: (self.z.0, i.z.0),
            },
            // 3 corner
            Cube {
                x: (i.x.1, self.x.1),
                y: (self.y.0, i.y.0),
                z: (self.z.0, i.z.0),
            },
            // 4 edge
            Cube {
                x: (self.x.0, i.x.0),
                y: (i.y.0, i.y.1),
                z: (self.z.0, i.z.0),
            },
            // 5 middle of bottom layer
            Cube {
                x: (i.x.0, i.x.1),
                y: (i.y.0, i.y.1),
                z: (self.z.0, i.z.0),
            },
            // 6 edge
            Cube {
                x: (i.x.1, self.x.1),
                y: (i.y.0, i.y.1),
                z: (self.z.0, i.z.0),
            },
            // 7 corner
            Cube {
                x: (self.x.0, i.x.0),
                y: (i.y.1, self.y.1),
                z: (self.z.0, i.z.0),
            },
            // 8 edge
            Cube {
                x: (i.x.0, i.x.1),
                y: (i.y.1, self.y.1),
                z: (self.z.0, i.z.0),
            },
            // 9 corner
            Cube {
                x: (i.x.1, self.x.1),
                y: (i.y.1, self.y.1),
                z: (self.z.0, i.z.0),
            },
            // MIDDLE LAYER
            // 1 corner
            Cube {
                x: (self.x.0, i.x.0),
                y: (self.y.0, i.y.0),
                z: (i.z.0, i.z.1),
            },
            // 2 edge
            Cube {
                x: (i.x.0, i.x.1),
                y: (self.y.0, i.y.0),
                z: (i.z.0, i.z.1),
            },
            // 3 corner
            Cube {
                x: (i.x.1, self.x.1),
                y: (self.y.0, i.y.0),
                z: (i.z.0, i.z.1),
            },
            // 4 edge
            Cube {
                x: (self.x.0, i.x.0),
                y: (i.y.0, i.y.1),
                z: (i.z.0, i.z.1),
            },
            // 5 middle cube, aka the intersection hole
            // we remove this
            // Cube {
            //     x: (i.x.0, i.x.1),
            //     y: (i.y.0, i.y.1),
            //     z: (i.z.0, i.z.1),
            // },
            // 6 edge
            Cube {
                x: (i.x.1, self.x.1),
                y: (i.y.0, i.y.1),
                z: (i.z.0, i.z.1),
            },
            // 7 corner
            Cube {
                x: (self.x.0, i.x.0),
                y: (i.y.1, self.y.1),
                z: (i.z.0, i.z.1),
            },
            // 8 edge
            Cube {
                x: (i.x.0, i.x.1),
                y: (i.y.1, self.y.1),
                z: (i.z.0, i.z.1),
            },
            // 9 corner
            Cube {
                x: (i.x.1, self.x.1),
                y: (i.y.1, self.y.1),
                z: (i.z.0, i.z.1),
            },
            // TOP LAYER

            // 1 corner
            Cube {
                x: (self.x.0, i.x.0),
                y: (self.y.0, i.y.0),
                z: (i.z.1, self.z.1),
            },
            // 2 edge
            Cube {
                x: (i.x.0, i.x.1),
                y: (self.y.0, i.y.0),
                z: (i.z.1, self.z.1),
            },
            // 3 corner
            Cube {
                x: (i.x.1, self.x.1),
                y: (self.y.0, i.y.0),
                z: (i.z.1, self.z.1),
            },
            // 4 edge
            Cube {
                x: (self.x.0, i.x.0),
                y: (i.y.0, i.y.1),
                z: (i.z.1, self.z.1),
            },
            // 5 middle of top layer
            Cube {
                x: (i.x.0, i.x.1),
                y: (i.y.0, i.y.1),
                z: (i.z.1, self.z.1),
            },
            // 6 edge
            Cube {
                x: (i.x.1, self.x.1),
                y: (i.y.0, i.y.1),
                z: (i.z.1, self.z.1),
            },
            // 7 corner
            Cube {
                x: (self.x.0, i.x.0),
                y: (i.y.1, self.y.1),
                z: (i.z.1, self.z.1),
            },
            // 8 edge
            Cube {
                x: (i.x.0, i.x.1),
                y: (i.y.1, self.y.1),
                z: (i.z.1, self.z.1),
            },
            // 9 corner
            Cube {
                x: (i.x.1, self.x.1),
                y: (i.y.1, self.y.1),
                z: (i.z.1, self.z.1),
            },
        ]
        .into_iter()
        .filter(|c| c.len() != 0)
        .collect()
    }

    fn len(&self) -> usize {
        let x = self.x.1 - self.x.0;
        let y = self.y.1 - self.y.0;
        let z = self.z.1 - self.z.0;

        if x <= 0 || z <= 0 || y <= 0 {
            0
        } else {
            x as usize * y as usize * z as usize
        }
    }
}
