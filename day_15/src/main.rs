#![feature(entry_insert)]

use std::collections::HashSet;
use std::fmt;
use std::fmt::{Debug, Formatter};

use crate::intcode::{Memory, Program};

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

// Only four movement commands are understood: north (1), south (2), west (3), and east (4).
// The repair droid can reply with any of the following status codes:
//  0: The repair droid hit a wall
//  1: The repair droid moved one step
//  2: The repair droid moved one step; its new position is the location of the oxygen system
fn part_1(memory: Memory) -> isize {
    let program = Program::new(memory, None);
    walk_maze(program, WalkMode::Oxygen).0
}

// This is the same code, except we 1) start at the oxygen; 2) don't stop once we find the oxygen.
// Instead, we stop once there are no more nodes to visit, and return the current distance (which
// will be the largest).
fn part_2(memory: Memory) -> isize {
    // First, find the oxygen
    let program = Program::new(memory, None);
    let (_, program) = walk_maze(program, WalkMode::Oxygen);

    // Then, starting from the oxygen's location, return the largest distance
    walk_maze(program, WalkMode::Full).0
}

#[derive(Eq, PartialEq)]
enum WalkMode {
    Oxygen,
    Full,
}

fn walk_maze(mut program: Program, mode: WalkMode) -> (isize, Program) {
    let move_commands = [1, 2, 3, 4];
    let move_dirs = ['N', 'S', 'W', 'E'];
    let reverse_commands = [2, 1, 4, 3];
    let reverse_of = |cmd: &isize| -> isize {
        let idx = move_commands.iter().position(|c| c == cmd).unwrap();
        reverse_commands[idx]
    };

    let mut visited_coords: HashSet<Coord> = HashSet::new();
    let mut to_visit = vec![];
    to_visit.push(Node {
        y: 0,
        x: 0,
        distance: 0,
        directions: vec![],
    });

    loop {
        // DRIVE PHASE
        // Go from origin to the next node to visit
        let current_node = to_visit.remove(0);
        for cmd in current_node.directions.iter() {
            program.input.push(*cmd);
            program.run();
        }
        visited_coords.insert(Coord(current_node.y, current_node.x));

        // DISCOVERY PHASE
        // Find adjacent nodes to this one, and queue them up for visiting
        for (idx, cmd) in move_commands.iter().enumerate() {
            let dir = move_dirs[idx];
            program.input.push(*cmd);
            program.run();
            let output = program.output.last().unwrap().clone();

            match output {
                0 => {}
                1 | 2 => {
                    let dy = match dir {
                        'N' => 1,
                        'S' => -1,
                        _ => 0,
                    };
                    let dx = match dir {
                        'E' => 1,
                        'W' => -1,
                        _ => 0,
                    };

                    let distance = current_node.distance + 1;

                    let mut directions = current_node.directions.clone();
                    directions.push(*cmd);
                    assert_eq!(distance as usize, directions.len());

                    let node = Node {
                        y: current_node.y + dy,
                        x: current_node.x + dx,
                        distance,
                        directions,
                    };

                    // Queue up the adjacent node for search if we haven't already
                    if !visited_coords.contains(&Coord(node.y, node.x)) {
                        to_visit.push(node.clone());
                    }

                    // Move back to original location
                    let reverse_command = reverse_of(cmd);
                    program.input.push(reverse_command);
                    program.run();

                    // Check if we made it to the goal; if so, return the distance. This works fine
                    // because we're doing a breadth-first search.
                    if mode == WalkMode::Oxygen && output == 2 {
                        return (distance, program);
                    }
                }
                _ => panic!("Unsupported output: {}", output),
            }
        }

        // REVERSE PHASE
        // Go back to origin at (0,0)
        for cmd in current_node.directions.iter().rev() {
            program.input.push(reverse_of(cmd));
            program.run();
        }

        if mode == WalkMode::Full && to_visit.is_empty() {
            return (current_node.distance + 1, program);
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
struct Node {
    y: isize,
    x: isize,
    distance: isize,
    directions: Vec<isize>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Node {{ y={}, x={}, dist={} }}",
            self.y, self.x, self.distance
        )
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
struct Coord(isize, isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {}

    #[test]
    fn test_part_2() {}
}
