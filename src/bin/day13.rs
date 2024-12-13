use ndarray::prelude::*;
use ndarray_linalg::Solve;

use aoc2024::common::read_input;
use regex::Regex;

const DAY: u32 = 13;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

#[derive(Debug)]
struct Machine {
    a_x: u64,
    a_y: u64,
    b_x: u64,
    b_y: u64,
    prize_x: u64,
    prize_y: u64,
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<Machine> {
    let button_a_regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let button_b_regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut result: Vec<Machine> = Vec::new();
    let mut current = Machine {
        a_x: 0,
        a_y: 0,
        b_x: 0,
        b_y: 0,
        prize_x: 0,
        prize_y: 0,
    };

    for line in &input {
        if let Some(caps) = button_a_regex.captures(&line) {
            current.a_x = caps.get(1).unwrap().as_str().parse().unwrap();
            current.a_y = caps.get(2).unwrap().as_str().parse().unwrap();
        } else if let Some(caps) = button_b_regex.captures(&line) {
            current.b_x = caps.get(1).unwrap().as_str().parse().unwrap();
            current.b_y = caps.get(2).unwrap().as_str().parse().unwrap();
        } else if let Some(caps) = prize_regex.captures(&line) {
            current.prize_x = caps.get(1).unwrap().as_str().parse().unwrap();
            current.prize_y = caps.get(2).unwrap().as_str().parse().unwrap();
            result.push(current);
            current = Machine {
                a_x: 0,
                a_y: 0,
                b_x: 0,
                b_y: 0,
                prize_x: 0,
                prize_y: 0,
            };
        }
    }
    result
}

// Parse sample input
fn _sample_input() -> Vec<Machine> {
    let data = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<Machine> {
    process_input(read_input(DAY))
}

fn machine_tokens(machine: &Machine, part2: bool) -> u64 {
    let a: Array2<f64> = array![
        [machine.a_x as f64, machine.b_x as f64],
        [machine.a_y as f64, machine.b_y as f64]
    ];
    let part2_extra: u64 = 10000000000000;
    let (target_x, target_y) = if part2 {
        (machine.prize_x + part2_extra, machine.prize_y + part2_extra)
    } else {
        (machine.prize_x, machine.prize_y)
    };

    let b: Array1<f64> = array![target_x as f64, target_y as f64];
    let x = a.solve_into(b).unwrap();

    let result_x = (x[0].round() as u64) * machine.a_x + (x[1].round() as u64) * machine.b_x;
    let result_y = (x[0].round() as u64) * machine.a_y + (x[1].round() as u64) * machine.b_y;

    if result_x == target_x && result_y == target_y {
        return 3 * x[0].round() as u64 + x[1].round() as u64;
    }
    0
}

fn part1(input: &Vec<Machine>) -> u64 {
    input.iter().map(|m| machine_tokens(m, false)).sum::<u64>()
}

fn part2(input: &Vec<Machine>) -> u64 {
    input.iter().map(|m| machine_tokens(m, true)).sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(480, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        println!("{:?}", part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(32067, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(92871736253789, part2(&input));
    }
}
