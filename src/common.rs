use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(day: u32) -> Vec<String> {
    let filename = format!("input/day{:02}.txt", day);
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}
