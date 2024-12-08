use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

use aoc2024::common::read_input;
use itertools::Itertools;

const DAY: u32 = 8;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> (usize, usize, HashMap<char, Vec<(usize, usize)>>) {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let width = input[0].len();
    let height = input.len();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_insert(Vec::new()).push((x, y));
            }
        }
    }
    (width, height, antennas)
}

// Parse sample input
fn _sample_input() -> (usize, usize, HashMap<char, Vec<(usize, usize)>>) {
    let data = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> (usize, usize, HashMap<char, Vec<(usize, usize)>>) {
    process_input(read_input(DAY))
}

fn antinodes(antennas: &Vec<(usize, usize)>, width: i32, height: i32) -> HashSet<(usize, usize)> {
    let iter = antennas.iter().combinations(2).flat_map(|pair| {
        let (a, b) = (pair[0], pair[1]);
        let dx = a.0 as i32 - b.0 as i32;
        let dy = a.1 as i32 - b.1 as i32;
        [
            (a.0 as i32 + dx, a.1 as i32 + dy),
            (b.0 as i32 - dx, b.1 as i32 - dy),
        ]
        .into_iter()
        .filter(|(x, y)| x >= &0 && x < &width && y >= &0 && y < &height)
        .map(|(x, y)| (x as usize, y as usize))
        .collect::<Vec<(usize, usize)>>()
    });
    HashSet::from_iter(iter)
}

fn antinodes2(antennas: &Vec<(usize, usize)>, width: i32, height: i32) -> HashSet<(usize, usize)> {
    let iter = antennas.iter().combinations(2).flat_map(|pair| {
        let (a, b) = (pair[0], pair[1]);
        let dx = a.0 as i32 - b.0 as i32;
        let dy = a.1 as i32 - b.1 as i32;

        // Generate points from a, away from b until the edge of the grid
        let a_iter = successors(Some((a.0 as i32, a.1 as i32)), |&(x, y)| {
            Some((x + dx, y + dy))
        })
        .take_while(|&(x, y)| x >= 0 && x < width && y >= 0 && y < height)
        .map(|(x, y)| (x as usize, y as usize));

        // Generate points from b, away from a until the edge of the grid
        let b_iter = successors(Some((b.0 as i32, b.1 as i32)), |&(x, y)| {
            Some((x - dx, y - dy))
        })
        .take_while(|&(x, y)| x >= 0 && x < width && y >= 0 && y < height)
        .map(|(x, y)| (x as usize, y as usize));

        // Combine iterators
        a_iter.chain(b_iter).collect::<Vec<(usize, usize)>>()
    });
    HashSet::from_iter(iter)
}

fn part1(input: &(usize, usize, HashMap<char, Vec<(usize, usize)>>)) -> i64 {
    let (width, height, antennas) = input;
    let mut result = HashSet::new();
    for (_, positions) in antennas {
        result.extend(antinodes(positions, *width as i32, *height as i32));
    }
    result.len() as i64
}

fn part2(input: &(usize, usize, HashMap<char, Vec<(usize, usize)>>)) -> i64 {
    let (width, height, antennas) = input;
    let mut result = HashSet::new();
    for (_, positions) in antennas {
        result.extend(antinodes2(positions, *width as i32, *height as i32));
    }
    result.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(14, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(34, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(398, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(1333, part2(&input));
    }
}
