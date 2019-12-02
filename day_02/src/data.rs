#[derive(Debug)]
pub struct Program {
    pub memory: Vec<usize>,
    pub ip: usize,
}

pub type Memory = Vec<usize>;

impl Program {
    pub fn new(memory: Memory) -> Program {
        Program {
            memory,
            ip: 0,
        }
    }

    pub fn run(mut self) -> Self {
        while let Some(opcode) = self.memory.get(self.ip) {
            let advance_ip_by = match opcode {
                1 => {
                    let (output_index, input_1_index, input_2_index) = self.parameters();
                    let value = self.memory[input_1_index] + self.memory[input_2_index];
                    self.memory[output_index] = value;
                    4
                },
                2 => {
                    let (output_index, input_1_index, input_2_index) = self.parameters();
                    let value = self.memory[input_1_index] * self.memory[input_2_index];
                    self.memory[output_index] = value;
                    4
                },
                99 => break,
                _ => panic!("Unknown opcode: {}", opcode),
            };

            self.ip += advance_ip_by;
        }

        self
    }

    fn parameters(&self) -> (usize, usize, usize) {
        let output_index = self.memory[self.ip + 3];
        let input_1_index = self.memory[self.ip + 1];
        let input_2_index = self.memory[self.ip + 2];
        (output_index, input_1_index, input_2_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_run() {
        assert_eq!(
            Program::new(vec![1,9,10,3,2,3,11,0,99,30,40,50]).run().memory,
            vec![3500,9,10,70,2,3,11,0,99,30,40,50]
        );
        assert_eq!(
            Program::new(vec![1,0,0,0,99]).run().memory,
            vec![2,0,0,0,99]
        );
        assert_eq!(
            Program::new(vec![2,3,0,3,99]).run().memory,
            vec![2,3,0,6,99]
        );
        assert_eq!(
            Program::new(vec![2,4,4,5,99,0]).run().memory,
            vec![2,4,4,5,99,9801]
        );
        assert_eq!(
            Program::new(vec![1,1,1,4,99,5,6,0,99]).run().memory,
            vec![30,1,1,4,2,5,6,0,99]
        );
    }
}
