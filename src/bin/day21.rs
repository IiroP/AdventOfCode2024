use std::{collections::HashMap, vec};

use aoc2024::common::read_input;
use itertools::Itertools;

const DAY: u32 = 21;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<String> {
    input
}

fn possible_paths() -> HashMap<(char, char), Vec<String>> {
    let others = "123456789".chars().collect::<Vec<char>>();
    let mut paths: HashMap<(char, char), Vec<String>> = HashMap::new();
    for i in others.iter() {
        for j in others.iter() {
            let start_col = (i.to_digit(10).unwrap() + 2) % 3;
            let start_row = (i.to_digit(10).unwrap() - 1) / 3;
            let end_col = (j.to_digit(10).unwrap() + 2) % 3;
            let end_row = (j.to_digit(10).unwrap() - 1) / 3;
            let sideways = if start_col > end_col {
                "<".repeat((start_col - end_col).try_into().unwrap())
            } else {
                ">".repeat((end_col - start_col).try_into().unwrap())
            };
            let verticals = if start_row > end_row {
                "v".repeat((start_row - end_row).try_into().unwrap())
            } else {
                "^".repeat((end_row - start_row).try_into().unwrap())
            };
            let path = format!("{}{}", sideways, verticals);
            paths.insert(
                (*i, *j),
                path.chars()
                    .permutations(path.len())
                    .unique()
                    .map(|a| format!("{}A", a.iter().collect::<String>()))
                    .collect(),
            );
        }
    }
    // 0 is always accessed from A or 2
    for j in others.iter() {
        let from_2 = paths.get(&(*j, '2')).unwrap().iter().map(|s| {
            let mut s1 = s.clone();
            s1.pop();
            format!("{}vA", s1)
        });
        let from_a = paths.get(&(*j, '3')).unwrap().iter().map(|s| {
            let mut s1 = s.clone();
            s1.pop();
            format!("{}v<A", s1)
        });
        let combined: Vec<String> = from_2.chain(from_a).unique().collect();
        paths.insert((*j, '0'), combined);
    }
    paths.insert(('A', '0'), vec!["<A".to_string()]);
    for j in others.iter() {
        let from_2 = paths
            .get(&('2', *j))
            .unwrap()
            .iter()
            .map(|s| format!("^{s}"));
        let from_a = paths
            .get(&('3', *j))
            .unwrap()
            .iter()
            .map(|s| format!(">^{s}"));
        let combined: Vec<String> = from_2.chain(from_a).unique().collect();
        paths.insert(('0', *j), combined);
    }

    // A is always accessed from 0 or A
    for j in others.iter() {
        let from_0 = paths.get(&(*j, '0')).unwrap().iter().map(|s| {
            let mut s1 = s.clone();
            s1.pop();
            format!("{}>A", s1)
        });
        let from_3 = paths.get(&(*j, '3')).unwrap().iter().map(|s| {
            let mut s1 = s.clone();
            s1.pop();
            format!("{}vA", s1)
        });
        let combined: Vec<String> = from_0.chain(from_3).unique().collect();
        paths.insert((*j, 'A'), combined);
    }
    paths.insert(('0', 'A'), vec![">A".to_string()]);
    for j in others.iter() {
        let from_0 = paths
            .get(&('0', *j))
            .unwrap()
            .iter()
            .map(|s| format!("<{s}"));
        let from_3 = paths
            .get(&('3', *j))
            .unwrap()
            .iter()
            .map(|s| format!("^{s}"));
        let combined: Vec<String> = from_0.chain(from_3).unique().collect();
        paths.insert(('A', *j), combined);
    }

    // Filter out too long paths
    for i in "0123456789A".chars() {
        paths.insert((i, i), vec!["A".to_string()]);
        for j in "0123456789A".chars() {
            let start_col = if i == 'A' {
                2
            } else if i == '0' {
                1
            } else {
                (i.to_digit(10).unwrap() + 2) % 3
            };
            let start_row = if i == 'A' || i == '0' {
                0
            } else {
                (i.to_digit(10).unwrap() - 1) / 3 + 1
            };
            let end_col = if j == 'A' {
                2
            } else if j == '0' {
                1
            } else {
                (j.to_digit(10).unwrap() + 2) % 3
            };
            let end_row = if j == 'A' || j == '0' {
                0
            } else {
                (j.to_digit(10).unwrap() - 1) / 3 + 1
            };
            let steps = (start_col as i32 - end_col as i32).abs()
                + (start_row as i32 - end_row as i32).abs()
                + 1;
            let entry = paths.get_mut(&(i, j)).unwrap();
            entry.retain(|p| p.len() as i32 == steps);
        }
    }

    paths
}

