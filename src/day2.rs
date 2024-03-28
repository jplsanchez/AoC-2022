#![allow(dead_code)]
mod models;

use crate::day2::models::JokenpoShape;
use crate::utils::read_to_vector;
use models::InfoDecoder1;
use models::InfoDecoder2;

// const PATH: &str = "src/day2/sample.txt";
const PATH: &str = "src/day2/input.txt";

pub fn run_part_1() {
    let mut total_score = 0;

    for line in read_to_vector(PATH) {
        let info = InfoDecoder1::from_string(&line);
        let match_result = info.player_shape.is_winning_against(&info.foe_shape);

        total_score += info.player_shape.to_points();
        total_score += match_result.to_points();
    }

    println!("Total score: {}", total_score);
}

pub fn run_part_2() {
    let mut total_score = 0;

    for line in read_to_vector(PATH) {
        let info = InfoDecoder2::from_string(&line);
        let player_shape =
            JokenpoShape::from_result_and_foe_shape(&info.foe_shape, &info.expected_result);

        total_score += player_shape.to_points();
        total_score += info.expected_result.to_points();
    }

    println!("Total score: {}", total_score);
}
