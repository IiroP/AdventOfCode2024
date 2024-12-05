use std::collections::{HashMap, HashSet};

use aoc2024::common::read_input;

const DAY: u32 = 5;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let splitted: Vec<&[String]> = input.split(|v| v == "").collect();

    // Create rules map
    let mut rules_map = HashMap::new();
    splitted[0]
        .iter()
        .map(|value| {
            let parts: Vec<&str> = value.split("|").collect();
            let a = parts[0].parse::<u32>().unwrap();
            let b = parts[1].parse::<u32>().unwrap();
            (a, b)
        })
        .for_each(|(left, right)| rules_map.entry(left).or_insert_with(Vec::new).push(right));

    // Create updates part
    let updates = splitted[1]
        .iter()
        .map(|v| {
            v.split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    (rules_map, updates)
}

// Parse sample input
fn _sample_input() -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let data = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    process_input(read_input(DAY))
}

fn part1(input: &(HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)) -> u64 {
    let (rules, updates) = input;
    updates
        .iter()
        .filter(|u| {
            // Find all valid updates
            let mut seen = HashSet::new();
            u.iter().all(|&num| {
                if let Some(curr_rules) = rules.get(&num) {
                    // If current number has a rule about previously seen number, return false
                    if curr_rules.iter().any(|&rule| seen.contains(&rule)) {
                        return false;
                    }
                }
                seen.insert(num);
                true
            })
        })
        .map(|v| v[v.len() / 2] as u64)
        .sum::<u64>()
}

// Uses topological sort (DFS) from https://en.wikipedia.org/wiki/Topological_sorting#Depth-first_search
fn part2(input: &(HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)) -> u64 {
    let (rules, updates) = input;

    fn visit(
        n: &u32,
        rules: &HashMap<u32, Vec<u32>>,
        visited: &mut HashSet<u32>,
        remaining: &mut HashSet<u32>,
        new: &mut Vec<u32>,
    ) {
        if !remaining.contains(&n) {
            // "If n has a permanent mark"
            return;
        } else if visited.contains(&n) {
            // "If n has a temporary mark"
            return;
        }
        visited.insert(*n);
        if let Some(curr_rules) = rules.get(n) {
            curr_rules
                .iter()
                .for_each(|value| visit(value, rules, visited, remaining, new));
        }
        remaining.remove(&n);
        new.push(*n);
    }

    updates
        .iter()
        .filter(|u| {
            // Find all invalid updates
            let mut seen = HashSet::new();
            u.iter().any(|&num| {
                if let Some(curr_rules) = rules.get(&num) {
                    if curr_rules.iter().any(|&rule| seen.contains(&rule)) {
                        return true;
                    }
                }
                seen.insert(num);
                false
            })
        })
        .map(|v| {
            // Construct the new order using topological sort
            // The order is reversed, but we only need the middle element :)
            let mut new: Vec<u32> = vec![];
            let mut remaining: HashSet<u32> = HashSet::from_iter(v.iter().cloned());
            let mut visited: HashSet<u32> = HashSet::new();

            while !remaining.is_empty() {
                let n = *remaining.iter().next().unwrap();
                visit(&n, rules, &mut visited, &mut remaining, &mut new);
            }
            new
        })
        .map(|v| v[v.len() / 2] as u64)
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(143, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(123, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(7198, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(4230, part2(&input));
    }
}
