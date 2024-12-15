use aoc2024::common::read_input;
use itertools::Itertools;
use regex::Regex;

const DAY: u32 = 14;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input, false);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input, false);
    println!("Day {DAY}, part 2: {part2_result}");
}

struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<Robot> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    input
        .iter()
        .map(|row| {
            let caps = re.captures(row).unwrap();
            Robot {
                x: caps.get(1).unwrap().as_str().parse().unwrap(),
                y: caps.get(2).unwrap().as_str().parse().unwrap(),
                vx: caps.get(3).unwrap().as_str().parse().unwrap(),
                vy: caps.get(4).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

// Parse sample input
fn _sample_input() -> Vec<Robot> {
    let data = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<Robot> {
    process_input(read_input(DAY))
}

fn position_at_time(robot: &Robot, time: i64, width: i64, height: i64) -> (i64, i64) {
    (
        ((robot.x + time * robot.vx) % width + width) % width,
        ((robot.y + time * robot.vy) % height + height) % height,
    )
}

// 1 | 2
// -----
// 3 | 4

fn quadrant(pos: (i64, i64), width: i64, height: i64) -> i64 {
    let (x, y) = pos;
    let mid_x = width / 2;
    let mid_y = height / 2;
    if x == mid_x || y == mid_y {
        // Middle
        0
    } else if x < mid_x && y < mid_y {
        // Top-left
        1
    } else if x > mid_x && y < mid_y {
        // Top-right
        2
    } else if x < mid_x && y > mid_y {
        // Bottom-left
        3
    } else {
        // Bottom-right
        4
    }
}

fn draw(positions: Vec<(i64, i64)>, width: i64, height: i64, time: i64) -> bool {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    for (x, y) in positions {
        grid[y as usize][x as usize] = '#';
    }
    let possible = grid.iter().any(|row| {
        let text: String = row.iter().collect();
        text.contains("#######")
    });
    if !possible {
        return false;
    }
    println!("Time: {}", time);
    for row in grid {
        println!("{}", row.iter().join(""));
    }
    true
}

fn part1(input: &Vec<Robot>, test: bool) -> i64 {
    let height = if test { 7 } else { 103 };
    let width: i64 = if test { 11 } else { 101 };
    input
        .iter()
        .map(|r| {
            let pos = position_at_time(r, 100, width, height);
            quadrant(pos, width, height)
        })
        .counts()
        .iter()
        .filter(|(value, _)| **value != 0)
        .map(|(_, count)| *count as i64)
        .product()
}

fn part2(input: &Vec<Robot>, test: bool) -> i64 {
    let height = if test { 7 } else { 103 };
    let width: i64 = if test { 11 } else { 101 };
    for i in 1..100000 {
        let positions = input.iter().map(|r| position_at_time(r, i, width, height));
        if draw(positions.collect(), width, height, i) {
            return i;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(12, part1(&input, true));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(231019008, part1(&input, false));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(8280, part2(&input, false));
    }
}
