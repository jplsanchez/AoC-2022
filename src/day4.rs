#![allow(dead_code)]
use crate::utils;
use models::Decoder;

mod models;

// const PATH: &str = "src/day4/sample.txt";
const PATH: &str = "src/day4/input.txt";

pub fn run_part_1() {
    let mut fully_contains = 0;

    for lines in utils::read_to_vector(PATH) {
        let pair = Decoder::decode(&lines);

        if pair.0.contains(&pair.1) || pair.1.contains(&pair.0) {
            fully_contains += 1;
        }
    }

    println!(
        "Number of pairs that one fully contains the other: {}",
        fully_contains
    );
}

pub fn run_part_2() {
    let mut overlaps = 0;

    for lines in utils::read_to_vector(PATH) {
        let pair = Decoder::decode(&lines);

        if pair.0.overlaps(&pair.1) {
            overlaps += 1;
        }
    }

    println!("Number of pairs that one overlaps the other: {}", overlaps);
}
