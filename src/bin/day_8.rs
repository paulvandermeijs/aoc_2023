use std::collections::HashMap;

fn main() {
    let mut lines = std::io::stdin().lines();
    let instructions = lines.next().unwrap().unwrap();
    let instructions = instructions.chars();

    lines.next();

    let (map, current) = lines.map(|line| line.unwrap()).fold(
        (
            HashMap::<String, (String, String)>::new(),
            Vec::<(String, String)>::new(),
        ),
        |(mut n, mut c), line| {
            n.insert(line[0..3].into(), (line[7..10].into(), line[12..15].into()));

            if 'A' == line.as_bytes()[2] as char {
                c.push((line[7..10].into(), line[12..15].into()));
            }

            (n, c)
        },
    );

    let result = current
        .into_iter()
        .map(|mut current| {
            let mut result = 1usize;

            for instruction in instructions.clone().into_iter().cycle() {
                let node = match instruction {
                    'L' => &current.0,
                    'R' => &current.1,
                    _ => panic!("Unknown instruction"),
                };

                if 'Z' == node.as_bytes()[2] as char {
                    return result;
                }

                current = map.get(node).unwrap().clone();
                result += 1;
            }

            0
        })
        .collect::<Vec<usize>>();
    let result = result
        .into_iter()
        .reduce(|c, v| num::integer::lcm(c, v))
        .unwrap();

    println!("{:?}", result);
}
