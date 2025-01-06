use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Proc {
    A: u32,
    B: u32,
    C: u32,
    program: Vec<u32>,
    pointer: usize,
    output: String,
}

impl Proc {
    fn new(A: u32, B: u32, C: u32, program: Vec<u32>) -> Self {
        Self {
            A: A,
            B: B,
            C: C,
            pointer: 0,
            output: "".to_string(),
            program: program,
        }
    }
    fn combo(&self, operand: u32) -> u32 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.A,
            5 => self.B,
            6 => self.C,
            7 => 0,
            _ => 0,
        }
    }

    fn adv(&mut self, operand: u32) {
        let denominator = 2_u32.pow(self.combo(operand));
        self.A = self.A.div_euclid(denominator);
        self.pointer += 2;
    }

    fn bdv(&mut self, operand: u32) {
        let denominator = 2_u32.pow(self.combo(operand));
        self.B = self.A.div_euclid(denominator);
        self.pointer += 2;
    }

    fn cdv(&mut self, operand: u32) {
        let denominator = 2_u32.pow(self.combo(operand));
        self.C = self.A.div_euclid(denominator);
        self.pointer += 2;
    }

    fn bxl(&mut self, operand: u32) {
        self.B = self.B ^ operand;
        self.pointer += 2;
    }

    fn bst(&mut self, operand: u32) {
        self.B = self.combo(operand).rem_euclid(8);
        self.pointer += 2;
    }

    fn jnz(&mut self, operand: u32) {
        if self.A == 0 {
            self.pointer += 2;
            return;
        }
        self.pointer = operand as usize;
    }

    fn bxc(&mut self, _operand: u32) {
        self.B = self.B ^ self.C;
        self.pointer += 2;
    }

    fn out(&mut self, operand: u32) {
        let result = self.combo(operand).rem_euclid(8);
        if self.output == "" {
            self.output = result.to_string();
        } else {
            self.output = format!("{},{}", self.output, result);
        }
        self.pointer += 2;
    }

    fn process(&mut self) {
        let operand = self.program[self.pointer];
        let literal = self.program[self.pointer + 1];
        match operand {
            0 => self.adv(literal),
            1 => self.bxl(literal),
            2 => self.bst(literal),
            3 => self.jnz(literal),
            4 => self.bxc(literal),
            5 => self.out(literal),
            6 => self.bdv(literal),
            7 => self.cdv(literal),
            _ => (),
        }
    }

    fn run(&mut self) {
        while self.pointer < self.program.len() {
            self.process();
        }
    }
}

fn main() {
    let mut pc: Proc = Proc::new(729, 0, 0, vec![0, 1, 5, 4, 3, 0]);
    pc = Proc::new(46323429,0,0,vec![2,4,1,1,7,5,1,5,4,3,0,3,5,5,3,0]);
    pc.run();

    while pc.pointer < pc.program.len() {
        pc.run();
    }
    

    println!("Sum is :{}", pc.output);
}

fn pause() {
    dbg!("Pausing! Press enter to continue...");

    let mut buffer = String::new();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}
