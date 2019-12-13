use std::collections::BTreeSet;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Clone, Debug)]
pub struct Asteroids(pub BTreeSet<Point>);

impl Asteroids {
    pub fn parse(input: &str) -> Asteroids {
        let mut asteroids = BTreeSet::new();

        for (y, line) in input.split("\n").enumerate() {
            for (x, datum) in line.trim().chars().enumerate() {
                match datum {
                    '.' => continue,
                    '#' => {
                        let point = Point {
                            x: x as isize,
                            y: y as isize,
                        };
                        asteroids.insert(point);
                    }
                    _ => panic!("Unknown datum: {}", datum),
                }
            }
        }

        Asteroids(asteroids)
    }
}

pub fn quadrant(dx: f64, dy: f64) -> usize {
    if dx >= 0.0 && dy < 0.0 {
        1
    } else if dx >= 0.0 && dy >= 0.0 {
        2
    } else if dx <= 0.0 && dy >= 0.0 {
        3
    } else if dx < 0.0 && dy < 0.0 {
        4
    } else {
        panic!("{}, {} is rico", dx, dy);
    }
}

pub fn reduce_fraction(num: isize, denom: isize) -> (isize, isize) {
    let num_abs = num.abs();
    let num_sign = if num == num_abs { 1 } else { -1 };
    let denom_abs = denom.abs();
    let denom_sign = if denom == denom_abs { 1 } else { -1 };

    let gcd = gcd(num_abs, denom_abs);
    (num_abs / gcd * num_sign, denom_abs / gcd * denom_sign)
}

fn gcd(a: isize, b: isize) -> isize {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}
