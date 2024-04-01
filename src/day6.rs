mod part1_tests;
mod part2_tests;

use std::collections::HashMap;

const PATH: &str = "src/day6/input.txt";

pub fn run_part_1() {
    let data = std::fs::read_to_string(PATH).unwrap();
    println!(
        "The first marker is after charracter {}",
        start_of_pack(&data, 4)
    );
}

pub fn run_part_2() {
    let data = std::fs::read_to_string(PATH).unwrap();
    println!(
        "The first marker is after charracter {}",
        start_of_pack(&data, 14)
    );
}

pub fn start_of_pack(data: &str, pack_size: usize) -> usize {
    for (i, _) in data.chars().enumerate() {
        let slice = &data[i..i + pack_size];
        if slice.only_contains_unique_chars() {
            return i + pack_size;
        }
    }

    panic!("No marker found!");
}

trait Day6Utils {
    fn only_contains_unique_chars(&self) -> bool;
}

impl Day6Utils for str {
    fn only_contains_unique_chars(&self) -> bool {
        let mut map = HashMap::new();
        for c in self.chars() {
            let key = map.entry(c).or_insert(0);
            *key += 1;
            if *key > 1 {
                return false;
            }
        }
        true
    }
}
