use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input").trim();

    let mut displays: Vec<(Vec<String>, Vec<String>)> = Vec::new();
    for line in input.lines() {
        let (a, b) = line.split_once(" | ").unwrap();
        displays.push((
            a.split_ascii_whitespace()
                .map(|s| {
                    let mut x: Vec<_> = s.chars().collect();
                    x.sort_unstable();
                    x.iter().collect()
                })
                .collect(),
            b.split_ascii_whitespace()
                .map(|s| {
                    let mut x: Vec<_> = s.chars().collect();
                    x.sort_unstable();
                    x.iter().collect()
                })
                .collect(),
        ));
    }

    let mut sum = 0;

    for d in displays.iter() {

        let mut mapping: HashMap<&str, u8> = HashMap::new();
        let mut stuff: HashSet<String> = d.0.iter().cloned().collect();

        let one = stuff.iter().find(|x| x.len() == 2).unwrap().clone();
        let four = stuff.iter().find(|x| x.len() == 4).unwrap().clone();
        let seven = stuff.iter().find(|x| x.len() == 3).unwrap().clone();
        let eight = stuff.iter().find(|x| x.len() == 7).unwrap().clone();

        stuff.remove(&one);
        stuff.remove(&four);
        stuff.remove(&seven);
        stuff.remove(&eight);

        mapping.insert(&one, 1);
        mapping.insert(&four, 4);
        mapping.insert(&seven, 7);
        mapping.insert(&eight, 8);

        let three = stuff
            .iter()
            .find(|x| x.chars().filter(|&c| !one.contains(c)).count() == 3)
            .unwrap()
            .clone();


        stuff.remove(&three);
        mapping.insert(&three, 3);


        let six = stuff
            .iter()
            .find(|x| x.chars().filter(|&c| !seven.contains(c)).count() == 4)
            .unwrap()
            .clone();

        stuff.remove(&six);
        mapping.insert(&six, 6);

        let zero = stuff
            .iter()
            .find(|x| x.chars().filter(|&c| !three.contains(c)).count() == 2)
            .unwrap()
            .clone();

        stuff.remove(&zero);
        mapping.insert(&zero, 0);

        let two = stuff
            .iter()
            .find(|x| x.chars().filter(|&c| !four.contains(c)).count() == 3)
            .unwrap()
            .clone();

        stuff.remove(&two);
        mapping.insert(&two, 2);

        let five = stuff
            .iter()
            .find(|x| x.chars().filter(|&c| !six.contains(c)).count() == 0)
            .unwrap()
            .clone();

        stuff.remove(&five);
        mapping.insert(&five, 5);

        let nine = stuff.iter().next().unwrap().clone();
        stuff.remove(&nine);
        mapping.insert(&nine, 9);

        assert!(stuff.is_empty());

        let mut digit: i32 = 0;
        for n in &d.1 {
            digit *= 10;
            digit += mapping[&n[..]] as i32;

        }

        sum += digit;
    }

    dbg!(sum);
}
