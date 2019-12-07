use std::collections::{HashMap, HashSet};

type Planet<'a> = &'a str;
type Orbits<'a> = (Planet<'a>, Planet<'a>);
type Orbiters<'a> = HashSet<Planet<'a>>;

fn main() {
    let input: Vec<Orbits> = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| {
            let mut objects = s.split(")");
            (objects.nth(0).unwrap(), objects.nth(0).unwrap())
        })
        .collect();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(all_orbits: &Vec<Orbits>) -> usize {
    let mut orbits: HashMap<Planet, Orbiters> = HashMap::new();

    for (center, orbiter) in all_orbits {
        orbits
            .entry(center)
            .or_insert_with(|| HashSet::new())
            .insert(orbiter);
    }

    let mut orbit_count = 0;
    let mut depth = 1;
    let mut current_orbits = orbits.get("COM").unwrap().clone();

    while !current_orbits.is_empty() {
        let mut next_orbits = HashSet::new();

        for orbiter in current_orbits {
            orbit_count += depth;

            if let Some(next_orbiters) = orbits.get(orbiter) {
                for next_orbiter in next_orbiters {
                    next_orbits.insert(*next_orbiter);
                }
            }
        }

        depth += 1;
        current_orbits = next_orbits;
    }

    orbit_count
}

fn part_2(all_orbits: &Vec<Orbits>) -> usize {
    let mut connected_planets: HashMap<Planet, Orbiters> = HashMap::new();

    for (center, orbiter) in all_orbits {
        connected_planets
            .entry(center)
            .or_insert_with(|| HashSet::new())
            .insert(orbiter);

        connected_planets
            .entry(orbiter)
            .or_insert_with(|| HashSet::new())
            .insert(center);
    }

    let mut distance = 0;
    let mut current_orbits = connected_planets.get("YOU").unwrap().clone();
    let mut visited = HashSet::new();
    visited.insert("YOU");

    while !current_orbits.is_empty() {
        let mut next_orbits = HashSet::new();

        for orbiter in current_orbits {
            visited.insert(orbiter);

            if orbiter == "SAN" {
                return distance - 1;
            } else {
                // Add connected parent orbits
                if let Some(next_orbiters) = connected_planets.get(orbiter) {
                    for next_orbiter in next_orbiters {
                        if !visited.contains(next_orbiter) {
                            next_orbits.insert(*next_orbiter);
                        }
                    }
                }
            }
        }

        distance += 1;
        current_orbits = next_orbits;
    }

    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let orbits = vec![
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
        ];
        assert_eq!(part_1(&orbits), 42);
    }

    #[test]
    fn test_part_2() {
        let orbits = vec![
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
            ("K", "YOU"),
            ("I", "SAN"),
        ];
        assert_eq!(part_2(&orbits), 4);
    }
}
