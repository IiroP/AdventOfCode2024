use aoc2024::common::read_input;

const DAY: u32 = 3;

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

// Parse sample input
fn _sample_input() -> Vec<String> {
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
fn day_input() -> Vec<String> {
    process_input(read_input(DAY))
}

fn part1(input: &Vec<String>) -> i64 {
    0
}

fn part2(input: &Vec<String>) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(0, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(0, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(0, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(0, part2(&input));
    }
}
