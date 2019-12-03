use crate::data::{Grid, Node};

mod data;

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.split(",").collect())
        .collect();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

// What is the Manhattan distance from the central port to the closest intersection?
fn part_1(wires: &Vec<Vec<&str>>) -> isize {
    let grid = Grid::new(&wires[0], &wires[1]);
    let mut closest: Option<Node> = None;

    for &node in grid.wire_intersections().iter() {
        if let Some(c) = closest {
            if (node.0.abs() + node.1.abs()) < (c.0.abs() + c.1.abs()) {
                closest = Some(*node);
            }
        } else {
            closest = Some(*node);
        }
    }

    let closest = closest.unwrap();
    closest.0.abs() + closest.1.abs()
}

// What is the fewest combined steps the wires must take to reach an intersection?
fn part_2(wires: &Vec<Vec<&str>>) -> usize {
    let grid = Grid::new(&wires[0], &wires[1]);
    let mut fewest_steps: Option<usize> = None;

    for node in grid.wire_intersections().iter() {
        let wire_0_steps = grid.nodes.get(node).unwrap()[0].unwrap();
        let wire_1_steps = grid.nodes.get(node).unwrap()[1].unwrap();
        let total_steps = wire_0_steps + wire_1_steps;

        if fewest_steps.is_none() || fewest_steps.unwrap() > total_steps {
            fewest_steps = Some(total_steps);
        }
    }

    fewest_steps.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(&vec![
                vec!["R8", "U5", "L5", "D3"],
                vec!["U7", "R6", "D4", "L4"],
            ]),
            6,
        );
        assert_eq!(
            part_1(&vec![
                vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
                vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
            ]),
            159,
        );
        assert_eq!(
            part_1(&vec![
                vec!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"],
                vec!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"],
            ]),
            135,
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(&vec![
                vec!["R8", "U5", "L5", "D3"],
                vec!["U7", "R6", "D4", "L4"],
            ]),
            30,
        );
        assert_eq!(
            part_2(&vec![
                vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
                vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
            ]),
            610,
        );
        assert_eq!(
            part_2(&vec![
                vec!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"],
                vec!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"],
            ]),
            410,
        );
    }
}
