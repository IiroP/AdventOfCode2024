use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

use aoc2024::common::read_input;

const DAY: u32 = 16;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

struct Map {
    start: usize,
    target: (usize, usize),
    vertices: Vec<((usize, usize), bool)>,
    edges: Vec<(usize, usize, u64)>,
    target_h: usize,
    target_v: usize,
}

// Parse input
fn process_input(input: Vec<String>) -> Map {
    let mut start = (0, 0);
    let mut target = (0, 0);
    let mut vertices = Vec::new();
    let mut edges: Vec<(usize, usize, u64)> = Vec::new();
    let height = input.len();
    let width = input[0].len();

    // Create vertices
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == 'S' {
                // Start
                start = (x, y);
            } else if c == 'E' {
                // End
                target = (x, y);
            }

            if c != '#' {
                // Horizontal and vertical
                vertices.push(((x, y), true)); // Horizontal
                vertices.push(((x, y), false)); // Vertical
            }
        }
    }

    // Create edges
    for (i, &(a, horizontal)) in vertices.iter().enumerate() {
        let (x, y) = a;
        let h_weight = if horizontal { 1 } else { 1001 };
        let v_weight = if horizontal { 1001 } else { 1 };
        // Horizontal
        if x > 0 {
            if let Some(b) = vertices.iter().position(|&(v, h)| v == (x - 1, y) && h) {
                edges.push((i, b, h_weight));
            }
        }
        if x < width - 1 {
            if let Some(b) = vertices.iter().position(|&(v, h)| v == (x + 1, y) && h) {
                edges.push((i, b, h_weight));
            }
        }

        // Vertical
        if y > 0 {
            if let Some(b) = vertices.iter().position(|&(v, h)| v == (x, y - 1) && !h) {
                edges.push((i, b, v_weight));
            }
        }
        if y < height - 1 {
            if let Some(b) = vertices.iter().position(|&(v, h)| v == (x, y + 1) && !h) {
                edges.push((i, b, v_weight));
            }
        }

        // Turn
        if let Some(b) = vertices
            .iter()
            .position(|&(v, h)| v == (x, y) && h == !horizontal)
        {
            edges.push((i, b, 1000));
        }
    }

    let start_index = vertices.iter().position(|&(v, h)| v == start && h).unwrap();
    let target_i1 = vertices
        .iter()
        .position(|&(v, h)| v == target && h)
        .unwrap();
    let target_i2 = vertices
        .iter()
        .position(|&(v, h)| v == target && !h)
        .unwrap();

    Map {
        start: start_index,
        target,
        vertices,
        edges,
        target_h: target_i1,
        target_v: target_i2,
    }
}

// Parse sample input
fn _sample_input() -> Map {
    let data = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Map {
    process_input(read_input(DAY))
}

fn dijkstra(
    start: usize,
    target: (usize, usize),
    vertices: &Vec<((usize, usize), bool)>,
    edges: &Vec<(usize, usize, u64)>,
    part2: bool,
) -> (HashMap<usize, u64>, HashMap<usize, Vec<usize>>) {
    let mut dist: HashMap<usize, u64> = vertices
        .iter()
        .enumerate()
        .map(|(i, _)| (i, u64::MAX))
        .collect();
    let mut prev: HashMap<usize, Vec<usize>> = vertices
        .iter()
        .enumerate()
        .map(|(i, _)| (i, Vec::new()))
        .collect();
    let mut queue = (0..vertices.len()).collect::<Vec<usize>>();
    dist.insert(start, 0);

    while queue.len() > 0 {
        //println!("{} vertices left", queue.len());
        let u = *queue.iter().min_by_key(|&i| dist[i]).unwrap();
        if vertices[u].0 == target {
            break;
        }
        queue.retain(|&i| i != u);

        for &(_, v, d) in edges.iter().filter(|(a, _, _)| *a == u) {
            let alt = dist[&u] + d;
            if alt < dist[&v] {
                dist.insert(v, alt);
                prev.insert(v, vec![u]);
            } else if part2 && alt == dist[&v] {
                // In part2, we need to keep track of all possible paths
                prev.get_mut(&v).unwrap().push(u);
                //println!("For vertex {:?}, prev is {:?}", vertices[v], prev[&v]);
            }
        }
    }
    return (dist, prev);
}

#[allow(dead_code)]
fn print_route(
    start: usize,
    target: (usize, usize),
    vertices: &Vec<((usize, usize), bool)>,
    prev: &HashMap<usize, i64>,
) {
    // Print route as grid, marking the start with S and end with E, path with #
    let mut grid: Vec<Vec<char>> =
        vec![
            vec!['.'; vertices.iter().map(|v| v.0 .0).max().unwrap() + 1];
            vertices.iter().map(|v| v.0 .1).max().unwrap() + 1
        ];
    let mut u = vertices
        .iter()
        .position(|&(v, h)| v == target && !h)
        .unwrap();
    while prev[&u] != -1 {
        let (x, y) = vertices[u].0;
        grid[y][x] = '#';
        u = prev[&u] as usize;
    }
    let (sx, sy) = vertices[start].0;
    let (ex, ey) = target;
    grid[sy][sx] = 'S';
    grid[ey][ex] = 'E';

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

// Slow but works
fn part1(input: &Map) -> u64 {
    let (dist, _) = dijkstra(
        input.start,
        input.target,
        &input.vertices,
        &input.edges,
        false,
    );
    let d1 = dist[&input.target_h];
    let d2 = dist[&input.target_v];
    min(d1, d2)
}

// Slow but works
fn part2(input: &Map) -> i64 {
    let (dist, prev) = dijkstra(
        input.start,
        input.target,
        &input.vertices,
        &input.edges,
        true,
    );
    let mut best: HashSet<(usize, usize)> = HashSet::new();
    let mut visited: HashSet<usize> = HashSet::new();
    let d1 = dist[&input.target_h];
    let d2 = dist[&input.target_v];
    let mut queue = vec![if d1 < d2 {
        input.target_h
    } else {
        input.target_v
    }];

    while queue.len() > 0 {
        //println!("{} vertices left for best", queue.len());
        let u = queue[0];
        queue.swap_remove(0);
        if visited.contains(&u) {
            continue;
        }

        best.insert(input.vertices[u].0);

        let mut next = prev[&u].clone();
        queue.append(&mut next);
        visited.insert(u);
    }

    /*// Print grid that shows all best spots as O
    let mut grid: Vec<Vec<char>> =
        vec![
            vec!['.'; input.vertices.iter().map(|v| v.0 .0).max().unwrap() + 1];
            input.vertices.iter().map(|v| v.0 .1).max().unwrap() + 1
        ];
    for &(x, y) in &best {
        grid[y][x] = 'O';
    }
    let (sx, sy) = input.vertices[input.start].0;
    let (ex, ey) = input.target;
    println!("Is end included: {}", best.contains(&input.target));
    println!(
        "Is start included: {}",
        best.contains(&input.vertices[input.start].0)
    );
    grid[sy][sx] = 'S';
    grid[ey][ex] = 'E';

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }*/

    best.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(7036, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(45, part2(&input));
    }

    #[test]
    fn test_part2b() {
        let input: Vec<String> = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        let input = process_input(input);
        assert_eq!(64, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(107512, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(561, part2(&input));
    }
}
