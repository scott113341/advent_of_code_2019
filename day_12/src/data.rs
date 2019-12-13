use std::fmt::{Debug, Error, Formatter};

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Coordinate(pub [isize; 3]);

impl Coordinate {
    pub fn add(&mut self, velocity: &Velocity) {
        self.0[0] += velocity.0[0];
        self.0[1] += velocity.0[1];
        self.0[2] += velocity.0[2];
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!(
            "pos=<x={:2}, y={:2}, z={:2}>",
            self.0[0], self.0[1], self.0[2]
        ))
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Velocity(pub [isize; 3]);

impl Debug for Velocity {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!(
            "vel=<x={:2}, y={:2}, z={:2}>",
            self.0[0], self.0[1], self.0[2]
        ))
    }
}

pub fn find_axis_cycle(mut coordinates: [isize; 4], mut velocities: [isize; 4]) -> usize {
    let starting_c = coordinates.clone();
    let starting_v = velocities.clone();

    let mut step = 1;

    loop {
        // Calculate new velocities in-place
        for c1 in 0..=3 {
            for c2 in 0..=3 {
                if c1 == c2 {
                    continue;
                }

                velocities[c1] += match coordinates[c1] - coordinates[c2] {
                    d if d > 0 => -1,
                    d if d < 0 => 1,
                    _ => 0,
                }
            }
        }

        // Apply the velocities
        for i in 0..=3 {
            coordinates[i] += velocities[i];
        }

        // Check if back where we started
        if coordinates == starting_c && velocities == starting_v {
            return step;
        } else {
            step += 1;
        }
    }
}

pub fn total_energy(position: &Coordinate, velocity: &Velocity) -> isize {
    let pot_energy: isize = position.0.iter().map(|p| p.abs()).sum();
    let kin_energy: isize = velocity.0.iter().map(|v| v.abs()).sum();
    pot_energy * kin_energy
}
