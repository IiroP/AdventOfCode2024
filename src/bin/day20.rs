use aoc2024::common::read_input;
use itertools::Itertools;

const DAY: u32 = 20;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input, false);
    println!("Day {DAY}, part 2: {part2_result}");
}

struct Grid {
    start: (usize, usize),
    end: (usize, usize),
    walls: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

// Parse input
fn process_input(input: Vec<String>) -> Grid {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut walls = Vec::new();
    let height = input.len();
    let width = input[0].len();
    for y in 0..input.len() {
        for (x, c) in input[y].chars().enumerate() {
            match c {
                'S' => start = (x, y),
                'E' => end = (x, y),
                '#' => walls.push((x, y)),
                _ => (),
            }
        }
    }
    Grid {
        start,
        end,
        walls,
        width,
        height,
    }
}

// Parse sample input
fn _sample_input() -> Grid {
    let data = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Grid {
    process_input(read_input(DAY))
}

fn dijkstra(
    start: usize,
    target: (usize, usize),
    vertices: &Vec<(usize, usize)>,
    neighbors: &Vec<Vec<usize>>,
) -> (Vec<u64>, Vec<i64>) {
    let mut dist: Vec<u64> = (0..vertices.len()).map(|_| u64::MAX).collect();
    let mut prev: Vec<i64> = (0..vertices.len()).map(|_| -1).collect();
    let mut queue = (0..vertices.len()).collect::<Vec<usize>>();
    dist.insert(start, 0);

    while queue.len() > 0 {
        //println!("{} vertices left", queue.len());
        let u = *queue.iter().min_by_key(|&i| dist[*i]).unwrap();
        if vertices[u] == target {
            break;
        }
        queue.retain(|&i| i != u);

        if dist[u] == u64::MAX {
            continue;
        }

        for &next in neighbors[u].iter() {
            let alt = dist[u] + 1;
            if alt < dist[next] {
                dist[next] = alt;
                prev[next] = u as i64;
            }
        }
    }
    return (dist, prev);
}

fn neighbors(
    vertices: &Vec<(usize, usize)>,
    walls: &Vec<(usize, usize)>,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut neighbors: Vec<Vec<usize>> = Vec::new();
    for (i, &(x, y)) in vertices.iter().enumerate() {
        // If wall => not possible to go anywhere
        if walls.contains(&(x, y)) {
            neighbors.push(Vec::new());
            continue;
        }
        let mut n = Vec::new();
        if x > 0 && !walls.contains(&(x - 1, y)) {
            n.push(i - 1);
        }
        if x < width - 1 && !walls.contains(&(x + 1, y)) {
            n.push(i + 1);
        }
        if y > 0 && !walls.contains(&(x, y - 1)) {
            n.push(i - width);
        }
        if y < height - 1 && !walls.contains(&(x, y + 1)) {
            n.push(i + width);
        }
        neighbors.push(n);
    }
    neighbors
}

fn possible_cheats(
    vertices: &Vec<(usize, usize)>,
    walls: &Vec<(usize, usize)>,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut possible: Vec<Vec<usize>> = Vec::new();
    for (i, &(x, y)) in vertices.iter().enumerate() {
        if walls.contains(&(x, y)) {
            possible.push(Vec::new());
            continue;
        }

        let mut n = Vec::new();
        if x > 1 && walls.contains(&(x - 1, y)) && !walls.contains(&(x - 2, y)) {
            n.push(i - 2);
        }
        if x < width - 2 && walls.contains(&(x + 1, y)) && !walls.contains(&(x + 2, y)) {
            n.push(i + 2);
        }
        if y > 1 && walls.contains(&(x, y - 1)) && !walls.contains(&(x, y - 2)) {
            n.push(i - 2 * width);
        }
        if y < height - 2 && walls.contains(&(x, y + 1)) && !walls.contains(&(x, y + 2)) {
            n.push(i + 2 * width);
        }
        possible.push(n);
    }
    possible
}

fn possible_cheats2(
    vertices: &Vec<(usize, usize)>,
    walls: &Vec<(usize, usize)>,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut possible: Vec<Vec<usize>> = Vec::new();
    let distance = 20;
    for &(x, y) in vertices.iter() {
        if walls.contains(&(x, y)) {
            possible.push(Vec::new());
            continue;
        }
        let mut n = Vec::new();
        let min_y = if y > distance - 1 { y - distance } else { 0 };
        let max_y = if height > distance && y < height - distance {
            y + distance
        } else {
            height - 1
        };
        let min_x = if x > distance - 1 { x - distance } else { 0 };
        let max_x = if width > distance && x < width - distance {
            x + distance
        } else {
            width - 1
        };
        for j in min_y..=max_y {
            for k in min_x..=max_x {
                if !walls.contains(&(k, j)) && (k, j) != (x, y) {
                    n.push(j * width + k);
                }
            }
        }
        possible.push(n);
    }
    possible
}

fn cheat_values(
    possible: &Vec<Vec<usize>>,
    dist: &Vec<u64>,
    width: usize,
) -> Vec<(usize, usize, u64)> {
    possible
        .iter()
        .enumerate()
        .flat_map(|(i, cheats)| {
            cheats.iter().flat_map(move |&c| {
                let cheat = if dist[i] == u64::MAX {
                    // Prevent overflow
                    u64::MAX
                } else {
                    let start = (i % width, i / width);
                    let end = (c % width, c / width);
                    let change = (start.0 as i64 - end.0 as i64).abs() as u64
                        + (start.1 as i64 - end.1 as i64).abs() as u64;
                    if change > 20 {
                        u64::MAX
                    } else {
                        dist[i] + change
                    }
                };
                if cheat < dist[c] {
                    Some((i, c, dist[c] - cheat))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn part1(input: &Grid) -> u64 {
    let width = input.width;
    let height = input.height;
    let vertices: Vec<(usize, usize)> = (0..width * height)
        .map(|i| (i % width, i / width))
        .collect();
    let start_index = vertices.iter().position(|&w| w == input.start).unwrap();
    let neighbors = neighbors(&vertices, &input.walls, width, height);
    let possible = possible_cheats(&vertices, &input.walls, width, height);
    let (dist, _) = dijkstra(start_index, input.end, &vertices, &neighbors);
    let cheats = cheat_values(&possible, &dist, width);
    cheats.iter().filter(|&(_, _, a)| *a >= 100).count() as u64
}

fn part2(input: &Grid, test: bool) -> u64 {
    let width = input.width;
    let height = input.height;
    let vertices: Vec<(usize, usize)> = (0..width * height)
        .map(|i| (i % width, i / width))
        .collect();
    let start_index = vertices.iter().position(|&w| w == input.start).unwrap();
    let neighbors = neighbors(&vertices, &input.walls, width, height);
    let possible = possible_cheats2(&vertices, &input.walls, width, height);
    let (dist, _) = dijkstra(start_index, input.end, &vertices, &neighbors);
    let cheats = cheat_values(&possible, &dist, width);
    let limit = if test { 50 } else { 100 };
    cheats
        .iter()
        .filter(|&(_, _, a)| *a >= limit)
        .unique()
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        part1(&input);
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        let result = part2(&input, true);
        assert_eq!(285, result);
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(1343, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(982891, part2(&input, false));
    }
}
