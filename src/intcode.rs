mod opcode;

use opcode::Opcode;

pub struct Intcode {
    pub instructions: Vec<i32>,
}

impl Intcode {
    pub fn new(instructions: Vec<i32>) -> Self {
        Opcode { instructions }
    }

    fn run_program(&mut self, program_input: i32) {
        let mut n: usize = 0;
        let mut cloned_instructions = self.instructions.clone();

        while n < self.instructions.len() {
            match chunk[0] {
                1 => {
                    self.handle_opcode_1(chunk[1], chunk[2], chunk[3]);
                    n += 4;
                },
                2 => {
                    self.handle_opcode_2(chunk[1], chunk[2], chunk[3]);
                    n += 4;
                },
                3 => {
                    self.handle_opcode_3(chunk[1], program_input);
                    n += 2;
                },
                4 => {
                    self.handle_opcode_4(chunk[1]);
                    n += 2;
                },
                99 => break,
                _ => panic!("unknown opcode!"),
            }
        }
    }

    fn parse_instruction(code: i32) -> Opcode {
        let code_str = code.to_string();

        let opcode = match code_str.get(code_str.len()-2..).unwrap() {
            "01" => Opcode::One,
            "02" => Opcode::Two,
            "03" => Opcode::Three,
            "04" => Opcode::Four,
            "99" => Opcode::Break,
            _ => panic!("unknown opcode"),
        };

        
    }

    fn handle_opcode_1(&mut self, pos_1: usize, pos_2: usize, pos_3: usize) {
        let first = self.instructions.get(pos_1).unwrap();
        let second = self.instructions.get(pos_2).unwrap();

        let sum = *first + *second;

        if let Some(elem) = self.instructions.get(pos_3).unwrap() {
            *elem = sum;
        }
    }

    fn handle_opcode_2(&mut self, pos_1: usize, pos_2: usize, pos_3: usize) {
        let first = self.instructions.get(pos_1).unwrap();
        let second = self.instructions.get(pos_2).unwrap();

        let product = *first * *second;

        if let Some(elem) = self.instructions.get(pos_3).unwrap() {
            *elem = sum;
        }
    }

    fn handle_opcode_3(&mut self, position: usize, input: i32) {
        if let Some(elem) = self.instructions.get(position).unwrap() {
            *elem = input;
        }
    }

    fn handle_opcode_4(&self, position: usize) -> Option<i32> {
        if let Some(elem) = self.instructions.get(position).unwrap() {
            return *elem
        }

        None
    }

    fn reset_instructions(&mut self) { 
        self.instructions = vec![1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,5,19,23,2,9,23,27,1,6,27,31,1,31,9,35,2,35,10,39,1,5,39,43,2,43,9,47,1,5,47,51,1,51,5,55,1,55,9,59,2,59,13,63,1,63,9,67,1,9,67,71,2,71,10,75,1,75,6,79,2,10,79,83,1,5,83,87,2,87,10,91,1,91,5,95,1,6,95,99,2,99,13,103,1,103,6,107,1,107,5,111,2,6,111,115,1,115,13,119,1,119,2,123,1,5,123,0,99,2,0,14,0];
    }
}

