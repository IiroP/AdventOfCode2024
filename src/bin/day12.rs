use std::collections::HashSet;

use aoc2024::common::read_input;

const DAY: u32 = 12;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<Vec<char>> {
    input
        .iter()
        .map(|s| s.chars().collect())
        .filter(|row: &Vec<char>| row.len() > 0)
        .collect()
}

// Parse sample input
fn _sample_input() -> Vec<Vec<char>> {
    let data = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<Vec<char>> {
    process_input(read_input(DAY))
}

// Count fence parts for a given position
fn connections(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    input: &Vec<Vec<char>>,
) -> Connections {
    let mut result = Connections {
        left: None,
        right: None,
        up: None,
        down: None,
    };
    let current = input[y][x];
    if x > 0 && input[y][x - 1] == current {
        result.left = Some((x - 1, y));
    }
    if y > 0 && input[y - 1][x] == current {
        result.up = Some((x, y - 1));
    }
    if x < width - 1 && input[y][x + 1] == current {
        result.right = Some((x + 1, y));
    }
    if y < height - 1 && input[y + 1][x] == current {
        result.down = Some((x, y + 1));
    }
    result
}

fn corners(x: usize, y: usize, width: usize, height: usize, input: &Vec<Vec<char>>) -> u64 {
    let mut result = vec![false, false, false, false]; // up, right, down, left
    let current = input[y][x];
    if y == 0 || input[y - 1][x] != current {
        result[0] = true;
    }
    if x == width - 1 || input[y][x + 1] != current {
        result[1] = true;
    }
    if y == height - 1 || input[y + 1][x] != current {
        result[2] = true;
    }
    if x == 0 || input[y][x - 1] != current {
        result[3] = true;
    }

    // Handle special corners (270 degrees), go counterclockwise
    let mut specials = 0;
    if result[2] && !result[1] && x < width - 1 && y < height - 1 {
        // Has fence on the bottom, but not on the right (and not on edge)
        if input[y + 1][x + 1] == current {
            specials += 1;
        }
    }
    if result[1] && !result[0] && x < width - 1 && y > 0 {
        // Has fence on the right, but not on the top (and not on the top edge)
        if input[y - 1][x + 1] == current {
            specials += 1;
        }
    }
    if result[0] && !result[3] && x > 0 && y > 0 {
        // Has fence on the top, but not on the left (and not on the left edge)
        if input[y - 1][x - 1] == current {
            specials += 1;
        }
    }
    if result[3] && !result[2] && x > 0 && y < height - 1 {
        // Has fence on the left, but not on the bottom (and not on the bottom edge)
        if input[y + 1][x - 1] == current {
            specials += 1;
        }
    }

    // Add first corner to the end
    result.push(result[0]);
    // Calculate corners
    result
        .windows(2)
        .filter(|&window| window == [true, true])
        .count() as u64
        + specials
}

struct Connections {
    left: Option<(usize, usize)>,
    right: Option<(usize, usize)>,
    up: Option<(usize, usize)>,
    down: Option<(usize, usize)>,
}

// Struct tp store connections (to same symbol) for a given position
impl Connections {
    pub fn as_vec(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        if let Some(v) = &self.left {
            result.push(*v);
        }
        if let Some(v) = &self.right {
            result.push(*v);
        }
        if let Some(v) = &self.up {
            result.push(*v);
        }
        if let Some(v) = &self.down {
            result.push(*v);
        }
        result
    }

    pub fn len(&self) -> usize {
        let mut result = 0;
        if self.left.is_some() {
            result += 1;
        }
        if self.right.is_some() {
            result += 1;
        }
        if self.up.is_some() {
            result += 1;
        }
        if self.down.is_some() {
            result += 1;
        }
        result
    }
}

// Depth-first search
fn dfs_component(
    v: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    component: &mut Vec<(usize, usize)>,
    input: &Vec<Vec<char>>,
) {
    visited.insert(v);
    component.push(v);
    let (x, y) = v;
    let width = input[0].len();
    let height = input.len();
    connections(x, y, width, height, input)
        .as_vec()
        .iter()
        .for_each(|edge| {
            if !visited.contains(edge) {
                dfs_component(*edge, visited, component, input);
            }
        });
}

fn part1(input: &Vec<Vec<char>>) -> u64 {
    let mut visited = HashSet::new();
    let height = input.len();
    let width = input[0].len();
    let mut total_price = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if visited.contains(&(x, y)) {
                continue;
            }
            let mut component = Vec::new();
            dfs_component((x, y), &mut visited, &mut component, input);
            //println!("{:?}", component);
            let area = component.len() as u64;
            let fence = component
                .iter()
                .map(|&(x, y)| 4 - connections(x, y, width, height, input).len() as u64)
                .sum::<u64>();
            let price = area * fence;
            total_price += price;
        }
    }

    total_price
}

fn part2(input: &Vec<Vec<char>>) -> u64 {
    let mut visited = HashSet::new();
    let height = input.len();
    let width = input[0].len();
    let mut total_price = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if visited.contains(&(x, y)) {
                continue;
            }
            let mut component = Vec::new();
            dfs_component((x, y), &mut visited, &mut component, input);
            let area = component.len() as u64;
            // Sides are calculated by counting corners (idea from Reddit)
            let sides: u64 = component
                .iter()
                .map(|&(x, y)| corners(x, y, width, height, input))
                .sum();
            let price = area * sides;
            total_price += price;
        }
    }

    total_price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(1930, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(1206, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(1533024, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(910066, part2(&input));
    }
}
