use std::io;

fn main() {
    let schematic = io::stdin().lines().fold(Vec::new(), |mut result, line| {
        result.push(line.unwrap().chars().collect::<Vec<char>>());

        result
    });

    let mut buffer: Vec<&char> = Vec::new();
    let mut parts: Vec<String> = Vec::new();
    let mut include: bool = false;

    for (y, row) in schematic.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value.is_numeric() {
                buffer.push(value);
                include = include || check_include(&schematic, (x, y));
            } else if buffer.len() > 0 {
                if include {
                    parts.push(buffer.iter().map(|v| v.to_string()).collect::<String>());
                }

                buffer.clear();
                include = false;
            }
        }

        if include {
            parts.push(buffer.iter().map(|v| v.to_string()).collect::<String>());
        }

        buffer.clear();
        include = false;
    }

    let result: u32 = parts.iter().map(|part| part.parse::<u32>().unwrap()).sum();

    println!("{result}");
}

fn check_include(schematic: &Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
    for oy in y as isize - 1..=y as isize + 1 {
        for ox in x as isize - 1..=x as isize + 1 {
            if ox == x as isize && oy == y as isize {
                continue;
            }

            if let Some(row) = schematic.get(oy as usize) {
                if let Some(value) = row.get(ox as usize) {
                    if '.' != *value && !value.is_numeric() {
                        return true;
                    }
                }
            }
        }
    }

    false
}
