use std::collections::HashMap;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Node {
    Space,
    Scaffolding,
}

pub enum RoboOrientation {
    Up,
    Down,
    Left,
    Right,
    Tumble,
}

pub type Graph = HashMap<Coord, Node>;

pub struct Scaffold {
    pub graph: Graph,
    pub robo_coord: Coord,
    pub robo_orientation: RoboOrientation,
}

impl Scaffold {
    pub fn from_output(output: &Vec<isize>) -> Scaffold {
        use Node::*;
        use RoboOrientation::*;

        let mut graph: Graph = HashMap::new();
        let mut robo_coord = None;
        let mut robo_orientation = None;

        let mut x = 0;
        let mut y = 0;

        for char_code in output {
            print!("{}", (*char_code as u8) as char);

            let coord = Coord { x, y };

            match *char_code {
                10 => {
                    x = 0;
                    y += 1;
                }
                35 => {
                    graph.insert(coord, Scaffolding);
                },
                46 => {
                    graph.insert(coord, Space);
                },

                94 => {
                    graph.insert(coord, Scaffolding);
                    robo_coord = Some(coord);
                    robo_orientation = Some(Up);
                },
                118 => {
                    graph.insert(coord, Scaffolding);
                    robo_coord = Some(coord);
                    robo_orientation = Some(Down);
                },
                60 => {
                    graph.insert(coord, Scaffolding);
                    robo_coord = Some(coord);
                    robo_orientation = Some(Left);
                },
                62 => {
                    graph.insert(coord, Scaffolding);
                    robo_coord = Some(coord);
                    robo_orientation = Some(Right);
                },
                88 => {
                    graph.insert(coord, Space);
                    robo_coord = Some(coord);
                    robo_orientation = Some(Tumble);
                },
                _ => panic!("Unknown char_code: {}", char_code),
            }

            if *char_code != 10 {
                x += 1;
            }
        }

        Scaffold {
            graph,
            robo_coord: robo_coord.unwrap(),
            robo_orientation: robo_orientation.unwrap(),
        }
    }
}

// L,4
// R,8
// L,6
// L,10
// L,6
// R,8
// R,10
// L,6
// L,6
// L,4
// R,8
// L,6
// L,10
// L,6
// R,8
// R,10
// L,6
// L,6
// L,4
// L,4
// L,10
// L,4
// L,4
// L,10
// L,6
// R,8
// R,10
// L,6
// L,6
// L,4
// R,8
// L,6
// L,10
// L,6
// R,8
// R,10
// L,6
// L,6
// L,4
// L,4
// L,10
