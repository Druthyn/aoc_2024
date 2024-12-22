use std::{ops::BitXor, str::FromStr};

use advent_of_code::split_input_at_emptyline;

advent_of_code::solution!(17);

#[derive(Clone, Copy)]
struct Processor {
    instruction_pointer: usize,
    registers: [i64; 3],
}

type SetRegFn = fn(&mut Processor, i64);

impl Processor {
    fn new(registers: &[&str]) -> Self {
        let reg_vec: Vec<i64> = registers
            .iter()
            .map(|s| s.split(": ").nth(1).unwrap().parse().unwrap())
            .collect();
        let mut reg = [0; 3];
        reg.copy_from_slice(&reg_vec);
        Self {
            instruction_pointer: 0,
            registers: reg,
        }
    }

    fn get_reg_a(&self) -> i64 {
        self.registers[0]
    }

    fn set_reg_a(&mut self, v: i64) {
        self.registers[0] = v;
    }

    fn get_reg_b(&self) -> i64 {
        self.registers[1]
    }

    fn set_reg_b(&mut self, v: i64) {
        self.registers[1] = v;
    }

    fn get_reg_c(&self) -> i64 {
        self.registers[2]
    }

    fn set_reg_c(&mut self, v: i64) {
        self.registers[2] = v;
    }

    fn xdv(&mut self, operand: u32, func: SetRegFn) -> Option<String> {
        self.instruction_pointer += 2;
        match self.combo(operand) {
            Some(v) => {
                let res = self.get_reg_a().div_euclid(2_i64.pow(v as u32));
                func(self, res);
                Some("".to_string())
            }
            None => None,
        }
    }

    fn process_instruction(&mut self, instructions: &[u32]) -> Option<String> {
        match instructions.get(self.instruction_pointer) {
            Some(&x) => {
                if let Some(&operand) = instructions.get(self.instruction_pointer + 1) {
                    match x {
                        0 => self.xdv(operand, Processor::set_reg_a),
                        1 => {
                            self.set_reg_b(self.get_reg_b().bitxor(operand as i64));
                            self.instruction_pointer += 2;
                            Some("".to_string())
                        }
                        2 => {
                            self.instruction_pointer += 2;
                            match self.combo(operand) {
                                Some(operand) => {
                                    self.set_reg_b(operand % 8);
                                    Some("".to_string())
                                }
                                None => None,
                            }
                        }
                        3 => {
                            if self.get_reg_a() != 0 {
                                self.instruction_pointer = operand as usize;
                            } else {
                                self.instruction_pointer += 2;
                            }
                            Some("".to_string())
                        }
                        4 => {
                            self.set_reg_b(self.get_reg_b().bitxor(self.get_reg_c()));
                            self.instruction_pointer += 2;
                            Some("".to_string())
                        }
                        5 => {
                            self.instruction_pointer += 2;
                            self.combo(operand).map(|v| (v % 8).to_string())
                        }
                        6 => self.xdv(operand, Processor::set_reg_b),
                        7 => self.xdv(operand, Processor::set_reg_c),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn combo(&self, operand: u32) -> Option<i64> {
        match operand {
            0..=3 => Some(operand as i64),
            4 => Some(self.get_reg_a()),
            5 => Some(self.get_reg_b()),
            6 => Some(self.get_reg_c()),
            7.. => None,
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let parts = split_input_at_emptyline(input);
    let mut processor = Processor::new(parts.first()?);

    let instructions: Vec<u32> = parts[1][0][9..]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut all_out = String::from_str("").unwrap();

    while let Some(out) = processor.process_instruction(&instructions) {
        if !out.is_empty() {
            all_out = all_out.to_owned() + "," + &out;
        }
    }

    Some(all_out[1..].to_string())
}

pub fn part_two(_input: &str) -> Option<i64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let binding = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&binding);
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
