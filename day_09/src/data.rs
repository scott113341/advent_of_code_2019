use std::collections::HashMap;
use std::convert::TryInto;

use ParameterMode::*;
use ParameterUse::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Program {
    pub memory: Memory,
    pub ext_memory: ExtMemory,
    pub ip: usize,
    pub relative_base: usize,
    pub input: Input,
    pub output: Output,
    pub exit_code: ExitCode,
}

pub type Memory = Vec<isize>;
pub type ExtMemory = HashMap<usize, isize>;
pub type Input = Vec<isize>;
pub type Output = Vec<isize>;
pub type ExitCode = Option<isize>;
pub type Instruction = isize;
pub type Opcode = isize;
pub type ParameterModes = Vec<ParameterMode>;

#[derive(Debug)]
pub struct ParamAt(usize, ParameterUse);

#[derive(Debug)]
pub enum ParameterUse {
    Read,
    Write,
}

#[derive(Debug)]
pub enum ParameterMode {
    PositionMode,
    ImmediateMode,
    RelativeMode,
}

impl Program {
    pub fn new(memory: Memory, input: Option<Input>) -> Program {
        Program {
            memory,
            ext_memory: HashMap::new(),
            ip: 0,
            relative_base: 0,
            input: input.unwrap_or(vec![]),
            output: vec![],
            exit_code: None,
        }
    }

