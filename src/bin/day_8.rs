use std::collections::HashMap;

fn main() {
    let mut lines = std::io::stdin().lines();
    let instructions = lines.next().unwrap().unwrap();
    let instructions = instructions.chars();

    lines.next();

    let map = lines.map(|line| line.unwrap()).fold(
        HashMap::<String, (String, String)>::new(),
        |mut n, line| {
            n.insert(line[0..3].into(), (line[7..10].into(), line[12..15].into()));
            n
        },
    );

    let mut current = map.get("AAA").unwrap();
    let mut result = 1;

    for instruction in instructions.into_iter().cycle() {
        let node = match instruction {
            'L' => &current.0,
            'R' => &current.1,
            _ => panic!("Unknown instruction"),
        };

        if "ZZZ" == node {
            break;
        }

        current = map.get(node).unwrap();
        result += 1;
    }

    println!("{:?}", result);
}
