use aoc2024::common::read_input;

const DAY: u32 = 15;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

struct InitialMap {
    instructions: Vec<char>,
    boxes: Vec<(i64, i64)>,
    walls: Vec<(i64, i64)>,
    robot: (i64, i64),
}

// Parse input
fn process_input(input: Vec<String>) -> InitialMap {
    let mut instructions: Vec<char> = Vec::new();
    let mut boxes: Vec<(i64, i64)> = Vec::new();
    let mut walls: Vec<(i64, i64)> = Vec::new();
    let mut robot: (i64, i64) = (0, 0);
    for (y, row) in input.iter().enumerate() {
        let mut chars: Vec<char> = row.chars().collect();
        if chars.len() == 0 {
            continue;
        } else if chars[0] == '#' {
            // Map
            chars.iter().enumerate().for_each(|(x, &c)| {
                let x = x as i64;
                let y = y as i64;
                if c == '#' {
                    // Wall
                    walls.push((x, y));
                } else if c == 'O' {
                    // Box
                    boxes.push((x, y));
                } else if c == '@' {
                    // Robot
                    robot = (x, y);
                }
            });
        } else {
            // Instructions
            instructions.append(&mut chars);
        }
    }
    InitialMap {
        instructions,
        boxes,
        walls,
        robot,
    }
}

// Parse sample input
fn _sample_input() -> InitialMap {
    let data = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    process_input(data)
}

// Parse day's input
fn day_input() -> InitialMap {
    process_input(read_input(DAY))
}

fn move_robot(
    robot: (i64, i64),
    direction: char,
    boxes: &mut Vec<(i64, i64)>,
    walls: &Vec<(i64, i64)>,
) -> (i64, i64) {
    let (x, y) = robot;
    let change: (i64, i64) = match direction {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => (0, 0),
    };
    let proposed = (x + change.0, y + change.1);
    // Check walls
    if walls.contains(&proposed) {
        return robot;
    }
    // Check boxes
    if boxes.contains(&proposed) {
        let mut possible_box = (proposed.0 + change.0, proposed.1 + change.1);
        while !walls.contains(&possible_box) {
            // Not wall
            if !boxes.contains(&possible_box) {
                // Not box => we can move box there
                boxes.push(possible_box);
                let old_index = boxes.iter().position(|&r| r == proposed).unwrap();
                boxes.swap_remove(old_index);
                return proposed;
            }
            possible_box = (possible_box.0 + change.0, possible_box.1 + change.1);
        }
        // Wall reached, so robot can't move
        return robot;
    }
    proposed
}

fn convert_coordinates(old: (i64, i64)) -> (i64, i64) {
    let (x, y) = old;
    (2 * x, y)
}

fn move_robot2(
    robot: (i64, i64),
    direction: char,
    boxes: &mut Vec<(i64, i64)>,
    walls: &Vec<(i64, i64)>,
) -> (i64, i64) {
    let (x, y) = robot;
    let change: (i64, i64) = match direction {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => (0, 0),
    };
    let proposed = (x + change.0, y + change.1); // Directly ahead
    let proposed2 = (proposed.0 - 1, proposed.1);
    // Check walls
    if walls.contains(&proposed) || walls.contains(&proposed2) {
        return robot;
    }
    // Check boxes
    if boxes.contains(&proposed) || boxes.contains(&proposed2) {
        let box_l = if boxes.contains(&proposed) {
            proposed
        } else {
            proposed2
        };
        let (success, new_boxes) = move_box(box_l, change, boxes, walls, 0);
        if success {
            *boxes = new_boxes;
        } else {
            return robot;
        }
    }
    proposed
}