    fn parse_param_mode(mode_char: Option<char>) -> ParameterMode {
        use ParameterMode::*;

        match mode_char {
            Some('0') => PositionMode,
            Some('1') => ImmediateMode,
            Some('2') => RelativeMode,
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

    pub fn read_mem(&mut self, idx: usize) -> isize {
        if idx < self.memory.len() {
            *self.memory.get(idx).unwrap()
        } else {
            *self.ext_memory.entry(idx).or_insert(0)
        }
    }

    pub fn write_mem(&mut self, idx: usize, value: isize) {
        if idx < self.memory.len() {
            *self.memory.get_mut(idx).unwrap() = value;
        } else {
            self.ext_memory.entry(idx).insert(value);
        }
    }

    pub fn run(&mut self) -> &mut Self {
        while let Some(&instruction) = self.memory.get(self.ip) {
            let opcode = Program::parse_opcode(instruction);

            let advance_ip_by = match opcode {
                // ADD: [c] = [a] + [b]
                1 => {
                    let params = self.get_params(instruction, vec![
                        ParamAt(0, Read),
                        ParamAt(1, Read),
                        ParamAt(2, Write),
                    ]);
                    let value = params[0] + params[1];
                    self.write_mem(params[2] as usize, value);
                    4
                }

                // MULTIPLY: [c] = [a] * [b]
                2 => {
                    let params = self.get_params(instruction, vec![
                        ParamAt(0, Read),
                        ParamAt(1, Read),
                        ParamAt(2, Write),
                    ]);
                    let value = params[0] * params[1];
                    self.write_mem(params[2] as usize, value);
                    4
                }

                // READ: [a] = input[0]
                3 => {
                    let params = self.get_params(instruction, vec![
                        ParamAt(0, Write),
                    ]);
                    if self.input.is_empty() {
                        // Pause execution
                        self.exit_code = Some(3);
                        return self;
                    } else {
                        // Read the input value
                        let value = self.input.remove(0);
                        self.write_mem(params[0] as usize, value);
                        2
                    }
                }

                // WRITE: output = [a]
                4 => {
                    let params = self.get_params(instruction, vec![
                        ParamAt(0, Read),
                    ]);
                    self.output.push(params[0]);
                    2
                }

                // JUMP-IF-TRUE: if [a] != 0 then ip = [b]
                5 => {
                    let params = self.get_params(instruction, vec![
                        ParamAt(0, Read),
                        ParamAt(1, Read),
                    ]);
                    if params[0] != 0 {
                        self.ip = params[1].try_into().unwrap();
                        0
                    } else {
                        3
                    }
                }

                // JUMP-IF-FALSE: if [a] == 0 then ip = [b]
                6 => {
                    let params = self.get_params(instruction, vec![
                        ParamAt(0, Read),
                        ParamAt(1, Read),
                    ]);
                    if params[0] == 0 {
                        self.ip = params[1].try_into().unwrap();
                        0
                    } else {
                        3
                    }
                }

                // LESS-THAN: [c] = [a] < [b] ? 1 : 0
                7 => {
                    let params = self.get_params(instruction, vec![
                        ParamAt(0, Read),
                        ParamAt(1, Read),
                        ParamAt(2, Write),
                    ]);
                    let value = if params[0] < params[1] { 1 } else { 0 };
                    self.write_mem(params[2] as usize, value);
                    4
                }

                // EQUALS: [c] = [a] == [b] ? 1 : 0
                8 => {
                    let params = self.get_params(instruction, vec![
                        ParamAt(0, Read),
                        ParamAt(1, Read),
                        ParamAt(2, Write),
                    ]);
                    let value = if params[0] == params[1] { 1 } else { 0 };
                    self.write_mem(params[2] as usize, value);
                    4
                }

                // ADJUST-RELATIVE-BASE: rb += [a]
                9 => {
                    let params = self.get_params(instruction, vec![
                        ParamAt(0, Read),
                    ]);
                    self.relative_base = (self.relative_base as isize + params[0]) as usize;
                    2
                }

                // END
                99 => {
                    self.exit_code = Some(99);
                    return self;
                }

                // UNKNOWN
                _ => panic!("Unknown opcode: {}", opcode),
            };

            self.ip += advance_ip_by;
        }

        self
    }

    fn get_params(&mut self, instruction: Instruction, params_at: Vec<ParamAt>) -> Vec<isize> {
        let get_param = |(mode, pat): (&ParameterMode, ParamAt)| {
            let ip_offset = pat.0 + 1;
            let ip_plus_offset = self.ip + ip_offset;

            match mode {
                PositionMode => {
                    let idx = self.read_mem(ip_plus_offset);
                    match pat.1 {
                        Read => self.read_mem(idx as usize),
                        Write => idx,
                    }
                }
                ImmediateMode => {
                    match pat.1 {
                        Read => self.read_mem(ip_plus_offset),
                        Write => panic!("ImmediateMode cannot be used for writes"),
                    }
                }
                RelativeMode => {
                    let idx = self.relative_base as isize + self.read_mem(ip_plus_offset);
                    match pat.1 {
                        Read => self.read_mem(idx as usize),
                        Write => idx,
                    }
                }
            }
        };

        let param_modes = Program::parse_param_modes(instruction, params_at.len());
        param_modes.iter().zip(params_at).map(get_param).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_run_day_09() {
        assert_eq!(
            Program::new(vec![109, 10, 109, -2, 204, -7, 99], None)
                .run()
                .output,
            vec![10],
        );
        assert_eq!(
            *Program::new(vec![109, 8, 203, 10, 99], Some(vec![11])).run(),
            Program {
                memory: vec![109, 8, 203, 10, 99],
                ext_memory: {
                    let mut mem = HashMap::new();
                    mem.insert(18, 11);
                    mem
                },
                ip: 4,
                relative_base: 8,
                input: vec![],
                output: vec![],
                exit_code: Some(99),
            },
        );
        assert_eq!(
            Program::new(
                vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
                None
            )
            .run()
            .output,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
        );
        assert_eq!(
            Program::new(vec![1102,34915192,34915192,7,4,7,99,0], None)
                .run()
                .output,
            vec![1219070632396864],
        );
        assert_eq!(
            Program::new(vec![104,1125899906842624,99], None)
                .run()
                .output,
            vec![1125899906842624],
        );
    }

    #[test]
    fn test_program_run_day_07() {
        assert_eq!(
            Program::new(vec![1002, 4, 3, 4, 33], None).run().memory,
            vec![1002, 4, 3, 4, 99],
        );
        assert_eq!(
            *Program::new(vec![3, 0, 4, 0, 99], Some(vec![1234])).run(),
            Program {
                memory: vec![1234, 0, 4, 0, 99],
                ext_memory: HashMap::new(),
                ip: 4,
                relative_base: 0,
                input: vec![],
                output: vec![1234],
                exit_code: Some(99),
            },
        );
    }

    #[test]
    fn test_program_run_day_02() {
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
