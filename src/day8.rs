#![allow(dead_code)]

mod tree_map;
mod visibility;

use tree_map::TreeMap;

// const PATH: &str = "src/day8/sample.txt";
const PATH: &str = "src/day8/input.txt";

pub(crate) fn run_part_1() {
    let data = crate::utils::read_to_vector(PATH);
    let map = TreeMap::new(data);

    println!(
        "The number of visible trees is: {}",
        map.count_visible_trees()
    );

    // map.print_visibility();
    // println!();
    // map.print_height();
}

pub(crate) fn run_part_2() {}
