use std::collections::HashSet;

use aoc2024::common::read_input;

const DAY: u32 = 10;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<Vec<u32>> {
    input
        .iter()
        .map(|row| row.chars().map(|v| v.to_digit(10).unwrap()).collect())
        .collect()
}

// Parse sample input
fn _sample_input() -> Vec<Vec<u32>> {
    let data = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<Vec<u32>> {
    process_input(read_input(DAY))
}

// Find possible next steps (neighbors with value+1)
fn next_neighbors(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    input: &Vec<Vec<u32>>,
) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let current = input[y][x];
    if current == 9 {
        return result;
    }
    let target = current + 1;
    if x > 0 && input[y][x - 1] == target {
        result.push((x - 1, y));
    }
    if y > 0 && input[y - 1][x] == target {
        result.push((x, y - 1));
    }
    if x < width - 1 && input[y][x + 1] == target {
        result.push((x + 1, y));
    }
    if y < height - 1 && input[y + 1][x] == target {
        result.push((x, y + 1));
    }
    result
}

// Depth-first search
fn dfs(
    v: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    input: &Vec<Vec<u32>>,
    ends: &mut Vec<(usize, usize)>,
) {
    visited.insert(v);
    // If trail end is found
    if input[v.1][v.0] == 9 {
        ends.push(v);
    }
    let (x, y) = v;
    let width = input[0].len();
    let height = input.len();
    next_neighbors(x, y, width, height, input)
        .iter()
        .for_each(|edge| {
            if !visited.contains(edge) {
                dfs(*edge, visited, input, ends);
            }
        });
}

// Depth-first search without visited set (to find all different paths)
fn dfs_part2(v: (usize, usize), input: &Vec<Vec<u32>>, ends: &mut Vec<(usize, usize)>) {
    // If trail end is found
    if input[v.1][v.0] == 9 {
        ends.push(v);
    }
    let (x, y) = v;
    let width = input[0].len();
    let height = input.len();
    next_neighbors(x, y, width, height, input)
        .iter()
        .for_each(|edge| {
            dfs_part2(*edge, input, ends);
        });
}

fn part1(input: &Vec<Vec<u32>>) -> i64 {
    let mut found = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 0 {
                let mut visited = HashSet::new();
                let mut ends = Vec::new();
                dfs((x, y), &mut visited, input, &mut ends);
                found += ends.len() as i64;
            }
        }
    }
    found
}

fn part2(input: &Vec<Vec<u32>>) -> i64 {
    let mut found = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 0 {
                let mut ends = Vec::new();
                dfs_part2((x, y), input, &mut ends);
                found += ends.len() as i64;
            }
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(36, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(81, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(566, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(1324, part2(&input));
    }
}
