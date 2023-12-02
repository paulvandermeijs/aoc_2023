use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    multi::separated_list0,
    sequence::{tuple, Tuple},
    IResult,
};
use std::io;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Debug, Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

trait Power {
    fn power(&self) -> u32;
}

impl Power for Vec<Set> {
    fn power(&self) -> u32 {
        use std::cmp::max;

        let min_set = self.iter().fold(
            Set {
                ..Default::default()
            },
            |result, current| Set {
                red: max(result.red, current.red),
                green: max(result.green, current.green),
                blue: max(result.blue, current.blue),
            },
        );

        min_set.power()
    }
}

impl Power for Set {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn main() {
    let lines = io::stdin()
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()
        .unwrap();

    let result: u32 = lines
        .iter()
        .map(|line| parse_game(line).unwrap().1)
        .map(|game| game.sets.power())
        .sum();

    println!("{result}");
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, (_, _, id, _, _)) = (tag("Game"), space1, digit1, tag(":"), space0).parse(input)?;
    let id = id.parse::<u32>().unwrap();
    let (input, sets) = parse_sets(input)?;

    Ok((input, Game { id, sets }))
}

fn parse_sets(input: &str) -> IResult<&str, Vec<Set>> {
    let result = separated_list0(tuple((tag(";"), space0)), parse_set)(input);

    result
}

fn parse_set(input: &str) -> IResult<&str, Set> {
    let (input, cubes) = separated_list0(tuple((tag(","), space0)), parse_cubes)(input)?;
    let set = cubes.iter().fold(
        Set {
            ..Default::default()
        },
        |mut set, cube| {
            match cube {
                ("red", n) => set.red += n,
                ("green", n) => set.green += n,
                ("blue", n) => set.blue += n,
                (_, _) => (),
            };
            set
        },
    );

    Ok((input, set))
}

fn parse_cubes(input: &str) -> IResult<&str, (&str, u32)> {
    let (input, (amount, _, color)) = alt((
        tuple((digit1, space1, tag("red"))),
        tuple((digit1, space1, tag("green"))),
        tuple((digit1, space1, tag("blue"))),
    ))(input)?;
    let amount = amount.parse::<u32>().unwrap();

    Ok((input, (color, amount)))
}
