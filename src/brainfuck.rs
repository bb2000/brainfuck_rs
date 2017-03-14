use std::io::{Read, Write, Stdin};
use std::io;

const ARRAY_SIZE: u32 = 30000;

pub struct Brainfuck {
    memory: Box<[u8]>,
    pointer: u32,
    program: Vec<u8>,
    input: Vec<u8>,
    pc: u32,
    instructions: u32,
}

impl Brainfuck {
    pub fn new(program: Vec<u8>, input: Vec<u8>) -> Brainfuck {
        Brainfuck {
            memory: vec![0 as u8; ARRAY_SIZE as usize].into_boxed_slice(),
            pointer: 0,
            program: program,
            input: input,
            pc: 0,
            instructions: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.pc as usize > self.program.len() - 1 {
                return;
            }
            let instruction = self.program[self.pc as usize];
            self.run_instruction(instruction as char);
        }
    }

    pub fn print_state(&self) {
        println!("Pointer location: {}", self.pointer);
        println!("Instruction : {}", self.pc);
        println!("Instructions completed: {}", self.instructions);
        for location in (0..self.memory.len()) {
            if self.memory[location] != 0 {
                println!("{}: {}[{}]", location, self.memory[location] as char,
                         self.memory[location]);
            }
        }
    }

    fn run_instruction(&mut self, instruction: char) {
        match instruction {
            '>' => { if self.pointer != ARRAY_SIZE - 1 {self.pointer += 1;} else { self.pointer = 0; }},
            '<' => { if self.pointer != 0 {self.pointer -= 1;} else {self.pointer = ARRAY_SIZE - 1 ;} },
            '+' => { if self.memory[self.pointer as usize] != 0b1111_1111 {self.memory[self.pointer as usize] += 1;} else { self.memory[self.pointer as usize] = 0; } },
            '-' => { if self.memory[self.pointer as usize] != 0 {self.memory[self.pointer as usize] -= 1;} else {self.memory[self.pointer as usize] = 0b1111_1111 }},
            '.' => { print!("{}", self.memory[self.pointer as usize] as char) },
            ',' => { self.memory[self.pointer as usize] = self.get_input(); }
            '[' => {
                    if self.memory[self.pointer as usize] == 0 {
                        let mut loops = 0;
                        loop {
                            self.pc += 1;
                            let value = self.program[self.pc as usize];
                            if value == '[' as u8 {
                                loops += 1;
                            } else if (value == ']' as u8) & (loops == 0) {
                                break
                            } else if value == ']' as u8 {
                                loops -= 1;
                            }
                        }
                    }
                   },
            ']' => {
                    if self.memory[self.pointer as usize] != 0 {
                        let mut loops = 0;
                        loop {
                            self.pc -= 1;
                            let value = self.program[self.pc as usize];
                            if value == ']' as u8 {
                                loops += 1;
                            } else if (value == '[' as u8) & (loops == 0) {
                                break
                            } else if value == '[' as u8 {
                                loops -= 1;
                            }
                        }
                    }
                   },
            '#' => { self.print_state(); },
            _  => { },
        }
        self.pc += 1;
        self.instructions += 1;
    }

    fn get_input(&mut self) -> u8 {
        if self.input.len() == 0 {
            0
        } else {
            self.input.remove(0)
        }
    }
}
