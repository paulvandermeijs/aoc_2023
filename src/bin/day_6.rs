fn main() {
    let races = std::io::stdin()
        .lines()
        .map(|l| {
            l.unwrap().split_ascii_whitespace().collect::<Vec<&str>>()[1..]
                .into_iter()
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let races = std::iter::zip(&races[0], &races[1]).collect::<Vec<(&u32, &u32)>>();

    let result = races
        .into_iter()
        .map(|(time, distance)| {
            (1..*time)
                .into_iter()
                .map(|hold| hold * (time - hold))
                .filter(|travel| travel > distance)
                .count()
        })
        .fold(1, |result, wins| result * wins);

    println!("{:?}", result);
}
