use std::collections::HashSet;

use aoc2024::common::read_input;

const DAY: u32 = 6;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> ((i32, i32), HashSet<(i32, i32)>, Vec<String>) {
    let width = input[0].len();
    let height = input.len();
    let mut start = (0, 0) as (i32, i32);
    let mut obstacles = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            match input[y].chars().nth(x).unwrap() {
                '#' => {
                    obstacles.insert((x.try_into().unwrap(), y.try_into().unwrap()));
                }
                '^' => start = (x.try_into().unwrap(), y.try_into().unwrap()),
                _ => {}
            }
        }
    }
    (start, obstacles, input)
}

// Parse sample input
fn _sample_input() -> ((i32, i32), HashSet<(i32, i32)>, Vec<String>) {
    let data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> ((i32, i32), HashSet<(i32, i32)>, Vec<String>) {
    process_input(read_input(DAY))
}

// Move guard and update history
fn move_guard(
    current: (i32, i32),
    dir: i32,
    obstacles: &HashSet<(i32, i32)>,
    visited: &mut HashSet<(i32, i32)>,
) -> ((i32, i32), i32) {
    visited.insert(current);
    move_guard_stateless(current, dir, obstacles)
}

// Move guard and update history (with direction)
fn move_guard_full(
    current: (i32, i32),
    dir: i32,
    obstacles: &HashSet<(i32, i32)>,
    visited: &mut HashSet<((i32, i32), i32)>,
) -> ((i32, i32), i32) {
    visited.insert((current, dir));
    move_guard_stateless(current, dir, obstacles)
}

// Move guard without updating history
fn move_guard_stateless(
    current: (i32, i32),
    dir: i32,
    obstacles: &HashSet<(i32, i32)>,
) -> ((i32, i32), i32) {
    let mut dir = dir;
    let proposed = match dir {
        0 => (current.0, current.1 - 1),
        1 => (current.0 + 1, current.1),
        2 => (current.0, current.1 + 1),
        3 => (current.0 - 1, current.1),
        _ => panic!("Invalid direction"),
    };
    if obstacles.contains(&proposed) {
        dir = (dir + 1) % 4;
        (current.to_owned(), dir)
    } else {
        (proposed, dir)
    }
}

// Check if position is inside grid
fn is_inside(pos: (i32, i32), width: i32, height: i32) -> bool {
    pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height
}

fn part1(input: &((i32, i32), HashSet<(i32, i32)>, Vec<String>)) -> i64 {
    let (start, obstacles, grid) = input;
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let mut visited = HashSet::new();

    let mut current = (start.to_owned(), 0);
    while is_inside(current.0, width, height) {
        current = move_guard(current.0, current.1, obstacles, &mut visited);
    }
    visited.len() as i64
}

// Slow (~8s with debug, <1s with release)
fn part2(input: &((i32, i32), HashSet<(i32, i32)>, Vec<String>)) -> i64 {
    let (start, obstacles, grid) = input;
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    // Idea 2: At any moment, try to put stone in front of guard and check if a loop is formed
    fn check_loop(
        guard: (i32, i32),
        dir_orig: i32,
        obstacles: &HashSet<(i32, i32)>,
        width: i32,
        height: i32,
        visited: &HashSet<((i32, i32), i32)>,
    ) -> bool {
        // Check validity of stone position
        let (stone, _) = move_guard_stateless(guard, dir_orig, obstacles);
        if !is_inside(stone, width, height) || visited.iter().any(|&(a, _)| a == stone) {
            return false;
        }
        // Set mutable variables
        let mut guard_pos = guard;
        let mut dir = (dir_orig + 1) % 4;
        let mut visited_loop = visited.clone();
        visited_loop.insert((guard, dir_orig));
        visited_loop.insert((guard, dir));
        // Check if loop is formed
        loop {
            let proposed = move_guard_stateless(guard_pos, dir, obstacles);
            if proposed.0 == stone {
                // Approaching stone, force turn
                dir = (dir + 1) % 4;
            } else if proposed.1 != dir {
                // Guard turns without moving
                dir = proposed.1;
            } else if visited_loop.contains(&(proposed.0, proposed.1)) {
                // Returned to same position => loop is ready
                return true;
            } else if !is_inside(proposed.0, width, height) {
                // Guard is outside of grid
                return false;
            } else {
                // Guard moves
                guard_pos = proposed.0;
                dir = proposed.1;
            }
            visited_loop.insert((guard_pos, dir));
        }
    }

    let mut visited: HashSet<((i32, i32), i32)> = HashSet::new();
    let mut stones = HashSet::new();
    let mut current = (start.to_owned(), 0);
    while is_inside(current.0, width, height) {
        // Place stone in front of guard
        let stone = move_guard_stateless(current.0, current.1, &obstacles).0;
        // Check if square is formed
        if check_loop(current.0, current.1, &obstacles, width, height, &visited) {
            stones.insert(stone);
        }
        // Move guard (normally)
        current = move_guard_full(current.0, current.1, obstacles, &mut visited);
    }
    stones.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(41, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(6, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(4826, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(1721, part2(&input));
    }
}
