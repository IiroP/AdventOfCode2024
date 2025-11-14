use std::collections::{HashMap, VecDeque};

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

fn find_broken_gates(gates: &Vec<Gate>, map: &HashMap<[char; 3], bool>) -> Vec<usize> {
    let mut broken = Vec::new();

    fn is_valid_xy(gate: &Gate, id: [char; 2]) -> bool {
        (gate.left == ['y', id[0], id[1]] && gate.right == ['x', id[0], id[1]])
            || (gate.left == ['x', id[0], id[1]] && gate.right == ['y', id[0], id[1]])
    }

    let mut prev_mem: [char; 3] = ['0', '0', '0'];
    let mut x_prev: [char; 3] = ['0', '0', '0'];

    let n = map.iter().filter(|(k, _)| k[0] == 'x').count();
    for i in 1..n {
        let id: [char; 2] = format!("{:02}", i)
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let target: [char; 3] = ['z', id[0], id[1]];
        println!("Checking {:?}", target);
        let gate = gates.iter().find(|g| g.target == target).unwrap();

        // Find current XOR gate
        let x_curr = gates
            .iter()
            .find(|&g| {
                g.cmd == 'X' && (g.left == ['x', id[0], id[1]] || g.right == ['x', id[0], id[1]])
            })
            .unwrap();

        // Skip if the gate is the first one
        if gate.left[0] == 'x'
            || gate.left[0] == 'y'
            || gate.right[0] == 'x'
            || gate.right[0] == 'y'
        {
            x_prev = x_curr.target;
            println!("Skipping");
            continue;
        }

        let prev_id: [char; 2] = format!("{:02}", i - 1)
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        // Find other relevant gates
        let mem = gates
            .iter()
            .find(|&g| {
                g.cmd == 'A'
                    && (g.left == ['x', prev_id[0], prev_id[1]]
                        || g.right == ['x', prev_id[0], prev_id[1]])
            })
            .unwrap();

        let left = gates.iter().find(|g| g.target == gate.left).unwrap();
        let right = gates.iter().find(|g| g.target == gate.right).unwrap();
        if gate.cmd != 'X' {
            println!("Expected XOR");
            broken.push(i);
        } else if left != x_curr && right != x_curr {
            println!("Fail at: {:?}", gate);
            broken.push(i);
        }
    }
    broken
}

fn part1(input: &(HashMap<[char; 3], bool>, Vec<Gate>)) -> usize {
    let mut map = input.0.clone();
    let gates = &input.1;

    // Evaluate all gates
    evaluate_all(gates, &mut map);

    get_number("z", &map)
}

fn part2(input: &(HashMap<[char; 3], bool>, Vec<Gate>), test: bool) -> String {
    let map = input.0.clone();
    let gates = &input.1;

    let gate_indices = (0..gates.len()).collect::<Vec<_>>();
    let swaps = if test { 4 } else { 8 };
    let mut possibilities = gate_indices.iter().copied().combinations(swaps);

    // Try all combinations of 4 gates
    let mut gates = gates.clone();
    let result = possibilities.find(|p| {
        // Calculate all permutations = different orders
        //println!("Testing {:?}", p);
        p.iter().permutations(p.len()).unique().any(|p1| {
            println!("Perm: {:?}", p1);
            gates = gates.clone();
            let mut map = map.clone();

            // Apply changes
            p1.windows(2).for_each(|v| {
                let a = v[0];
                let b = v[1];
                let target_a = gates[*a].target;
                let target_b = gates[*b].target;
                gates[*a].target = target_b;
                gates[*b].target = target_a;
            });

            // Evaluate all gates
            evaluate_all(&gates, &mut map);

            // Check if we have the right result
            let x = get_number("x", &map);
            let y = get_number("y", &map);
            let z = get_number("z", &map);
            let target = x & y;

            z == target
        })
    });

    result
        .unwrap()
        .iter()
        .map(|&a| gates[a].target)
        .sorted()
        .map(|v| v.iter().collect::<String>())
        .join(",")
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
    fn test_find_broken() {
        let (map, gates) = day_input();
        let result = find_broken_gates(&gates, &map);
        println!("{:?}", result);
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
    fn test_part2() {
        let input: Vec<String> = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00"
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        let input = process_input(input);
        assert_eq!("z00,z01,z02,z05", part2(&input, true));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(61886126253040, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!("", part2(&input, false));
    }
}
