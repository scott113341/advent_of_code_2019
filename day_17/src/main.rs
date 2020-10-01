#![feature(entry_insert)]

use crate::intcode::{Memory, Program};
use crate::data::{Scaffold, Coord, Node};

mod data;
mod intcode;

fn main() {
    let memory: Memory = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part_1: {}", part_1(memory.clone()));
    println!("part_2: {}", part_2(memory.clone()));
}

// Locate all scaffold intersections; for each, its alignment parameter is the distance between its
// left edge and the left edge of the view multiplied by the distance between its top edge and the
// top edge of the view. What is the sum of the alignment parameters for the scaffold intersections?
fn part_1(memory: Memory) -> isize {
    use Node::*;

    let mut alignment_param_sum = 0;

    let mut program = Program::new(memory, None);
    program.run();
    let scaffold = Scaffold::from_output(&program.output);

    for (coord, node) in scaffold.graph.iter() {
        let x = coord.x;
        let y = coord.y;

        // Skip Space nodes
        if node == &Space {
            continue;
        }

        let adjacent_nodes = [
            scaffold.graph.get(&Coord { x: x - 1, y }),
            scaffold.graph.get(&Coord { x: x + 1, y }),
            scaffold.graph.get(&Coord { x, y: y - 1 }),
            scaffold.graph.get(&Coord { x, y: y + 1 }),
        ];

        let is_intersection = adjacent_nodes.iter().all(|&n| n == Some(&Scaffolding));

        if is_intersection {
            alignment_param_sum += x * y;
        }
    }

    alignment_param_sum
}

// Visually, I figured out the commands that need to get sent to the robot. Unsurprisingly (given
// the constraint of 3 "functions" and 1 "main routine", each with a max of at most 20 characters,
// not counting the newline), there are some repeats, which I've annotated below.
//
//   L,4    ╦
//   R,8    ║ A
//   L,6    ║
//   L,10   ╩
//   L,6      ╦
//   R,8      ║
//   R,10     ║ B
//   L,6      ║
//   L,6      ╩
//   L,4    ╦
//   R,8    ║ A
//   L,6    ║
//   L,10   ╩
//   L,6      ╦
//   R,8      ║
//   R,10     ║ B
//   L,6      ║
//   L,6      ╩
//   L,4        ╦
//   L,4        ║ C
//   L,10       ╩
//   L,4        ╦
//   L,4        ║ C
//   L,10       ╩
//   L,6      ╦
//   R,8      ║
//   R,10     ║ B
//   L,6      ║
//   L,6      ╩
//   L,4    ╦
//   R,8    ║ A
//   L,6    ║
//   L,10   ╩
//   L,6      ╦
//   R,8      ║
//   R,10     ║ B
//   L,6      ║
//   L,6      ╩
//   L,4        ╦
//   L,4        ║ C
//   L,10       ╩
//
// Given this, it's fairly simple to instruct the robot to do what we want.
fn part_2(mut memory: Memory) -> isize {
    // "Force the vacuum robot to wake up by changing the value in your ASCII program at address 0
    // from 1 to 2"
    memory[0] = 2;

    // Initialize the program and run it to the first time it needs input
    let mut program = Program::new(memory, None);
    program.run();

    // Helper function to translate commands into ASCII integers
    let str_to_ascii = |str: &str| -> Vec<isize> {
        str.chars().map(|c| c as isize).collect()
    };

    // Main movement routine, and movement functions A/B/C
    program.input = str_to_ascii("A,B,A,B,C,C,B,A,B,C\n");
    program.run();
    program.input = str_to_ascii("L,4,R,8,L,6,L,10\n");
    program.run();
    program.input = str_to_ascii("L,6,R,8,R,10,L,6,L,6\n");
    program.run();
    program.input = str_to_ascii("L,4,L,4,L,10\n");
    program.run();

    // Continuous video feed? "y" or "n"
    program.input = str_to_ascii("n\n");

    // Run the program and return the output!
    program.run();
    *program.output.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
    }

    #[test]
    fn test_part_2() {
    }
}
