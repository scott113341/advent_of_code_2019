use crate::data::{Memory, Program, IO};

mod data;

fn main() {
    let input: Vec<isize> = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part_1: {}", part_1(&input, Some(1)));
    println!("part_2: {}", part_2(&input, Some(5)));
}

fn part_1(memory: &Memory, input: IO) -> isize {
    let mut program = Program::new(memory.clone(), input);
    program = program.run();
    program.output.unwrap()
}

fn part_2(memory: &Memory, input: IO) -> isize {
    let mut program = Program::new(memory.clone(), input);
    program = program.run();
    program.output.unwrap()
}
