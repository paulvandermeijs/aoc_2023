use std::io;

fn main() {
    let lines = io::stdin()
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()
        .unwrap();

    let sum: u32 = lines.iter().map(find_digits).sum();

    println!("{sum}");
}

fn find_digits(line: &String) -> u32 {
    let first = line.chars().find(|char| char.is_numeric()).unwrap();
    let last = line.chars().rfind(|char| char.is_numeric()).unwrap();

    let result = format!("{first}{last}").parse::<u32>().unwrap();

    result
}
