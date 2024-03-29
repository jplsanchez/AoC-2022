#![allow(dead_code)]
use std::collections::HashSet;

// const PATH: &str = "src/day3/sample.txt";
const PATH: &str = "src/day3/input.txt";

pub fn run_part_1() {
    let mut priorities_sum = 0;

    for rucksack in crate::utils::read_to_vector(PATH) {
        let rucksack: Vec<i32> = rucksack.chars().map(char_to_priority).collect();

        let compartment1 = rucksack
            .iter()
            .take(rucksack.len() / 2)
            .cloned()
            .collect::<HashSet<i32>>();
        let compartment2 = rucksack
            .iter()
            .skip(rucksack.len() / 2)
            .cloned()
            .collect::<HashSet<i32>>();

        priorities_sum += compartment1
            .intersection(&compartment2)
            .cloned()
            .next()
            .unwrap();
    }

    println!("Sum of repeated priorities: {}", priorities_sum);
}

pub fn run_part_2() {
    let mut priorities_sum = 0;
    let elves = crate::utils::read_to_vector(PATH);

    let elves = elves
        .iter()
        .map(|x| x.chars().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>();

    for i in (0..elves.len()).step_by(3) {
        priorities_sum += find_badge((&elves[i], &elves[i + 1], &elves[i + 2]));
    }

    println!("Sum of badges priorities: {}", priorities_sum);
}

fn find_badge(elves: (&HashSet<char>, &HashSet<char>, &HashSet<char>)) -> i32 {
    (elves.0)
        .intersection(&(elves.1))
        .cloned()
        .collect::<HashSet<char>>()
        .intersection(&(elves.2))
        .cloned()
        .next()
        .map(char_to_priority)
        .unwrap()
}

fn char_to_priority(item: char) -> i32 {
    match item {
        'a'..='z' => return item as i32 - 96,
        'A'..='Z' => return item as i32 - 38,
        _ => return 0,
    }
}
