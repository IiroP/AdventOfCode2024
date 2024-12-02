use aoc2024::common::read_input;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day 2, part 1: {}", part1_result);
    let part2_result = part2(&input);
    println!("Day 2, part 2: {}", part2_result);
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<Vec<i64>> {
    input
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

// Parse sample input
fn _sample_input() -> Vec<Vec<i64>> {
    let data = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<Vec<i64>> {
    process_input(read_input(2))
}

// Check if two consecutive numbers are valid
fn is_valid(a: i64, b: i64) -> bool {
    a < b && b - a >= 1 && b - a <= 3
}

// Check if the "report" is valid
fn check_report(v: &Vec<i64>) -> bool {
    v.windows(2).all(|w| is_valid(w[0], w[1]))
}

fn part1(input: &Vec<Vec<i64>>) -> i64 {
    let increasing = input
        .iter()
        .filter(|v| v.windows(2).all(|w| is_valid(w[0], w[1])))
        .count() as i64;
    let decreasing = input
        .iter()
        .filter(|v| v.windows(2).all(|w| is_valid(w[1], w[0])))
        .count() as i64;
    //let increasing2 = input.iter().filter(|v| check_report(v)).count() as i64;
    //let decreasing2 = input
    //    .iter()
    //    .filter(|v| check_report(&v.iter().rev().map(|a| *a).collect::<Vec<i64>>()))
    //    .count() as i64;
    increasing + decreasing
}

fn part2(input: &Vec<Vec<i64>>) -> i64 {
    // Check if removing one element makes the report valid
    fn check_removed(v: &Vec<i64>) -> bool {
        for i in 0..v.len() {
            let mut v = v.clone();
            v.remove(i);
            if check_report(&v) {
                return true;
            }
        }
        false
    }
    input
        .iter()
        .map(|v| {
            let reversed = v.iter().rev().map(|a| *a).collect::<Vec<i64>>();
            check_removed(v) || check_removed(&reversed)
        })
        .filter(|a| *a)
        .count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(2, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(4, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(390, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(439, part2(&input));
    }
}
