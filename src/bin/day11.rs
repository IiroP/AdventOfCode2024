use std::collections::HashMap;

use aoc2024::common::read_input;

const DAY: u32 = 11;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: String) -> Vec<u64> {
    input.split(" ").map(|num| num.parse().unwrap()).collect()
}

// Parse sample input
fn _sample_input() -> Vec<u64> {
    let data = "125 17".to_string();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<u64> {
    let data = read_input(DAY)[0].clone();
    process_input(data)
}

// Calculate new stone value(s) for a stone
fn update_stone(value: u64) -> Vec<u64> {
    // Rule 0
    if value == 0 {
        return vec![1];
    }

    // Rule 1
    let value_text = value.to_string();
    if value_text.len() % 2 == 0 {
        let splitted = value_text.split_at(value_text.len() / 2);
        return vec![
            splitted.0.parse::<u64>().unwrap(),
            splitted.1.parse::<u64>().unwrap(),
        ];
    }

    vec![value * 2024]
}

// Combine stone counts
fn combine_counts(input: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut map = HashMap::new();

    for (value, count) in input {
        *map.entry(value).or_insert(0) += count;
    }

    map.into_iter().collect()
}

fn part1(input: &Vec<u64>) -> usize {
    let mut current = input.clone();
    for _ in 0..25 {
        current = current.iter().map(|v| update_stone(*v)).flatten().collect();
    }
    current.len()
}

fn part2(input: &Vec<u64>) -> u64 {
    // Store stone values and their count in vector
    let mut current: Vec<(u64, u64)> = input.iter().map(|v| (*v, 1)).collect();

    for _ in 0..75 {
        current = current
            .iter()
            .flat_map(|(value, count)| {
                update_stone(*value)
                    .iter()
                    .map(|v| (*v, *count))
                    .collect::<Vec<_>>()
            })
            .collect();
        current = combine_counts(current);
    }
    current.iter().map(|(_, count)| *count as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(55312, part1(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(207683, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(244782991106220, part2(&input));
    }
}
