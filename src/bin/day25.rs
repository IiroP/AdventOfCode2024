use std::collections::HashSet;

use aoc2024::common::read_input;

const DAY: u32 = 25;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> (HashSet<Vec<usize>>, HashSet<Vec<usize>>) {
    let mut locks: HashSet<Vec<usize>> = HashSet::new();
    let mut keys: HashSet<Vec<usize>> = HashSet::new();

    fn parse_one(
        input: &Vec<String>,
        from: usize,
        locks: &mut HashSet<Vec<usize>>,
        keys: &mut HashSet<Vec<usize>>,
    ) {
        let mut counts = vec![0; 5];
        for row in 0..7 {
            for col in 0..5 {
                let c = input[from + row].chars().nth(col).unwrap();
                if c == '#' {
                    counts[col] += 1;
                }
            }
        }

        // Ignore the border
        for i in 0..5 {
            counts[i] -= 1;
        }

        if input[from] == "#####" {
            locks.insert(counts);
        } else {
            keys.insert(counts);
        }
    }
    (0..input.len())
        .step_by(8)
        .for_each(|i| parse_one(&input, i, &mut locks, &mut keys));

    (locks, keys)
}

// Parse sample input
fn _sample_input() -> (HashSet<Vec<usize>>, HashSet<Vec<usize>>) {
    let data = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> (HashSet<Vec<usize>>, HashSet<Vec<usize>>) {
    process_input(read_input(DAY))
}

fn part1(input: &(HashSet<Vec<usize>>, HashSet<Vec<usize>>)) -> usize {
    let (locks, keys) = input;
    locks
        .iter()
        .map(|l| {
            keys.iter()
                .filter(|k| k.iter().zip(l.iter()).all(|(ki, li)| ki + li <= 5))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(3, part1(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(2950, part1(&input));
    }
}
