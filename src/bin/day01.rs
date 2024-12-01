use aoc2024::common::read_input;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day 1, part 1: {}", part1_result);
    let part2_result = part2(&input);
    println!("Day 1, part 2: {}", part2_result);
}

fn process_input(input: Vec<String>) -> (Vec<i64>, Vec<i64>) {
    input
        .iter()
        .map(|l| {
            let mut parts = l.split_whitespace();
            (
                parts.next().unwrap().parse::<i64>().unwrap(),
                parts.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .unzip()
}

fn _sample_input() -> (Vec<i64>, Vec<i64>) {
    let data = vec!["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

fn day_input() -> (Vec<i64>, Vec<i64>) {
    process_input(read_input(1))
}

fn part1(input: &(Vec<i64>, Vec<i64>)) -> i64 {
    let (mut a, mut b) = input.clone();
    a.sort();
    b.sort();
    a.iter().zip(b.iter()).map(|(a, b)| (b - a).abs()).sum()
}

fn part2(input: &(Vec<i64>, Vec<i64>)) -> i64 {
    let (a, b) = input.clone();
    a.into_iter()
        .map(|v| v * b.iter().filter(|x| **x == v).count() as i64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(11, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(31, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(2166959, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(23741109, part2(&input));
    }
}
