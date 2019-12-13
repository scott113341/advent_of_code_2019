use crate::data::{find_axis_cycle, total_energy, Coordinate, Velocity};
use num::Integer;
use text_io::{scan, try_scan};

mod data;

fn main() {
    let input: Vec<Coordinate> = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|line| {
            let x: isize;
            let y: isize;
            let z: isize;
            scan!(line.bytes() => "<x={}, y={}, z={}>", x, y, z);
            Coordinate([x, y, z])
        })
        .collect();

    println!("part_1: {}", part_1(input.clone(), 1000));
    println!("part_2: {}", part_2(input.clone()));
}

// Then, it might help to calculate the total energy in the system. The total energy for a single
// moon is its potential energy multiplied by its kinetic energy. A moon's potential energy is the
// sum of the absolute values of its x, y, and z position coordinates. A moon's kinetic energy is
// the sum of the absolute values of its velocity coordinates.
fn part_1(coords: Vec<Coordinate>, n_steps: usize) -> isize {
    let mut positions = [
        coords[0].clone(),
        coords[1].clone(),
        coords[2].clone(),
        coords[3].clone(),
    ];
    let mut velocities = [
        Velocity([0, 0, 0]),
        Velocity([0, 0, 0]),
        Velocity([0, 0, 0]),
        Velocity([0, 0, 0]),
    ];

    for _step in 1..=n_steps {
        // Calculate new velocities in-place
        for i in 0..=3 {
            for j in 0..=3 {
                if i == j {
                    continue;
                }

                for k in 0..=2 {
                    velocities[i].0[k] += match positions[i].0[k] - positions[j].0[k] {
                        d if d > 0 => -1,
                        d if d < 0 => 1,
                        _ => 0,
                    }
                }
            }
        }

        // Apply the velocities
        for i in 0..=3 {
            positions[i].add(&velocities[i]);
        }
    }

    let mut energy = 0;
    for i in 0..=3 {
        energy += total_energy(&positions[i], &velocities[i]);
    }
    energy
}

fn part_2(coordinates: Vec<Coordinate>) -> usize {
    let coords = [
        coordinates[0].clone(),
        coordinates[1].clone(),
        coordinates[2].clone(),
        coordinates[3].clone(),
    ];

    let mut axis_cycle_times = vec![];

    for i in 0..=2 {
        axis_cycle_times.push(find_axis_cycle(
            [
                coords[0].0[i],
                coords[1].0[i],
                coords[2].0[i],
                coords[3].0[i],
            ],
            [0, 0, 0, 0],
        ));
    }

    axis_cycle_times[0]
        .lcm(&axis_cycle_times[1])
        .lcm(&axis_cycle_times[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_1() -> Vec<Coordinate> {
        vec![
            Coordinate([-1, 0, 2]),
            Coordinate([2, -10, -7]),
            Coordinate([4, -8, 8]),
            Coordinate([3, 5, -1]),
        ]
    }

    fn example_2() -> Vec<Coordinate> {
        vec![
            Coordinate([-8, -10, 0]),
            Coordinate([5, 5, 10]),
            Coordinate([2, -7, 3]),
            Coordinate([9, -8, -3]),
        ]
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(example_1(), 10), 179);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(example_1()), 2772);
        assert_eq!(part_2(example_2()), 4_686_774_924);
    }
}
