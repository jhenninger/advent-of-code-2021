use std::collections::{HashMap, HashSet};

type Connections<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn main() {
    let input = include_str!("../input");

    let mut connections: Connections = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        connections.entry(a).or_default().insert(b);
        connections.entry(b).or_default().insert(a);
    }

    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&connections, HashSet::new(), "start"),
        part_2(&connections, HashSet::new(), "start", true)
    );
}

fn part_1<'a>(
    connections: &'a Connections,
    mut visited: HashSet<&'a str>,
    current: &'a str,
) -> usize {
    if current == "end" {
        return 1;
    }

    if is_small(current) && !visited.insert(current) {
        return 0;
    }

    connections[current]
        .iter()
        .map(|&next| part_1(connections, visited.clone(), next))
        .sum()
}

fn part_2<'a>(
    connections: &'a Connections,
    mut visited: HashSet<&'a str>,
    current: &'a str,
    duplicate_allowed: bool,
) -> usize {
    if current == "end" {
        return 1;
    }

    let duplicate_required = is_small(current) && !visited.insert(current);

    if duplicate_required && !duplicate_allowed {
        return 0;
    }

    connections[current]
        .iter()
        .filter(|&&next| next != "start")
        .map(|&next| {
            part_2(
                connections,
                visited.clone(),
                next,
                !duplicate_required && duplicate_allowed,
            )
        })
        .sum()
}

fn is_small(name: &str) -> bool {
    name.starts_with(char::is_lowercase)
}
