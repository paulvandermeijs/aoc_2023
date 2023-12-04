fn main() {
    let (result, _) = std::io::stdin().lines().map(|l| l.unwrap()).fold(
        (0, Vec::<u32>::new()),
        |(count, copies), l| {
            let inc = 1 + copies.len();
            let (left, right) = l.split_once(':').unwrap().1.split_once('|').unwrap();

            let left = left
                .split(' ')
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let matches = right
                .split(' ')
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<u32>().unwrap())
                .filter(|v| left.contains(v))
                .count();

            let mut copies = copies
                .into_iter()
                .map(|v| v - 1)
                .filter(|v| *v > 0)
                .collect::<Vec<u32>>();

            if matches > 0 {
                for _ in 0..inc {
                    copies.push(matches as u32);
                }
            }

            (count + inc, copies)
        },
    );

    println!("{:?}", result);
}
