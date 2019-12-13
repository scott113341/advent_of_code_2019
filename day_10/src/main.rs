use crate::data::{quadrant, reduce_fraction, Asteroids, Point};
use std::collections::BTreeSet;

mod data;

fn main() {
    let input = include_str!("input.txt").trim();
    let asteroids = Asteroids::parse(input);

    println!("part_1: {}", part_1(asteroids.clone()));
    println!(
        "part_2: {}",
        part_2(asteroids.clone(), Point { x: 17, y: 22 }, 200)
    );
}

fn part_1(asteroids: Asteroids) -> usize {
    let mut best = (0, Point { y: 0, x: 0 });

    // For each asteroid
    for asteroid in asteroids.0.iter() {
        let mut line_of_sight_count = 0;

        // Check every other asteroid
        for test_asteroid in asteroids.0.iter() {
            if asteroid == test_asteroid {
                continue;
            }

            // Compute the reduced slope (2/10) => (1/5)
            let dx = test_asteroid.x - asteroid.x;
            let dy = test_asteroid.y - asteroid.y;
            let slope = reduce_fraction(dx, dy);

            let mut has_line_of_sight = true;
            let mut current_point = Point {
                x: asteroid.x + slope.0,
                y: asteroid.y + slope.1,
            };

            // Travel out towards the asteroid, but if we encounter a different asteroid first,
            // we know it's not visible
            while &current_point != test_asteroid {
                if asteroids.0.contains(&current_point) {
                    has_line_of_sight = false;
                    break;
                }

                current_point.x += slope.0;
                current_point.y += slope.1;
            }

            if has_line_of_sight {
                line_of_sight_count += 1;
            }
        }

        // Keep track of the asteroid with the most visible
        if line_of_sight_count > best.0 {
            best = (line_of_sight_count, asteroid.clone());
        }
    }

    best.0
}

// Which will be the 200th asteroid to be vaporized; what do you get if you multiply its X
// coordinate by 100 and then add its Y coordinate? (For example, 8,2 becomes 802.)
fn part_2(mut asteroids: Asteroids, station: Point, nth_asteroid: usize) -> isize {
    // Build a set of slopes (lowest common denominator)
    let mut unique_slopes = BTreeSet::new();
    for asteroid in asteroids.0.iter() {
        if asteroid == &station {
            continue;
        }

        let dx = asteroid.x - station.x;
        let dy = asteroid.y - station.y;
        let slope = reduce_fraction(dx, dy);
        unique_slopes.insert(slope);
    }

    // Sort slopes by "clockwise" order
    let mut slopes: Vec<&(isize, isize)> = unique_slopes.iter().collect();

    slopes.sort_by(|(adx, ady), (bdx, bdy)| {
        let adx = *adx as f64;
        let ady = *ady as f64;
        let bdx = *bdx as f64;
        let bdy = *bdy as f64;

        let a_atan = -1.0 * (adx / ady).atan();
        let b_atan = -1.0 * (bdx / bdy).atan();
        let a_quad = quadrant(adx, ady);
        let b_quad = quadrant(bdx, bdy);

        (a_quad, a_atan).partial_cmp(&(b_quad, b_atan)).unwrap()
    });

    // Keep track of vaporized asteroids
    let mut vaporized = Vec::new();
    let mut idx = 0;

    while vaporized.len() < nth_asteroid {
        // Can make multiple clockwise sweeps with our slopes
        idx = idx % slopes.len();

        // Grab the slope
        let slope = slopes[idx];
        let mut current_point = Point {
            x: station.x + slope.0,
            y: station.y + slope.1,
        };

        // Push outwards from the station along the slope, vaporizing the first asteroid we
        // encounter (and breaking if we've gone too far).
        loop {
            if asteroids.0.contains(&current_point) {
                asteroids.0.remove(&current_point);
                vaporized.push(current_point);
                break;
            }

            if current_point.x.abs() > 10_000 || current_point.y.abs() > 10_000 {
                break;
            }

            current_point.x += slope.0;
            current_point.y += slope.1;
        }

        idx += 1;
    }

    // Return the computed value from the last asteroid's coordinates
    vaporized[nth_asteroid - 1].x * 100 + vaporized[nth_asteroid - 1].y
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_1() -> Asteroids {
        Asteroids::parse(
            "
            .#..#
            .....
            #####
            ....#
            ...##
            "
            .trim(),
        )
    }

    fn example_2() -> Asteroids {
        Asteroids::parse(
            "
            ......#.#.
            #..#.#....
            ..#######.
            .#.#.###..
            .#..#.....
            ..#....#.#
            #..#....#.
            .##.#..###
            ##...#..#.
            .#....####
            "
            .trim(),
        )
    }

    fn example_3() -> Asteroids {
        Asteroids::parse(
            "
            #.#...#.#.
            .###....#.
            .#....#...
            ##.#.#.#.#
            ....#.#.#.
            .##..###.#
            ..#...##..
            ..##....##
            ......#...
            .####.###.
            "
            .trim(),
        )
    }

    fn example_4() -> Asteroids {
        Asteroids::parse(
            "
            .#..#..###
            ####.###.#
            ....###.#.
            ..###.##.#
            ##.##.#.#.
            ....###..#
            ..#.#..#.#
            #..#.#.###
            .##...##.#
            .....#.#..
            "
            .trim(),
        )
    }

    fn example_5() -> Asteroids {
        Asteroids::parse(
            "
            .#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##
            "
            .trim(),
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(example_1()), 8);
        assert_eq!(part_1(example_2()), 33);
        assert_eq!(part_1(example_3()), 35);
        assert_eq!(part_1(example_4()), 41);
        assert_eq!(part_1(example_5()), 210);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(example_5(), Point { x: 11, y: 13 }, 1), 1112);
        assert_eq!(part_2(example_5(), Point { x: 11, y: 13 }, 2), 1201);
        assert_eq!(part_2(example_5(), Point { x: 11, y: 13 }, 3), 1202);
        assert_eq!(part_2(example_5(), Point { x: 11, y: 13 }, 10), 1208);
        assert_eq!(part_2(example_5(), Point { x: 11, y: 13 }, 20), 1600);
        assert_eq!(part_2(example_5(), Point { x: 11, y: 13 }, 50), 1609);
        assert_eq!(part_2(example_5(), Point { x: 11, y: 13 }, 100), 1016);
        assert_eq!(part_2(example_5(), Point { x: 11, y: 13 }, 199), 906);
        assert_eq!(part_2(example_5(), Point { x: 11, y: 13 }, 200), 802);
        assert_eq!(part_2(example_5(), Point { x: 11, y: 13 }, 201), 1009);
        assert_eq!(part_2(example_5(), Point { x: 11, y: 13 }, 201), 1009);
        assert_eq!(part_2(example_5(), Point { x: 11, y: 13 }, 299), 1101);
    }
}
