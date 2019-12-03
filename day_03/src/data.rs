use std::collections::HashMap;

pub type Node = (isize, isize);

#[derive(Debug)]
pub struct Grid {
    // Value means "has been visited by wire[idx], and if so, how many steps?"
    pub nodes: HashMap<Node, [Option<usize>; 2]>,
}

impl Grid {
    pub fn new(wire_0: &Vec<&str>, wire_1: &Vec<&str>) -> Grid {
        let mut grid = Grid {
            nodes: HashMap::new(),
        };

        for (wire_index, &wire) in [wire_0, wire_1].iter().enumerate() {
            let mut current = (0, 0);
            let mut wire_step_count = 0;

            for segment in wire {
                let (dir, count) = segment.split_at(1);
                let count = count.parse().unwrap();

                for _ in 0..count {
                    current = match dir {
                        "U" => (current.0, current.1 + 1),
                        "R" => (current.0 + 1, current.1),
                        "D" => (current.0, current.1 - 1),
                        "L" => (current.0 - 1, current.1),
                        _ => panic!("Unknown direction: {}", dir),
                    };
                    wire_step_count += 1;

                    let node = grid.nodes.entry(current).or_insert([None, None]);
                    node[wire_index] = node[wire_index].or(Some(wire_step_count));
                }
            }
        }

        grid
    }

    pub fn wire_intersections(&self) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter_map(|(node, &[wire_0, wire_1])| {
                if wire_0.is_none() || wire_1.is_none() {
                    None
                } else {
                    Some(node)
                }
            })
            .collect()
    }
}
