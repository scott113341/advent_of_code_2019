#![feature(entry_insert)]

use crate::data::{Memory, Program};

mod data;

fn main() {
    let memory: Memory = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part_1: {}", part_1(memory.clone()));
    println!("part_2: {}", part_2(memory.clone()));
}

// Run it in test mode by providing it the value 1; it should only output a single value, the BOOST
// keycode. What BOOST keycode does it produce?
fn part_1(memory: Memory) -> isize {
    let mut program = Program::new(memory, Some(vec![1]));
    program.run();
    program.output[0]
}

fn part_2(memory: Memory) -> isize {
    let mut program = Program::new(memory, Some(vec![2]));
    program.run();
    program.output[0]
}
