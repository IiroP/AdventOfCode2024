use std::{collections::HashSet, usize};

use aoc2024::common::read_input;
use itertools::Itertools;

const DAY: u32 = 23;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: Vec<String>) -> (Vec<Vertex>, Vec<(usize, usize)>) {
    let mut vertices: Vec<Vertex> = Vec::new();

    fn pos_or_insert(value: &str, vertices: &mut Vec<Vertex>) -> usize {
        let value_tuple = (value.chars().nth(0).unwrap(), value.chars().nth(1).unwrap());
        vertices
            .iter()
            .position(|x| x.name.0 == value_tuple.0 && x.name.1 == value_tuple.1)
            .unwrap_or_else(||{
                vertices.push(Vertex {
                    name: value_tuple,
                });
                vertices.len() - 1
            })
    }

    let edges: Vec<(usize, usize)> = input
        .iter()
        .flat_map(|s| {
            let splitted: Vec<&str> = s.split("-").collect();
            let left = pos_or_insert(splitted[0], &mut vertices);
            let right = pos_or_insert(splitted[1], &mut vertices);
            [(left, right), (right, left)] // undirected graph
        })
        .collect();

    (vertices, edges)
}

// Parse sample input
fn _sample_input() -> (Vec<Vertex>, Vec<(usize, usize)>) {
    let data = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> (Vec<Vertex>, Vec<(usize, usize)>) {
    process_input(read_input(DAY))
}

#[derive(Clone, Copy)]
struct Vertex {
    name: (char, char),
}

// Find 3-cliques
fn find_cliques(edges: &Vec<(usize, usize)>) -> Vec<[usize; 3]> {
    let mut cliques: Vec<[usize; 3]> = edges
        .iter()
        .flat_map(|&(v0, v1)| {
            edges
                .iter()
                .filter(|&e| {
                    e.0 == v0
                        && e.1 != v1
                        && edges.iter().find(|&e1| e1.0 == e.1 && e1.1 == v1).is_some()
                })
                .map(|e| [v0, v1, e.1])
                .collect::<Vec<_>>()
        })
        .collect();
    cliques.iter_mut().for_each(|c| c.sort());
    cliques.iter().unique().cloned().collect()
}

// Extend clique with some vertex
fn extend_clique(
    current: &Vec<usize>,
    vertices: &Vec<Vertex>,
    edges: &Vec<(usize, usize)>,
) -> Option<usize> {
    (0..vertices.len()).find(|&i| {
        !current.contains(&i) // not already in the clique
            && current.iter().all(|&c| { 
                // all vertices in the clique are connected to the new vertex
                edges.iter().find(|&e| e.0 == c && e.1 == i).is_some()
            })
    })
}

fn part1(input: &(Vec<Vertex>, Vec<(usize, usize)>)) -> usize {
    let (vertices, edges) = input;
    let t_vertices = vertices
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if v.name.0 == 't' { Some(i) } else { None })
        .collect::<Vec<_>>();
    let cliques = find_cliques(edges);
    cliques
        .iter()
        .filter(|&c| c.iter().any(|v| t_vertices.contains(v)))
        .count()
}

fn part2(input: &(Vec<Vertex>, Vec<(usize, usize)>)) -> String {
    let (vertices, edges) = input;
    let mut explored: HashSet<usize> = HashSet::new();
    (0..vertices.len())
        .map(|i| {
            let mut clique = vec![i];
            while let Some(next) = extend_clique(&clique, vertices, edges) {
                clique.push(next);
            }
            explored.extend(clique.iter());
            clique
        })
        .max_by_key(|v| v.len())
        .unwrap()
        .iter()
        .map(|&i| format!("{}{}", vertices[i].name.0, vertices[i].name.1))
        .sorted()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper() {
        let input = day_input();
        let (vertices, edges) = &input;
        let current = vec![0];
        let result = extend_clique(&current, vertices, edges);
        println!("{:?}", result);
    }

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(7, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!("co,de,ka,ta", part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(1467, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!("di,gs,jw,kz,md,nc,qp,rp,sa,ss,uk,xk,yn", part2(&input));
    }
}
