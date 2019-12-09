use itertools::Itertools;

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

fn part_1(memory: Memory) -> isize {
    let mut highest_output = 0;

    for combo in (0..=4).permutations(5) {
        let previous_amp_outputs = &mut vec![0];

        for phase_setting in combo {
            let mut input = vec![phase_setting];
            input.append(previous_amp_outputs);

            let program = &mut Program::new(memory.clone(), Some(input));
            program.run();
            *previous_amp_outputs = program.output.as_ref().unwrap().clone();
        }

        if previous_amp_outputs[0] > highest_output {
            highest_output = previous_amp_outputs[0];
        }
    }

    highest_output
}

fn part_2(memory: Memory) -> isize {
    let mut highest_output = 0;

    for combo in (5..=9).permutations(5) {
        let mut done = false;
        let mut amp_idx = 0;
        let mut previous_amp_outputs = vec![0];
        let mut final_amp_output = 0;

        let amps = &mut vec![
            Program::new(memory.clone(), Some(vec![*combo.get(0).unwrap()])),
            Program::new(memory.clone(), Some(vec![*combo.get(1).unwrap()])),
            Program::new(memory.clone(), Some(vec![*combo.get(2).unwrap()])),
            Program::new(memory.clone(), Some(vec![*combo.get(3).unwrap()])),
            Program::new(memory.clone(), Some(vec![*combo.get(4).unwrap()])),
        ];

        while !done {
            let amp = &mut amps[amp_idx];
            amp.input.as_mut().unwrap().append(&mut previous_amp_outputs);
            amp.run();

            if amp_idx == 4 && amp.exit_code == Some(99) {
                final_amp_output = amp.output.as_ref().unwrap()[0];
                done = true;
            } else {
                previous_amp_outputs = amp.output.clone().unwrap();
                amp.output = None;
                amp.exit_code = None;
                amp_idx = (amp_idx + 1) % 5;
            }
        }

        if final_amp_output > highest_output {
            highest_output = final_amp_output;
        }
    }

    highest_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            ]),
            43210,
        );
        assert_eq!(
            part_1(vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ]),
            54321,
        );
        assert_eq!(
            part_1(vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ]),
            65210,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(vec![
                3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5
            ]),
            139629729,
        );
        assert_eq!(
            part_2(vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ]),
            18216,
        );
    }
}
