use std::collections::{HashMap, HashSet, VecDeque};

use aoc2024::common::read_input;
use itertools::Itertools;
use regex::Regex;

const DAY: u32 = 24;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input, false);
    println!("Day {DAY}, part 2: {part2_result}");
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Gate {
    left: [char; 3],
    cmd: char, // A(nd), O(r), X(or)
    right: [char; 3],
    target: [char; 3],
}

// Parse input
fn process_input(input: Vec<String>) -> (HashMap<[char; 3], bool>, Vec<Gate>) {
    let initial = Regex::new(r"^(\S+): (\d)$").unwrap();
    let connection = Regex::new(r"^(\S+) (AND|OR|XOR) (\S+) -> (\S+)$").unwrap();
    let mut output = HashMap::new();
    let mut gates = Vec::new();
    for line in &input {
        if let Some(captures) = initial.captures(&line) {
            let target: [char; 3] = captures
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let value = captures.get(2).unwrap().as_str();
            output.insert(target, value == "1");
        } else if let Some(captures) = connection.captures(&line) {
            let left = captures.get(1).unwrap().as_str();
            let cmd = captures.get(2).unwrap().as_str();
            let right = captures.get(3).unwrap().as_str();
            let target = captures.get(4).unwrap().as_str();
            gates.push(Gate {
                left: left.chars().collect::<Vec<_>>().try_into().unwrap(),
                cmd: cmd.chars().next().unwrap(),
                right: right.chars().collect::<Vec<_>>().try_into().unwrap(),
                target: target.chars().collect::<Vec<_>>().try_into().unwrap(),
            });
        }
    }
    // Insert zero gate
    output.insert(['0', '0', '0'], false);
    (output, gates)
}

// Parse day's input
fn day_input() -> (HashMap<[char; 3], bool>, Vec<Gate>) {
    process_input(read_input(DAY))
}

// Evaluate a gate, update map
fn eval_gate(gate: &Gate, map: &mut HashMap<[char; 3], bool>) {
    let value = match gate.cmd {
        'A' => map[&gate.left] & map[&gate.right],
        'O' => map[&gate.left] | map[&gate.right],
        'X' => map[&gate.left] ^ map[&gate.right],
        _ => panic!("Unknown command"),
    };
    map.insert(gate.target.to_owned(), value);
}

// Read number from specific prefix
fn get_number(prefix: &str, map: &HashMap<[char; 3], bool>) -> usize {
    let mut result = 0;
    let x_pattern = Regex::new(r"^x(\d+)$").unwrap();
    let y_pattern = Regex::new(r"^y(\d+)$").unwrap();
    let z_pattern = Regex::new(r"^z(\d+)$").unwrap();
    let pattern = match prefix {
        "x" => x_pattern,
        "y" => y_pattern,
        "z" => z_pattern,
        _ => panic!("Unknown prefix"),
    };
    for (key, value) in map {
        let key_str: String = key.iter().collect();
        if let Some(id) = pattern.captures(&key_str) {
            if *value {
                result += 1 << id.get(1).unwrap().as_str().parse::<usize>().unwrap();
            }
        }
    }
    result
}

fn evaluate_all(gates: &Vec<Gate>, map: &mut HashMap<[char; 3], bool>) {
    let mut remaining: VecDeque<usize> = (0..gates.len()).collect();

    // Evaluate gates until all are resolved
    while !remaining.is_empty() {
        let i = remaining.pop_front().unwrap();
        let gate = &gates[i];
        if map.contains_key(&gate.left) && map.contains_key(&gate.right) {
            // Evaluate gate
            eval_gate(gate, map);
        } else {
            // Not ready yet, put it back
            remaining.push_back(i);
        }
    }
}

// Name gates based on the structure of ripple-carry adder
fn name_gates(gates: &Vec<Gate>) -> HashMap<[char; 3], String> {
    let mut names = HashMap::new();
    gates.iter().for_each(|g| {
        if g.left[0] == 'x' || g.left[0] == 'y' {
            let index = g.left[1..]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            names.insert(g.target, format!("{}{:02}", g.cmd, index));
        }
    });

    fn iterate_gate(g: &Gate, names: &mut HashMap<[char; 3], String>) {
        let left = names.get(&g.left).map_or("---", |v| v);
        let right = names.get(&g.right).map_or("---", |v| v);
        if g.cmd == 'O' {
            if left.starts_with('A') {
                names.insert(g.target, format!("c{:02}", &left[1..]));
            } else if right.starts_with('A') {
                names.insert(g.target, format!("c{:02}", &right[1..]));
            } else if left.starts_with('d') {
                names.insert(g.target, format!("c{:02}", &left[1..]));
            } else if right.starts_with('d') {
                names.insert(g.target, format!("c{:02}", &right[1..]));
            }
        } else if g.cmd == 'A' {
            if left.starts_with('X') {
                names.insert(g.target, format!("d{:02}", &left[1..]));
            } else if right.starts_with('X') {
                names.insert(g.target, format!("d{:02}", &right[1..]));
            } else if left.starts_with('c') {
                names.insert(
                    g.target,
                    format!("d{:02}", &left[1..].parse::<usize>().unwrap() + 1),
                );
            } else if right.starts_with('c') {
                names.insert(
                    g.target,
                    format!("d{:02}", &right[1..].parse::<usize>().unwrap() + 1),
                );
            }
        } else if g.cmd == 'X' {
            if right.starts_with('X') {
                names.insert(g.target, format!("z{:02}", &right[1..]));
            } else if left.starts_with('X') {
                names.insert(g.target, format!("z{:02}", &left[1..]));
            } else if (left == "A00" && right == "X01") || (right == "A00" && left == "X01") {
                names.insert(g.target, "z01".to_string());
            } else if left.starts_with('c') {
                names.insert(
                    g.target,
                    format!("z{:02}", &left[1..].parse::<usize>().unwrap() + 1),
                );
            } else if right.starts_with('c') {
                names.insert(
                    g.target,
                    format!("z{:02}", &right[1..].parse::<usize>().unwrap() + 1),
                );
            }
        }
    }

    // Iterate this multiple times to get new names from previous rounds
    (0..3).for_each(|_| {
        gates.iter().for_each(|g| {
            iterate_gate(g, &mut names);
        });
    });
    names
}

