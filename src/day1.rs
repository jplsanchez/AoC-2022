// Each line represents an quantity of calories that a elf is carring
// if the next line is not blank it belongs to the same elf
// if the line is blank then is the next elf
#![allow(dead_code)]

mod models;
use models::CaloriesRank;
use models::Elf;

// const PATH: &str = "./src/day1/sample.txt";
const PATH: &str = "./src/day1/input.txt";

pub fn run_part_1() {
    let mut most_calories: i32 = 0;
    let mut elf = Elf::new();

    for line in crate::utils::read_to_vector(PATH) {
        match line.as_str() {
            "" => {
                if elf.calories > most_calories {
                    most_calories = elf.calories;
                };
                elf = Elf::new();
            }
            line => {
                elf.calories += line.parse().unwrap_or(0);
            }
        }
    }

    println!("The elf with most calories has {} calories", most_calories);
}

pub fn run_part_2() {
    let mut top3 = CaloriesRank { rank: [0, 0, 0] };
    let mut elf = Elf::new();

    for line in crate::utils::read_to_vector(PATH) {
        match line.as_str() {
            "" => {
                top3.try_add_to_rank(elf.calories);
                elf = Elf::new();
            }
            line => {
                elf.calories += line.parse().unwrap_or(0);
            }
        }
    }

    // give the last elf a try, since there is no blank line in the end
    top3.try_add_to_rank(elf.calories);

    println!(
        "The top 3 calories are: {}, {}, {}. And the sum of then is: {}",
        top3.rank[0],
        top3.rank[1],
        top3.rank[2],
        top3.sum_rank()
    )
}
