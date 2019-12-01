fn main() {
    let input: Vec<isize> = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

// For each module, take its mass, divide by three, round down, and subtract 2 => then sum
fn part_1(masses: &Vec<isize>) -> isize {
    masses
        .iter()
        .map(|m| m / 3 - 2)
        .sum()
}

// Fuel requires fuel, same formula, disregard "negative mass" fuel
fn part_2(masses: &Vec<isize>) -> isize {
    let mut total_fuel = 0;

    for &mass in masses {
        let mut mass_needing_fuel = mass;

        // Stop adding "fuel for fuel" once it goes "negative mass"
        while mass_needing_fuel > 0 {
            mass_needing_fuel = mass_needing_fuel / 3 - 2;

            // Only add "positive mass" fuel
            if mass_needing_fuel > 0 {
                total_fuel += mass_needing_fuel;
            }
        }
    }

    total_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&vec![12]), 2);
        assert_eq!(part_1(&vec![14]), 2);
        assert_eq!(part_1(&vec![1969]), 654);
        assert_eq!(part_1(&vec![100756]), 33583);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&vec![14]), 2);
        assert_eq!(part_2(&vec![1969]), 966);
        assert_eq!(part_2(&vec![100756]), 50346);
    }
}
