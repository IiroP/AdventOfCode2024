use aoc2024::common::read_input;
use itertools::Itertools;

const DAY: u32 = 7;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<(i64, Vec<i64>)> {
    input
        .iter()
        .map(|row| {
            let mut splitted = row.split(": ");
            let target = splitted.next().unwrap().parse::<i64>().unwrap();
            let values = splitted
                .next()
                .unwrap()
                .split(" ")
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (target, values)
        })
        .collect()
}

// Parse sample input
fn _sample_input() -> Vec<(i64, Vec<i64>)> {
    let data = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<(i64, Vec<i64>)> {
    process_input(read_input(DAY))
}

fn total_calibration_result(input: &Vec<(i64, Vec<i64>)>, operations: Vec<&str>) -> i64 {
    input
        .iter()
        .filter_map(|(target, values)| {
            (0..values.len())
                .map(|_| &operations)
                .multi_cartesian_product()
                .any(|i| {
                    let mut iter = i.iter();
                    let result = values
                        .iter()
                        .copied()
                        .reduce(|total, next| match **iter.next().unwrap() {
                            "add" => total + next,
                            "mul" => total * next,
                            "cat" => format!("{total}{next}").parse().unwrap(),
                            _ => panic!("Unknown operation"),
                        })
                        .unwrap();
                    result == *target
                })
                .then(|| target)
        })
        .sum()
}

fn part1(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    let operations = vec!["add", "mul"];
    total_calibration_result(input, operations)
}

// Slow (~40s with dev profile, ~8s with release profile) but works
fn part2(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    let operations = vec!["add", "mul", "cat"];
    total_calibration_result(input, operations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(3749, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(11387, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(2654749936343, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(124060392153684, part2(&input));
    }
}