#[allow(dead_code)]
fn gate_with_names(gate: &Gate, names: &HashMap<[char; 3], String>) -> String {
    let og_target = gate.target.iter().collect();
    let og_left = gate.left.iter().collect();
    let og_right = gate.right.iter().collect();

    let gate_name = names.get(&gate.target).unwrap_or(&og_target);
    let left_name = names.get(&gate.left).unwrap_or(&og_left);
    let right_name = names.get(&gate.right).unwrap_or(&og_right);
    format!(
        "Gate {}: {} {} {}, originally {}: {} {} {}",
        gate_name, left_name, gate.cmd, right_name, og_target, og_left, gate.cmd, og_right
    )
}

fn test_gates(input: &(HashMap<[char; 3], bool>, Vec<Gate>), n: usize) -> Vec<String> {
    let gates = &input.1;
    let names = name_gates(gates);
    let mut errors = HashSet::new();

    // Optionally print all gates with names
    /*gates.iter().for_each(|g| {
        let gate_desc = gate_with_names(g, &names);
        //if names.get(&g.target).is_none() {
        //    println!("Gate has no name: {}", gate_desc);
        //}
        println!("{}", gate_desc);
    });*/

    (0..n).for_each(|i| {
        let default_name = "".to_string();
        let real_name = names
            .get(&[
                'z',
                format!("{:02}", i).chars().nth(0).unwrap(),
                format!("{:02}", i).chars().nth(1).unwrap(),
            ])
            .unwrap_or(&default_name);

        // Test z gate names
        if real_name.len() > 0 && i > 0 && format!("z{:02}", i) != *real_name {
            println!("Gate z{:02} has unexpected name {}", i, real_name);
            errors.insert(format!("z{:02}", i));
            errors.insert(real_name.clone());
        }

        // Validate c__ gate
        let c_gate = gates.iter().find(|g| {
            let left = names.get(&g.left);
            let right = names.get(&g.right);
            g.cmd == 'O'
                && (left == Some(&format!("A{:02}", i))
                    || right == Some(&format!("A{:02}", i))
                    || left == Some(&format!("d{:02}", i))
                    || right == Some(&format!("d{:02}", i)))
        });
        if let Some(c) = c_gate {
            let default = "---".to_string();
            let left = names.get(&c.left).unwrap_or(&default);
            let right = names.get(&c.right).unwrap_or(&default);
            if (left == &format!("A{:02}", i) && right == &format!("d{:02}", i))
                || (right == &format!("A{:02}", i) && left == &format!("d{:02}", i))
            {
                // Valid
            } else if left == &format!("A{:02}", i) {
                // A__ is valid, d__ is missing
                let missing = format!("d{:02}", i);
                errors.insert(right.clone());
                errors.insert(missing);
            } else if left == &format!("d{:02}", i) {
                // d__ is valid, A__ is missing
                let missing = format!("A{:02}", i);
                errors.insert(right.clone());
                errors.insert(missing);
            } else if right == &format!("A{:02}", i) {
                // A__ is valid, d__ is missing
                let missing = format!("d{:02}", i);
                errors.insert(left.clone());
                errors.insert(missing);
            } else if right == &format!("d{:02}", i) {
                // d__ is valid, A__ is missing
                let missing = format!("A{:02}", i);
                errors.insert(left.clone());
                errors.insert(missing);
            }
        }
    });

    let mut result = errors
        .into_iter()
        .map(|a| {
            names
                .iter()
                .find(|(_, v)| **v == *a)
                .map(|(k, _)| k.iter().collect::<String>())
                .unwrap_or("---".to_string())
        })
        .collect_vec();
    result.sort();
    result
}

fn part1(input: &(HashMap<[char; 3], bool>, Vec<Gate>)) -> usize {
    let mut map = input.0.clone();
    let gates = &input.1;

    // Evaluate all gates
    evaluate_all(gates, &mut map);

    get_number("z", &map)
}

fn part2(input: &(HashMap<[char; 3], bool>, Vec<Gate>), test: bool) -> String {
    let n = if test { 6 } else { 45 };
    let result = test_gates(input, n);
    result.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Parse sample input
    fn sample_input() -> (HashMap<[char; 3], bool>, Vec<Gate>) {
        let data = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        process_input(data)
    }

    fn sample_input_large() -> (HashMap<[char; 3], bool>, Vec<Gate>) {
        let data = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        process_input(data)
    }

    #[test]
    fn test_something() {
        let input = day_input();
        let mut map = input.0;
        let gates = &input.1;
        // Evaluate all gates
        evaluate_all(gates, &mut map);

        let x = get_number("x", &map);
        let y = get_number("y", &map);
        let z = get_number("z", &map);
        let target = x & y;
        println!("z: {:b}", z);
        println!("t: 0{:b}", target);
    }

    #[test]
    fn test_part1() {
        let input = sample_input();
        assert_eq!(4, part1(&input));
    }

    #[test]
    fn test_part1_large() {
        let input = sample_input_large();
        assert_eq!(2024, part1(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(61886126253040, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!("fgt,fpq,nqk,pcp,srn,z07,z24,z32", part2(&input, false));
    }
}
