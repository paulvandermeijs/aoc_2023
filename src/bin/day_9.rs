fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_ascii_whitespace()
                .map(|v| v.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();

    fn diff(values: &Vec<isize>) -> Vec<isize> {
        let mut differences = Vec::new();

        for i in 1..values.len() {
            differences.push(values[i] - values[i - 1]);
        }

        differences
    }

    fn extend(mut line: Vec<isize>) -> Vec<isize> {
        match line.last() {
            Some(0) => {
                line.push(0);
                line
            }
            _ => {
                line.push(extend(diff(&line)).last().unwrap() + line.last().unwrap());
                line
            }
        }
    }

    let result = lines
        .into_iter()
        .map(|line| *extend(line).last().unwrap())
        .sum::<isize>();

    println!("{:?}", result);
}
