use aoc2024::common::read_input;
use regex::Regex;

const DAY: u32 = 3;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse sample input
fn _sample_input() -> String {
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_owned()
}

// Parse day's input
fn day_input() -> String {
    process_input(&read_input(DAY))
}

// Join the input into a single string
fn process_input(input: &Vec<String>) -> String {
    input.join("")
}

// Part 1
fn part1(input: &String) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            let a: i64 = caps[1].parse().unwrap();
            let b: i64 = caps[2].parse().unwrap();
            a * b
        })
        .sum::<i64>()
}

// Part 2: The trick was that newlines don't matter
fn part2(input: &String) -> i64 {
    // start: match everything until the first "don't" or "do" (or end of line)
    let start = Regex::new(r"^(.+?)(?:(?:don't|do)\(\)|$)").unwrap();
    // mid: match everything between "do" and "don't" (or end of line)
    let mid = Regex::new(r"do\(\)(.+?)(?:don't\(\)|$)").unwrap();
    // mul: match the "mul" commands
    let mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Handle beginning of string separately
    [&start, &mid]
        .iter()
        .map(|re| {
            // Iterate over the captures of the regex ("enabled commands")
            re.captures_iter(input)
                .map(|caps| {
                    // Iterate over the captured "mul" commands
                    mul.captures_iter(&caps[1])
                        .map(|c| {
                            let a: i64 = c[1].parse().unwrap();
                            let b: i64 = c[2].parse().unwrap();
                            a * b
                        })
                        .sum::<i64>() // sum of all "mul" commands inside a substring
                })
                .sum::<i64>() // sum of all substrings
        })
        .sum::<i64>() // sum of all parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(161, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_owned();
        let input2 =
            "mul(1,1)..do()mul(1,2)..do()...mul(1,3)..don't()..mul(1,4)..do()..mul(1,5)".to_owned();
        let input3 = _sample_input();
        assert_eq!(48, part2(&input));
        assert_eq!(11, part2(&input2));
        assert_eq!(161, part2(&input3));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(196826776, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(106780429, part2(&input));
    }
}
