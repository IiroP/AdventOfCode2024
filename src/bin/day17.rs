use aoc2024::common::read_input;
use itertools::Itertools;
use regex::Regex;

const DAY: u32 = 17;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> (u64, Vec<u64>) {
    let mut a = 0;
    let mut program = Vec::new();
    let program_re = Regex::new(r"Program: ((?:\d|,)+)").unwrap();
    let register_re = Regex::new(r"Register A: (\d+)").unwrap();
    input.iter().for_each(|line| {
        if let Some(caps) = program_re.captures(line) {
            program = caps[1].split(",").flat_map(|s| s.parse::<u64>()).collect();
        }
        if let Some(caps) = register_re.captures(line) {
            a = caps[1].parse::<u64>().unwrap();
        }
    });
    (a, program)
}

// Parse sample input
fn _sample_input() -> (u64, Vec<u64>) {
    let data = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> (u64, Vec<u64>) {
    process_input(read_input(DAY))
}

fn do_operation(
    instr: u64,
    opcode: u64,
    a: &mut u64,
    b: &mut u64,
    c: &mut u64,
    out: &mut Vec<u64>,
) -> Option<usize> {
    fn combo(instr: u64, a: &u64, b: &u64, c: &u64) -> u64 {
        match instr {
            4 => *a,
            5 => *b,
            6 => *c,
            x => x,
        }
    }

    match opcode {
        0 => {
            // adv, Division
            *a = *a / (2 as u64).pow(combo(instr, a, b, c).try_into().unwrap());
        }
        1 => {
            // bxl, Bitwise XOR
            *b = *b ^ instr;
        }
        2 => {
            // bst
            *b = combo(instr, a, b, c) % 8;
        }
        3 => {
            // jnz
            if *a != 0 {
                return Some(instr as usize);
            }
        }
        4 => {
            // bxc
            *b = *b ^ *c;
        }
        5 => {
            // out
            out.push(combo(instr, a, b, c) % 8);
        }
        6 => {
            // bdv
            *b = *a / (2 as u64).pow(combo(instr, a, b, c).try_into().unwrap());
        }
        7 => {
            // cdv
            *c = *a / (2 as u64).pow(combo(instr, a, b, c).try_into().unwrap());
        }
        _ => (),
    }
    None
}

fn simulate(initial_a: u64, program: &Vec<u64>) -> Vec<u64> {
    let mut a = initial_a;
    let mut b = 0;
    let mut c = 0;
    let mut i = 0;
    let max_i = program.len();
    let mut out = Vec::new();

    while i < max_i {
        let opcode = program[i];
        let instr = program[i + 1];
        let result = do_operation(instr, opcode, &mut a, &mut b, &mut c, &mut out);
        //println!("a: {}, b: {}, c: {}, out: {:?}", a, b, c, out);
        //if out.len() > 0 && out[out.len() - 1] != program[out.len() - 1] {
        //    return false;
        //}
        i = result.unwrap_or_else(|| i + 2);
    }
    //println!("Initial A: {:b} ({initial_a}), out: {:?}", initial_a, out);
    //out == *program
    out
}

fn part1(input: &(u64, Vec<u64>)) -> String {
    let (a, program) = input;
    let mut a = *a;
    let mut b = 0;
    let mut c = 0;
    let mut i = 0;
    let max_i = program.len();
    let mut out = Vec::new();

    while i < max_i {
        let opcode = program[i];
        let instr = program[i + 1];
        let result = do_operation(instr, opcode, &mut a, &mut b, &mut c, &mut out);
        i = result.unwrap_or_else(|| i + 2);
    }

    out.iter().join(",")
}

// The key observation is that the program shifts a by 3 bits after each iteration
fn part2(input: &(u64, Vec<u64>)) -> u64 {
    let (_, program) = input;
    let mut current = 0;

    for i in (0..program.len()).rev() {
        //println!("i={i}");
        for a in 0..(1 << 18) {
            // usually 0..8 is enough, but sometimes it's not
            let test_a = (current << 3) + a;
            let result = simulate(test_a, program);
            if result == program[i..] {
                //println!("a = {:b} ({test_a}), result = {:?}", test_a, result);
                current = test_a;
                break;
            }
        }
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(&input));
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        let input = process_input(input);
        assert_eq!(117440, part2(&input));
    }

    #[test]
    fn test_x() {
        let a = 202975183645226;
        let (_, program) = day_input();
        assert_eq!("2,4,1,1,7,5,0,3,1,4,4,4,5,5,3,0", part1(&(a, program)));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!("6,1,6,4,2,4,7,3,5", part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(202975183645226, part2(&input));
    }
}
