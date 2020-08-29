use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Synthesis {
    pub reactions: Vec<Reaction>,
}

impl Synthesis {
    pub fn new() -> Synthesis {
        Synthesis {
            reactions: Vec::new(),
        }
    }

    pub fn add_reaction(&mut self, line: &String) {
        lazy_static! {
            static ref FULL_RE: Regex = Regex::new(r"^(.+) => (\d+) (\w+)$").unwrap();
            static ref INPUT_RE: Regex = Regex::new(r"^(\d+) (\w+)$").unwrap();
        }

        let full_caps = FULL_RE.captures(line).unwrap();

        let output = Chemical {
            name: full_caps[3].to_string(),
            count: full_caps[2].parse().unwrap(),
        };

        let inputs = full_caps[1]
            .split(", ")
            .map(|input| {
                let caps = INPUT_RE.captures(input).unwrap();
                Chemical {
                    name: caps[2].to_string(),
                    count: caps[1].parse().unwrap(),
                }
            })
            .collect();

        self.reactions.push(Reaction { inputs, output });
    }

    pub fn reaction_producing(&self, name: &String) -> &Reaction {
        self.reactions
            .iter()
            .find(|r| r.output.name == *name)
            .unwrap()
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Reaction {
    pub inputs: Vec<Chemical>,
    pub output: Chemical,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Chemical {
    pub name: String,
    pub count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synthesis_add_reaction() {
        let mut synthesis = Synthesis::new();
        synthesis.add_reaction(&"12 JSMPL, 1 RFSHT => 8 NLTCF".to_string());

        assert_eq!(
            synthesis.reactions,
            vec![Reaction {
                inputs: vec![
                    Chemical {
                        name: "JSMPL".to_string(),
                        count: 12
                    },
                    Chemical {
                        name: "RFSHT".to_string(),
                        count: 1
                    },
                ],
                output: Chemical {
                    name: "NLTCF".to_string(),
                    count: 8
                },
            }]
        );
    }
}
