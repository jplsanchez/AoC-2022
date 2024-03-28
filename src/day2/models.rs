use core::panic;

pub struct InfoDecoder1 {
    pub player_shape: JokenpoShape,
    pub foe_shape: JokenpoShape,
}

impl InfoDecoder1 {
    pub fn from_string(string: &String) -> InfoDecoder1 {
        let chars: Vec<char> = string.chars().collect();
        InfoDecoder1 {
            player_shape: JokenpoShape::from_xyz_encryption(chars[2]),
            foe_shape: JokenpoShape::from_abc_encryption(chars[0]),
        }
    }

    fn is_player_winner(&self) {}
}

pub struct InfoDecoder2 {
    pub foe_shape: JokenpoShape,
    pub expected_result: JokenpoResult,
}

impl InfoDecoder2 {
    pub fn from_string(string: &String) -> InfoDecoder2 {
        let chars: Vec<char> = string.chars().collect();
        InfoDecoder2 {
            foe_shape: JokenpoShape::from_abc_encryption(chars[0]),
            expected_result: JokenpoResult::from_xyz_encryption(chars[2]),
        }
    }

    fn is_player_winner(&self) {}
}

#[derive(Clone)]
pub enum JokenpoShape {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl JokenpoShape {
    fn from_abc_encryption(data: char) -> JokenpoShape {
        match data {
            'A' => JokenpoShape::Rock,
            'B' => JokenpoShape::Paper,
            'C' => JokenpoShape::Scissors,
            other => panic!("Encryption '{}'is not mapped", other),
        }
    }

    fn from_xyz_encryption(data: char) -> JokenpoShape {
        match data {
            'X' => JokenpoShape::Rock,
            'Y' => JokenpoShape::Paper,
            'Z' => JokenpoShape::Scissors,
            other => panic!("Encryption '{}'is not mapped", other),
        }
    }

    pub fn is_winning_against(&self, other: &JokenpoShape) -> JokenpoResult {
        match self {
            JokenpoShape::Rock => match other {
                JokenpoShape::Rock => JokenpoResult::Draw,
                JokenpoShape::Paper => JokenpoResult::Lose,
                JokenpoShape::Scissors => JokenpoResult::Win,
            },
            JokenpoShape::Paper => match other {
                JokenpoShape::Rock => JokenpoResult::Win,
                JokenpoShape::Paper => JokenpoResult::Draw,
                JokenpoShape::Scissors => JokenpoResult::Lose,
            },
            JokenpoShape::Scissors => match other {
                JokenpoShape::Rock => JokenpoResult::Lose,
                JokenpoShape::Paper => JokenpoResult::Win,
                JokenpoShape::Scissors => JokenpoResult::Draw,
            },
        }
    }

    pub fn to_points(&self) -> i32 {
        match self {
            JokenpoShape::Rock => 1,
            JokenpoShape::Paper => 2,
            JokenpoShape::Scissors => 3,
        }
    }

    pub fn from_result_and_foe_shape(
        foe_shape: &JokenpoShape,
        result: &JokenpoResult,
    ) -> JokenpoShape {
        match result {
            JokenpoResult::Win => match foe_shape {
                JokenpoShape::Rock => JokenpoShape::Paper,
                JokenpoShape::Paper => JokenpoShape::Scissors,
                JokenpoShape::Scissors => JokenpoShape::Rock,
            },
            JokenpoResult::Lose => match foe_shape {
                JokenpoShape::Rock => JokenpoShape::Scissors,
                JokenpoShape::Paper => JokenpoShape::Rock,
                JokenpoShape::Scissors => JokenpoShape::Paper,
            },
            JokenpoResult::Draw => foe_shape.clone(),
        }
    }
}

pub enum JokenpoResult {
    Win,
    Lose,
    Draw,
}

impl JokenpoResult {
    pub fn to_points(&self) -> i32 {
        match self {
            JokenpoResult::Win => 6,
            JokenpoResult::Lose => 0,
            JokenpoResult::Draw => 3,
        }
    }

    fn from_xyz_encryption(data: char) -> JokenpoResult {
        match data {
            'Z' => JokenpoResult::Win,
            'X' => JokenpoResult::Lose,
            'Y' => JokenpoResult::Draw,
            _ => panic!("Encryption '{}' is not mapped", data),
        }
    }
}
