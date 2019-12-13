#![feature(entry_insert)]

use crate::data::{Memory, Program};
use std::collections::BTreeMap;

mod data;

fn main() {
    let memory: Memory = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part_1: {}", part_1(memory.clone()));
    println!("part_2:");
    part_2(memory.clone());
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
struct Point(isize, isize);

type Panels = BTreeMap<Point, isize>;

struct Robot {
    position: Point,
    facing: Point,
}

// The Intcode program will serve as the brain of the robot. The program uses input instructions to
// access the robot's camera: provide 0 if the robot is over a black panel or 1 if the robot is over
// a white panel. Then, the program will output two values:
// - The color to paint the panel the robot is over: 0=black; 1=white
// - The direction the robot should turn: 0=left 90 degrees; 1=right 90 degrees
// After the robot turns, it move forward exactly one panel. The robot starts facing up.
fn part_1(memory: Memory) -> usize {
    let mut program = Program::new(memory, None);
    let mut panels: Panels = BTreeMap::new();
    let mut robot = Robot {
        position: Point(0, 0),
        facing: Point(0, 1),
    };

    run_robot(&mut program, &mut panels, &mut robot);

    panels.len()
}

// After starting the robot on a single white panel instead, what does it paint?
fn part_2(memory: Memory) {
    let mut program = Program::new(memory, None);
    let mut panels: Panels = BTreeMap::new();
    let mut robot = Robot {
        position: Point(0, 0),
        facing: Point(0, 1),
    };

    panels.insert(Point(0, 0), 1);
    run_robot(&mut program, &mut panels, &mut robot);

    let min_x = *panels.iter().map(|(Point(x, _y), _)| x).min().unwrap();
    let max_x = *panels.iter().map(|(Point(x, _y), _)| x).max().unwrap();
    let min_y = *panels.iter().map(|(Point(_x, y), _)| y).min().unwrap();
    let max_y = *panels.iter().map(|(Point(_x, y), _)| y).max().unwrap();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let color = panels
                .get(&Point(x, y))
                .or(Some(&0))
                .and_then(|c| if c == &0 { Some('▓') } else { Some('░') })
                .unwrap();
            print!("{}", color);
        }
        print!("\n");
    }
}

fn run_robot(program: &mut Program, panels: &mut Panels, robot: &mut Robot) {
    loop {
        let current_color = panels.get(&robot.position).unwrap_or(&0);
        program.input.push(*current_color);
        program.run();

        if program.exit_code == Some(99) {
            break;
        }

        // Save the paint color
        let paint_color = program.output[0];
        panels.insert(robot.position.clone(), paint_color);

        // Rotate the robot
        robot.facing = match (&robot.facing, program.output[1]) {
            // Up
            (Point(0, 1), 0) => Point(-1, 0),
            (Point(0, 1), 1) => Point(1, 0),

            // Right
            (Point(1, 0), 0) => Point(0, 1),
            (Point(1, 0), 1) => Point(0, -1),

            // Down
            (Point(0, -1), 0) => Point(1, 0),
            (Point(0, -1), 1) => Point(-1, 0),

            // Left
            (Point(-1, 0), 0) => Point(0, -1),
            (Point(-1, 0), 1) => Point(0, 1),

            // RIP
            _ => panic!(
                "Robot can't be facing {:?} and turn {}",
                &robot.facing, program.output[1]
            ),
        };

        // Move the robot
        robot.position.0 += robot.facing.0;
        robot.position.1 += robot.facing.1;

        // Wipe output
        program.output.clear();
    }
}
