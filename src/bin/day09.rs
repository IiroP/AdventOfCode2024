use std::{cmp::min, iter::repeat};

use aoc2024::common::read_input;

const DAY: u32 = 9;

pub fn main() {
    let input = day_input();
    let part1_result = part1(&input);
    println!("Day {DAY}, part 1: {part1_result}");
    let part2_result = part2(&input);
    println!("Day {DAY}, part 2: {part2_result}");
}

// Parse input
fn process_input(input: String) -> (Vec<i64>, Vec<(i64, i64)>, i64) {
    let mut current_file: i64 = 0;
    let mut empty: Vec<i64> = Vec::new();
    let mut files: Vec<(i64, i64)> = Vec::new();
    let mut total_len: i64 = 0;
    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            // File
            let file_len = c.to_digit(10).unwrap() as i64;
            files.push((current_file, file_len));
            current_file += 1;
            total_len += file_len;
        } else {
            // Empty
            empty.push(c.to_digit(10).unwrap() as i64);
        }
    }
    // First vector contains empty slot lengths, second vector has (id, len) tuples
    (empty, files, total_len)
}

// Parse sample input
fn _sample_input() -> (Vec<i64>, Vec<(i64, i64)>, i64) {
    let data = "2333133121414131402".to_string();
    process_input(data)
}

// Parse day's input
fn day_input() -> (Vec<i64>, Vec<(i64, i64)>, i64) {
    let data = read_input(DAY)[0].clone();
    process_input(data)
}

fn part1(input: &(Vec<i64>, Vec<(i64, i64)>, i64)) -> i64 {
    let (empty, files, total) = input;

    // Update checksum with file on original position
    fn take_left_file(
        file: &(i64, i64),
        current_index: &mut i64,
        checksum: &mut i64,
        max_chars: i64,
    ) {
        let (id, len) = file;
        (0..min(*len, max_chars)).for_each(|_| {
            *checksum += id * *current_index;
            *current_index += 1;
        });
    }

    // Update checksum with filler
    fn fill_hole(
        hole: i64,
        current_index: &mut i64,
        checksum: &mut i64,
        filler_iter: &mut dyn Iterator<Item = i64>,
        max_chars: i64,
    ) {
        (0..min(hole, max_chars)).for_each(|_| {
            *checksum += filler_iter.next().unwrap() * *current_index;
            *current_index += 1;
        });
    }

    // Generate filler numbers (file content in reverse order)
    fn generate_numbers_rev(files: &Vec<(i64, i64)>) -> impl Iterator<Item = i64> + '_ {
        files
            .iter()
            .rev()
            .flat_map(|&(number, len)| repeat(number).take(len as usize))
    }

    let mut current_index: i64 = 0;
    let mut left_iter = files.iter();
    let mut hole_iter = empty.iter();
    let mut filler_iter = generate_numbers_rev(files);

    let mut checksum: i64 = 0;
    while current_index < *total {
        let left = left_iter.next().unwrap();
        let max_chars = *total - current_index;
        take_left_file(left, &mut current_index, &mut checksum, max_chars);
        let hole = hole_iter.next().unwrap();
        let max_chars = *total - current_index;
        fill_hole(
            *hole,
            &mut current_index,
            &mut checksum,
            &mut filler_iter,
            max_chars,
        );
    }
    checksum
}

// Not very well optimized, but still ~3s with dev build and ~instantly with release build
fn part2(input: &(Vec<i64>, Vec<(i64, i64)>, i64)) -> i64 {
    let (empty, files, _) = input;

    let mut current_index: i64 = 0;
    let mut hole_iter = empty.iter();
    let files = files
        .iter()
        .map(|(id, len)| {
            let start = current_index;
            current_index += *len;
            let end = current_index; //exclusive
            current_index += *hole_iter.next().unwrap_or(&0);
            (start, end, *len, *id)
        })
        .collect::<Vec<(i64, i64, i64, i64)>>();

    // Move file as far left as possible
    fn move_file(file: &(i64, i64, i64, i64), files: &mut Vec<(i64, i64, i64, i64)>) {
        let result = files
            .windows(2)
            .find(|pair| {
                let (_, _, len, _) = file;
                let (_, prev_end, _, _) = pair[0];
                let (next_start, _, _, _) = pair[1];
                next_start - prev_end >= *len
            })
            .map(|pair| pair[0].1);
        if let Some(hole_start) = result {
            if hole_start >= file.0 {
                // Only move left
                return;
            }
            // Change file start and end
            let index = files.iter().position(|f| f == file).unwrap();
            let _ = std::mem::replace(
                &mut files[index],
                (hole_start, hole_start + file.2, file.2, file.3),
            );
        }
        files.sort_by(|a, b| a.0.cmp(&b.0));
        // Alternatively insert after specific file and remove the old value
    }

    let mut new_files = files.clone();
    files
        .iter()
        .rev()
        .for_each(|file| move_file(file, &mut new_files));

    new_files
        .iter()
        .map(|(start, end, _, value)| (*start..*end).map(|i| i * value).sum::<i64>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = _sample_input();
        assert_eq!(1928, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = _sample_input();
        assert_eq!(2858, part2(&input));
    }

    #[test]
    fn real_part1() {
        let input = day_input();
        assert_eq!(6331212425418, part1(&input));
    }

    #[test]
    fn real_part2() {
        let input = day_input();
        assert_eq!(6363268339304, part2(&input));
    }
}
