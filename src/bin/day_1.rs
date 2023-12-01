use std::io;

use regex::{Captures, Regex};

fn main() {
    let day_regex_first = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let day_regex_last = Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let sum: u32 = io::stdin()
        .lines()
        .map(|line| find_digits(&day_regex_first, &day_regex_last, line.unwrap()))
        .sum();

    println!("{sum}");
}

fn find_digits(day_regex_first: &Regex, day_regex_last: &Regex, line: String) -> u32 {
    let replace = |caps: &Captures| match &caps[1] {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => "0",
    };
    let replaced_first_line = day_regex_first.replace(&line, replace);
    let first = replaced_first_line
        .chars()
        .find(|char| char.is_numeric())
        .unwrap();
    let replaced_last_line = day_regex_last.replace(&line, replace);
    let last = replaced_last_line
        .chars()
        .rfind(|char| char.is_numeric())
        .unwrap();

    let result = format!("{first}{last}").parse::<u32>().unwrap();

    result
}
