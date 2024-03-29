#![allow(dead_code)]
use crate::utils;
use models::Data;

mod models;

// const PATH: &str = "src/day5/sample.txt";
const PATH: &str = "src/day5/input.txt";

pub fn run_part_1() {
    let data = Data::decode(utils::read_to_vector(PATH));

    let mut stacks = data.stacks.clone();

    for command in data.commands {
        let source_stack = &mut stacks[command.source as usize - 1];
        let mut buffer: Vec<char> = Vec::new();

        // Move the elements from source to buffer
        for _ in 0..command.quantity {
            buffer.push(source_stack.pop().unwrap());
        }

        // Move the elements from buffer to target
        let target_stack = &mut stacks[command.target as usize - 1];

        for _ in 0..command.quantity {
            target_stack.append(&mut buffer);
        }
    }

    print!(" The top elements of the stacks are: ");
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!()
}

pub fn run_part_2() {
    let data = Data::decode(utils::read_to_vector(PATH));

    let mut stacks = data.stacks.clone();

    for command in data.commands {
        let source_stack = &mut stacks[command.source as usize - 1];

        // Move the elements from source to buffer
        let mut buffer = source_stack
            .drain(source_stack.len() - command.quantity..)
            .collect::<Vec<char>>();

        // Move the elements from buffer to target
        let target_stack = &mut stacks[command.target as usize - 1];
        target_stack.append(&mut buffer);
    }

    print!(" The top elements of the stacks are: ");
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!()
}
