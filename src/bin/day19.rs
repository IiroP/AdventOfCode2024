use std::collections::HashMap;

use aoc2024::common::read_input;

const DAY: u32 = 19;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> (Vec<String>, Vec<String>) {
    let towels: Vec<String> = input[0].split(", ").map(|s| s.to_string()).collect();
    let patterns = input[2..].to_vec();
    (towels, patterns)
}

// Parse sample input
fn _sample_input() -> (Vec<String>, Vec<String>) {
    let data = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> (Vec<String>, Vec<String>) {
    process_input(read_input(DAY))
}

// My original implementation, for history purposes
#[allow(dead_code)]
fn is_valid_original(
    pattern: &String,
    towels: &Vec<String>,
    memory: &mut HashMap<String, bool>,
) -> bool {
    //println!("Checking {pattern}");
    towels.iter().any(|t| {
        if t == pattern {
            return true;
        }
        if memory.contains_key(pattern) {
            //println!("Memory hit for {pattern}, result is {}", memory[pattern]);
            return memory[pattern];
        }

        if pattern.find(t).unwrap_or(usize::MAX) == 0 {
            // Pattern starts with towel t
            let remaining = pattern[t.len()..].to_string();
            let status = is_valid_original(&remaining, towels, memory);
            //println!("Memory miss for {remaining}, result is {}", status);
            memory.insert(remaining, status);
            return status;
        }
        false
    })
}

// Pseudocode from https://stackoverflow.com/a/5996945
fn is_valid(pattern: &String, towels: &Vec<String>, memory: &mut HashMap<String, bool>) -> bool {
    // Base cases
    if pattern.len() == 0 {
        return true;
    }
    if memory.contains_key(pattern) {
        return memory[pattern];
    }

    memory.insert(pattern.clone(), false);
    for t in towels {
        let len = t.len();
        if len > pattern.len() {
            continue;
        }
        let start = pattern[..len].to_string();
        let rest = pattern[len..].to_string();
        if start == *t && is_valid(&rest, towels, memory) {
            memory.insert(pattern.clone(), true);
        }
    }

    memory[pattern]
}

fn calculate_ways(
    pattern: &String,
    towels: &Vec<String>,
    memory: &mut HashMap<String, u64>,
) -> u64 {
    if memory.contains_key(pattern) {
        return memory[pattern];
    }
    if pattern.len() == 0 {
        return 1;
    }
    let mut ways = 0;

    // Recursive case
    for i in towels {
        if pattern.starts_with(i) {
            let remaining = &pattern[i.len()..].to_string();
            ways += calculate_ways(remaining, towels, memory);
        }
    }
    memory.insert(pattern.clone(), ways);
    ways
}

fn part1(input: &(Vec<String>, Vec<String>)) -> i64 {
    let (towels, patterns) = input;
    let mut memory = HashMap::new();
    patterns
        .iter()
        .filter(|p| is_valid(p, towels, &mut memory))
        .count() as i64
}

fn part2(input: &(Vec<String>, Vec<String>)) -> u64 {
    let (towels, patterns) = input;
    let mut memory = HashMap::new();
    patterns
        .iter()
        .map(|p| calculate_ways(p, towels, &mut memory))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(6, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(16, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(322, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(715514563508258, part2(&input));
    }
}
