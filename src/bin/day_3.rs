use std::{collections::HashMap, io};

fn main() {
    let schematic = io::stdin().lines().fold(Vec::new(), |mut result, line| {
        result.push(line.unwrap().chars().collect::<Vec<char>>());

        result
    });

    let mut buffer: Vec<&char> = Vec::new();
    let mut ratios: HashMap<(usize, usize), Vec<String>> = HashMap::new();
    let mut current_gear: Option<(usize, usize)> = None;

    let mut add_ratio = |gear, value| {
        match ratios.get_mut(&gear) {
            Some(r) => r.push(value),
            None => {
                ratios.insert(gear, vec![value]);
            }
        };
    };

    for (y, row) in schematic.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value.is_numeric() {
                buffer.push(value);
                current_gear = current_gear.or_else(|| get_connected_gear(&schematic, (x, y)));
            } else if buffer.len() > 0 {
                if let Some(gear) = current_gear {
                    add_ratio(gear, buffer.clone().into_iter().collect());
                }

                buffer.clear();
                current_gear = None;
            }
        }

        if let Some(gear) = current_gear {
            add_ratio(gear, buffer.clone().into_iter().collect());
        }

        buffer.clear();
        current_gear = None;
    }

    let result: u32 = ratios
        .iter()
        .filter(|(_, r)| r.len() == 2)
        .map(|(_, r)| r[0].parse::<u32>().unwrap() * r[1].parse::<u32>().unwrap())
        .sum();

    println!("{result}");
}

fn get_connected_gear(
    schematic: &Vec<Vec<char>>,
    (x, y): (usize, usize),
) -> Option<(usize, usize)> {
    for oy in y as isize - 1..=y as isize + 1 {
        for ox in x as isize - 1..=x as isize + 1 {
            if ox == x as isize && oy == y as isize {
                continue;
            }

            if let Some(row) = schematic.get(oy as usize) {
                if let Some(value) = row.get(ox as usize) {
                    if '*' == *value {
                        return Some((ox as usize, oy as usize));
                    }
                }
            }
        }
    }

    None
}