fn possible_paths2() -> HashMap<(char, char), Vec<String>> {
    let paths: HashMap<(char, char), Vec<String>> = HashMap::from([
        // A
        (('A', 'A'), vec!["A".to_string()]),
        (('A', '^'), vec!["<A".to_string()]),
        (('A', '>'), vec!["vA".to_string()]),
        (('A', 'v'), vec!["v<A".to_string(), "<vA".to_string()]),
        (('A', '<'), vec!["v<<A".to_string(), "<v<A".to_string()]),
        // ^
        (('^', 'A'), vec![">A".to_string()]),
        (('^', '^'), vec!["A".to_string()]),
        (('^', '>'), vec![">vA".to_string(), "v>A".to_string()]),
        (('^', 'v'), vec!["vA".to_string()]),
        (('^', '<'), vec!["v<A".to_string()]),
        // <
        (('<', 'A'), vec![">>^A".to_string(), ">^>A".to_string()]),
        (('<', '^'), vec![">^A".to_string()]),
        (('<', '>'), vec![">>A".to_string()]),
        (('<', 'v'), vec![">A".to_string()]),
        (('<', '<'), vec!["A".to_string()]),
        // v
        (('v', 'A'), vec!["^>A".to_string(), ">^A".to_string()]),
        (('v', '^'), vec!["^A".to_string()]),
        (('v', '>'), vec![">A".to_string()]),
        (('v', 'v'), vec!["A".to_string()]),
        (('v', '<'), vec!["<A".to_string()]),
        // <
        (('>', 'A'), vec!["^A".to_string()]),
        (('>', '^'), vec!["<^A".to_string(), "^<A".to_string()]),
        (('>', '>'), vec!["A".to_string()]),
        (('>', 'v'), vec!["<A".to_string()]),
        (('>', '<'), vec!["<<A".to_string()]),
    ]);
    paths
}
// Parse sample input
fn _sample_input() -> Vec<String> {
    let data = "029A
980A
179A
456A
379A"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<String> {
    process_input(read_input(DAY))
}

fn process_line(
    line: &String,
    start: char,
    paths: &HashMap<(char, char), Vec<String>>,
) -> Vec<String> {
    let mut parts = vec![];
    let mut prev = start;
    for c in line.chars() {
        let p = paths.get(&(prev, c)).unwrap();
        parts.push(p);
        prev = c;
    }
    let choices: Vec<String> = parts
        .iter()
        .map(|v| v.iter())
        .multi_cartesian_product()
        .map(|p| p.into_iter().cloned().collect::<String>())
        .collect();
    choices
}

fn part1(input: &Vec<String>) -> u64 {
    let paths = possible_paths();
    let paths2 = possible_paths2();
    input
        .iter()
        .map(|line| {
            let number = line.split_at(3).0.parse::<u64>().unwrap();
            let choices = process_line(line, 'A', &paths);
            //println!("Choices: {:?}", choices);
            let len = choices
                .iter()
                .map(|choice| {
                    let choices2 = process_line(choice, 'A', &paths2);
                    //println!("Final choices: {:?}", final_choices);
                    choices2
                        .iter()
                        .map(|choice2| {
                            let final_choices = process_line(choice2, 'A', &paths2);
                            final_choices.iter().map(|v| v.len()).min().unwrap()
                        })
                        .min()
                        .unwrap()
                })
                .min()
                .unwrap();
            number * (len as u64)
        })
        .sum()
}

fn part2(_input: &Vec<String>) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(126384, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(0, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(242484, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(0, part2(&input));
    }
}

// 255544 too high
