use std::collections::HashMap;

fn main() {
    let mut lines = std::io::stdin().lines();
    let instructions = lines.next().unwrap().unwrap();
    let instructions = instructions.chars();

    lines.next();

    let (map, mut current) = lines.map(|line| line.unwrap()).fold(
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

    let mut result = 1;

    for instruction in instructions.into_iter().cycle() {
        let (nodes, end) = current
            .iter()
            .fold((Vec::new(), true), |(mut nodes, end), current| {
                let node = match instruction {
                    'L' => &current.0,
                    'R' => &current.1,
                    _ => panic!("Unknown instruction"),
                };
                nodes.push(node);

                let end = end && 'Z' == node.as_bytes()[2] as char;

                (nodes, end)
            });

        if end {
            break;
        }

        current = nodes
            .into_iter()
            .map(|node| map.get(node).unwrap().clone())
            .collect::<Vec<(String, String)>>();

        result += 1;
    }

    println!("{:?}", result);
}
