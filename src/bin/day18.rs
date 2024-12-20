use aoc2024::common::read_input;

const DAY: u32 = 18;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input, false);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input, false);
    println!("Day {DAY}, part 2: {:?}", part2_result);
}

// Parse input
fn process_input(input: Vec<String>) -> Vec<(usize, usize)> {
    input
        .iter()
        .map(|s| {
            let mut a = s.split(",");
            (
                a.next().unwrap().parse::<usize>().unwrap(),
                a.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect()
}

// Parse sample input
fn _sample_input() -> Vec<(usize, usize)> {
    let data = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
    .split('\n')
    .map(|s| s.to_string())
    .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> Vec<(usize, usize)> {
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
    stones: &Vec<(usize, usize)>,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut neighbors: Vec<Vec<usize>> = Vec::new();
    for (i, &(x, y)) in vertices.iter().enumerate() {
        // If stone => not possible to go anywhere
        if stones.contains(&(x, y)) {
            neighbors.push(Vec::new());
            continue;
        }
        let mut n = Vec::new();
        if x > 0 && !stones.contains(&(x - 1, y)) {
            n.push(i - 1);
        }
        if x < width - 1 && !stones.contains(&(x + 1, y)) {
            n.push(i + 1);
        }
        if y > 0 && !stones.contains(&(x, y - 1)) {
            n.push(i - width);
        }
        if y < height - 1 && !stones.contains(&(x, y + 1)) {
            n.push(i + width);
        }
        neighbors.push(n);
    }
    neighbors
}

fn part1(input: &Vec<(usize, usize)>, test: bool) -> u64 {
    let elapsed = if test { 12 } else { 1024 };
    let len = if test { 7 } else { 71 };
    let stones = input[..elapsed].to_vec();
    let vertices: Vec<(usize, usize)> = (0..len * len).map(|i| (i % len, i / len)).collect();
    let neighbors = neighbors(&vertices, &stones, len, len);
    let (dist, _) = dijkstra(0, (len - 1, len - 1), &vertices, &neighbors);
    dist[len * len - 1]
}

fn part2(input: &Vec<(usize, usize)>, test: bool) -> (usize, usize) {
    let len = if test { 7 } else { 71 };
    let vertices: Vec<(usize, usize)> = (0..len * len).map(|i| (i % len, i / len)).collect();
    let indices = (1..input.len()).collect::<Vec<usize>>();
    let result = indices.partition_point(|&i| {
        //println!("Trying with {} stones", i);
        let stones = input[..i].to_vec();
        //println!("Stones: {:?}", stones);
        let neighbors = neighbors(&vertices, &stones, len, len);
        let (dist, _) = dijkstra(0, (len - 1, len - 1), &vertices, &neighbors);
        //println!("Distance: {}", dist[len * len - 1]);
        //println!("Last stone: {:?}", stones.last().unwrap());
        dist[len * len - 1] < u64::MAX
    });
    input[result]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(22, part1(&input, true));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!((6, 1), part2(&input, true));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(316, part1(&input, false));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!((45, 18), part2(&input, false));
    }
}
