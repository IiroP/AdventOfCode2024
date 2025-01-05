use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
};

use aoc2024::common::read_input;

const DAY: u32 = 22;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<i64> {
    input.iter().map(|s| s.parse().unwrap()).collect()
}

// Parse sample input
fn _sample_input() -> Vec<i64> {
    let data = "1
10
100
2024"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<i64> {
    process_input(read_input(DAY))
}

fn mix(value: i64, secret: i64) -> i64 {
    value ^ secret
}

fn prune(value: i64) -> i64 {
    value & 0xffffff
}

fn evolve(value: i64) -> i64 {
    let phase1 = prune(mix(value << 6, value));
    let phase2 = prune(mix(phase1 >> 5, phase1));
    prune(mix(phase2 << 11, phase2))
}

fn part1(input: &Vec<i64>) -> i64 {
    input
        .iter()
        .map(|v| {
            let mut value = *v;
            for _ in 0..2000 {
                value = evolve(value);
            }
            value
        })
        .sum()
}

// Returns the appearance of the pattern and the first corresponding price
fn pattern_values(value: i64) -> Vec<(Vec<i64>, i64)> {
    let mut value = value;
    let mut values = vec![value % 10];
    for _ in 0..(2000 - 1) {
        value = evolve(value);
        values.push(value % 10); // last digit
    }

    // Calculate changes
    let changes = values.windows(2).map(|v| v[1] - v[0]).collect::<Vec<i64>>();

    let mut seen = HashSet::new();
    changes
        .windows(4)
        .enumerate()
        .filter_map(|(i, w)| {
            if seen.insert(w.to_vec()) {
                Some((w.to_vec(), values[i + 4]))
            } else {
                None
            }
        })
        .collect::<Vec<(Vec<i64>, i64)>>()
}

fn part2(input: &Vec<i64>) -> i64 {
    let mut result_map = HashMap::new();
    input
        .iter()
        .map(|v| pattern_values(*v))
        .flatten()
        .for_each(|(series, value)| {
            result_map.entry(series).or_insert(0).add_assign(value);
        });
    result_map.values().max().unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helpers() {
        println!("{:?}", 1 << 6);
        println!("{:?}", 32 >> 5);
        println!("{:?}", 1 << 11);
        assert_eq!(37, mix(15, 42));
        assert_eq!(16113920, prune(100000000));
        assert_eq!(15887950, evolve(123));
    }

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(37327623, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = vec![1, 2, 3, 2024];
        assert_eq!(23, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(20506453102, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(2423, part2(&input));
    }
}