fn move_box(
    pos: (i64, i64),
    change: (i64, i64),
    boxes: &Vec<(i64, i64)>,
    walls: &Vec<(i64, i64)>,
    depth: u32,
) -> (bool, Vec<(i64, i64)>) {
    if depth > 10 {
        println!("Too deep recursion");
        return (false, Vec::new());
    }

    let proposed_l = (pos.0 + change.0, pos.1 + change.1);
    let proposed_r = (proposed_l.0 + 1, proposed_l.1);
    let proposed_ll = (proposed_l.0 - 1, proposed_l.1);

    if pos == (6, 4) {
        print!("");
    }
    //println!("Trying to move {:?} to {:?}", pos, proposed_l);

    let (success, mut boxes_proposed) = if walls.contains(&proposed_l)
        || walls.contains(&proposed_r)
        || walls.contains(&proposed_ll)
    {
        // Wall reached
        return (false, Vec::new());
    } else if boxes.contains(&proposed_l) {
        // Left half of proposed position has a box
        move_box(proposed_l, change, boxes, walls, depth + 1)
    } else if change.0 != 1 && boxes.contains(&proposed_ll) && boxes.contains(&proposed_r) {
        // Two boxes ahead (that would be pushed), only possible with vertical movement
        let (s1, b1) = move_box(proposed_ll, change, boxes, walls, depth + 1);
        let (s2, b2) = move_box(proposed_r, change, &b1, walls, depth + 1);
        (s1 && s2, b2)
    } else if change.0 != -1 && boxes.contains(&proposed_r) {
        // Right half of proposed position has a box
        move_box(proposed_r, change, boxes, walls, depth + 1)
    } else if change.0 != 1 && boxes.contains(&proposed_ll) {
        // Left half of proposed position has right half of a box (only possible on vertical movement)
        move_box(proposed_ll, change, boxes, walls, depth + 1)
    } else {
        (true, boxes.to_owned())
    };
    if success {
        // Success, so move this box
        let target = (pos.0 + change.0, pos.1 + change.1);
        let old_index = boxes_proposed.iter().position(|&r| r == pos);
        if let Some(i) = old_index {
            boxes_proposed.push(target);
            boxes_proposed.swap_remove(i);
        }
        (true, boxes_proposed)
    } else {
        (false, Vec::new())
    }
}

#[allow(dead_code)]
fn draw_map(boxes: &Vec<(i64, i64)>, walls: &Vec<(i64, i64)>, robot: (i64, i64)) {
    let max_x = walls
        .iter()
        .chain(boxes.iter())
        .map(|&(x, _)| x)
        .max()
        .unwrap_or(0)
        + 1;
    let max_y = walls
        .iter()
        .chain(boxes.iter())
        .map(|&(_, y)| y)
        .max()
        .unwrap_or(0);

    for y in 0..=max_y {
        for x in 0..=max_x {
            if (x, y) == robot {
                print!("@")
            } else if boxes.contains(&(x, y)) {
                print!("[");
            } else if boxes.contains(&(x - 1, y)) {
                print!("]");
            } else if walls.contains(&(x, y)) || walls.contains(&(x - 1, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part1(input: &InitialMap) -> i64 {
    let mut boxes = input.boxes.clone();
    let mut robot = input.robot;
    for &c in input.instructions.iter() {
        robot = move_robot(robot, c, &mut boxes, &input.walls);
    }
    boxes.iter().map(|(x, y)| y * 100 + x).sum()
}

fn part2(input: &InitialMap) -> i64 {
    let walls: Vec<(i64, i64)> = input
        .walls
        .iter()
        .map(|&old| convert_coordinates(old))
        .collect();
    let mut boxes: Vec<(i64, i64)> = input
        .boxes
        .iter()
        .map(|&old| convert_coordinates(old))
        .collect();
    let mut robot = convert_coordinates(input.robot);
    //draw_map(&boxes, &walls, robot);
    for &c in input.instructions.iter() {
        robot = move_robot2(robot, c, &mut boxes, &walls);
        //draw_map(&boxes, &walls, robot);
    }
    boxes.iter().map(|(x, y)| y * 100 + x).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        let input = process_input(input);
        assert_eq!(2028, part1(&input));
    }

    #[test]
    fn test_part1b() {
        let input = _sample_input();
        assert_eq!(10092, part1(&input));
    }

    #[test]
    fn test_part2a() {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        let input = process_input(input);
        part2(&input);
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(9021, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(1505963, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(1543141, part2(&input));
    }
}
