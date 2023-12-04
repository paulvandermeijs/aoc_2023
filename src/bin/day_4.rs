fn main() {
    let result: u32 = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let (left, right) = l.split_once(':').unwrap().1.split_once('|').unwrap();

            let left = left
                .split(' ')
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let score = right
                .split(' ')
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<u32>().unwrap())
                .filter(|v| left.contains(v))
                .fold(0, |f, _| match f {
                    0 => 1,
                    _ => f * 2,
                });

            score
        })
        .sum();

    println!("{:?}", result);
}
