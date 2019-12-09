use std::convert::TryInto;

#[derive(Debug, Eq, PartialEq)]
pub struct Program {
    pub memory: Memory,
    pub ip: usize,
    pub input: Input,
    pub output: Output,
    pub exit_code: ExitCode,
}

pub type Memory = Vec<isize>;
pub type Input = Option<Vec<isize>>;
pub type Output = Option<Vec<isize>>;
pub type ExitCode = Option<isize>;
pub type Instruction = isize;
pub type Opcode = isize;
pub type ParameterModes = Vec<ParameterMode>;

#[derive(Debug)]
pub enum ParameterMode {
    PositionMode,
    ImmediateMode,
}

impl Program {
    pub fn new(memory: Memory, input: Input) -> Program {
        Program {
            memory,
            ip: 0,
            input,
            output: None,
            exit_code: None,
        }
    }

    fn parse_param_mode(mode_char: Option<char>) -> ParameterMode {
        use ParameterMode::*;

        match mode_char {
            Some('0') => PositionMode,
            Some('1') => ImmediateMode,
            None => PositionMode,
            Some(c) => panic!("Unknown parameter mode: {}", c),
        }
    }

    fn parse_opcode(instruction: Instruction) -> Opcode {
        let str_opcode = format!("{}", instruction);

        match str_opcode.len() {
            1 => instruction,
            2 => instruction,
            _ => str_opcode[(str_opcode.len() - 2)..].parse().unwrap(),
        }
    }

    fn parse_param_modes(instruction: Instruction, n_params: usize) -> ParameterModes {
        let mut param_modes = Vec::with_capacity(n_params);

        // Left-zero-pad the instruction to at least 2 characters
        //    "1" =>   "01"
        // "0101" => "0101"
        let full_instruction = format!("{:02}", instruction);
        let instruction_len = full_instruction.len();

        for idx in 0..n_params {
            let param_mode_char = instruction_len
                .checked_sub(3 + idx)
                .and_then(|i| full_instruction.chars().nth(i));
            param_modes.push(Program::parse_param_mode(param_mode_char));
        }

        param_modes
    }

    pub fn run(&mut self) -> &mut Self {
        while let Some(&instruction) = self.memory.get(self.ip) {
            let opcode = Program::parse_opcode(instruction);

            let advance_ip_by = match opcode {
                // ADD: [c] = [a] + [b]
                1 => {
                    let param_modes = Program::parse_param_modes(instruction, 2);
                    let params = self.get_params(&param_modes);
                    let value = params.get(0).unwrap() + params.get(1).unwrap();
                    let write_idx = self.memory[self.ip + 3] as usize;
                    self.memory[write_idx] = value;
                    4
                }

                // MULTIPLY: [c] = [a] * [b]
                2 => {
                    let param_modes = Program::parse_param_modes(instruction, 2);
                    let params = self.get_params(&param_modes);
                    let value = params.get(0).unwrap() * params.get(1).unwrap();
                    let write_idx = self.memory[self.ip + 3] as usize;
                    self.memory[write_idx] = value;
                    4
                }

                // READ: [a] = input[0]
                3 => {
                    let write_idx = self.memory[self.ip + 1] as usize;

                    if self.input.as_ref().unwrap().is_empty() {
                        // Pause execution
                        self.exit_code = Some(3);
                        return self;
                    } else {
                        // Use the input value
                        let input = self.input.as_mut().unwrap();
                        self.memory[write_idx] = input.remove(0);
                        self.input = Some(input.to_vec());
                        2
                    }
                }

                // WRITE: output = [a]
                4 => {
                    let read_idx = self.memory[self.ip + 1] as usize;
                    if self.output.is_none() {
                        self.output = Some(vec![]);
                    }
                    self.output.as_mut().unwrap().push(self.memory[read_idx]);
                    2
                }

                // JUMP-IF-TRUE: if [a] != 0 then ip = [b]
                5 => {
                    let param_modes = Program::parse_param_modes(instruction, 2);
                    let params = self.get_params(&param_modes);
                    if *params.get(0).unwrap() != 0 {
                        self.ip = (*params.get(1).unwrap()).try_into().unwrap();
                        0
                    } else {
                        3
                    }
                }

                // JUMP-IF-FALSE: if [a] == 0 then ip = [b]
                6 => {
                    let param_modes = Program::parse_param_modes(instruction, 2);
                    let params = self.get_params(&param_modes);
                    if *params.get(0).unwrap() == 0 {
                        self.ip = (*params.get(1).unwrap()).try_into().unwrap();
                        0
                    } else {
                        3
                    }
                }

                // LESS-THAN: [c] = [a] < [b] ? 1 : 0
                7 => {
                    let param_modes = Program::parse_param_modes(instruction, 2);
                    let params = self.get_params(&param_modes);
                    let value = if params.get(0) < params.get(1) { 1 } else { 0 };
                    let write_idx = self.memory[self.ip + 3] as usize;
                    self.memory[write_idx] = value;
                    4
                }

                // EQUALS: [c] = [a] == [b] ? 1 : 0
                8 => {
                    let param_modes = Program::parse_param_modes(instruction, 2);
                    let params = self.get_params(&param_modes);
                    let value = if params.get(0) == params.get(1) { 1 } else { 0 };
                    let write_idx = self.memory[self.ip + 3] as usize;
                    self.memory[write_idx] = value;
                    4
                }

                // END
                99 => {
                    self.exit_code = Some(99);
                    return self;
                },

                // UNKNOWN
                _ => panic!("Unknown opcode: {}", opcode),
            };

            self.ip += advance_ip_by;
        }

        panic!();
    }

    fn get_params(&self, param_modes: &ParameterModes) -> Vec<isize> {
        use ParameterMode::*;

        let get_param = |(idx, m)| {
            let ip_offset = idx + 1;
            match m {
                &PositionMode => self.memory[self.memory[self.ip + ip_offset] as usize],
                &ImmediateMode => self.memory[self.ip + ip_offset],
            }
        };

        param_modes.iter().enumerate().map(get_param).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_run() {
        assert_eq!(
            Program::new(vec![1002, 4, 3, 4, 33], None).run().memory,
            vec![1002, 4, 3, 4, 99],
        );
        assert_eq!(
            *Program::new(vec![3, 0, 4, 0, 99], Some(vec![1234])).run(),
            Program {
                memory: vec![1234, 0, 4, 0, 99],
                ip: 4,
                input: Some(vec![]),
                output: Some(vec![1234]),
                exit_code: Some(99),
            },
        );
    }

    #[test]
    fn test_program_run_day_2() {
        assert_eq!(
            Program::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], None)
                .run()
                .memory,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
        assert_eq!(
            Program::new(vec![1, 0, 0, 0, 99], None).run().memory,
            vec![2, 0, 0, 0, 99],
        );
        assert_eq!(
            Program::new(vec![2, 3, 0, 3, 99], None).run().memory,
            vec![2, 3, 0, 6, 99],
        );
        assert_eq!(
            Program::new(vec![2, 4, 4, 5, 99, 0], None).run().memory,
            vec![2, 4, 4, 5, 99, 9801],
        );
    }
}
