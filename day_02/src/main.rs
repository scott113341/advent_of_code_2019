use crate::data::{Program, Memory};

mod data;

fn main() {
    let input: Vec<usize> = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

// Before running the program, replace position 1 with the value 12 and replace position 2 with the
// value 2. What value is left at position 0 after the program halts?
fn part_1(memory: &Memory) -> usize {
    let mut program = Program::new(memory.clone());
    program.memory[1] = 12;
    program.memory[2] = 2;
    program.run().memory[0]
}

// Find the input noun and verb that cause the program to produce the output 19690720. What
// is 100 * noun + verb? (For example, if noun=12 and verb=2, the answer would be 1202.) Each of the
// two input values will be between 0 and 99, inclusive.
fn part_2(memory: &Memory) -> usize {
    for input_1 in 0..=99 {
        for input_2 in 0..=99 {
            let mut program = Program::new(memory.clone());
            program.memory[1] = input_1;
            program.memory[2] = input_2;
            program = program.run();

            if program.memory[0] == 19690720 {
                return (100 * input_1) + input_2;
            }
        }
    }

    panic!("Did not find a pair of inputs resulting in 19690720");
}
