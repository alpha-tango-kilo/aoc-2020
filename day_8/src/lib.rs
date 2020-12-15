use anyhow::{Result, Error, Context};
use std::str::FromStr;
use Operation::*;
use std::convert::TryFrom;

pub type Program = Vec<Operation>;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // operand is worked out excluding the sign as Rust won't parse "-2"
        let mut operand = s[4..].parse()?;

        // unwrap is safe because otherwise parsing the number would have failed
        if s.chars().nth(5).unwrap() == '-' {
            operand *= -1;
        }

        //println!("Opcode: {}\tOperand: {}", &s[..3], operand);

        Ok(match &s[..3] {
            "nop" => Nop(operand),
            "acc" => Acc(operand),
            "jmp" => Jmp(operand),
              _   => return Err(Error::msg("Invalid opcode")),
        })
    }
}

#[derive(Debug)]
pub struct Computer {
    program: Program,
    accumulator: isize,
    program_counter: usize,
    trace: Vec<usize>,
}

impl Computer {
    pub fn new(program: Program) -> Self {
        Computer {
            program,
            accumulator: 0,
            program_counter: 0,
            trace: Vec::new(),
        }
    }

    pub fn part_one_run(&mut self) -> Result<isize> {
        loop {
            self.do_op()?;
            if self.trace.contains(&self.program_counter) {
                return Ok(self.accumulator);
            }
        }
    }

    pub fn part_two_run(&mut self) -> Result<isize> {
        loop {
            self.do_op()?;
            if self.program_counter == self.program.len() {
                return Ok(self.accumulator);
            } else if self.trace.contains(&self.program_counter) {
                return Err(Error::msg("Looping"));
            }
        }
    }

    fn do_op(&mut self) -> Result<()> {
        if let Some(op) = self.program.get(self.program_counter) {
            self.trace.push(self.program_counter);
            match op {
                Nop(_) => self.program_counter += 1,
                Acc(n) => {
                    self.accumulator += n;
                    self.program_counter += 1;
                },
                Jmp(n) => {
                    let a = isize::try_from(self.program_counter)
                        .context("Failed to perform jump due to program counter being larger than can fit in an isize")?;
                    self.program_counter = usize::try_from(a + n)
                        .context("Jump resulted in negative program counter")?;
                },
            }
            Ok(())
        } else {
            return Err(Error::msg("No instruction found"));
        }
    }
}

pub fn read_program(string: String) -> Result<Vec<Operation>> {
    string.lines()
        .map(Operation::from_str)
        .collect()
}

pub fn generate_permutations(prog: Program) -> Vec<Program> {
    let mut permutations = Vec::new();
    for n in 0..prog.len() {
        match prog.get(n).unwrap() {
            Nop(operand) => {
                let mut new_prog = prog.clone();
                new_prog.insert(n, Jmp(*operand));
                permutations.push(new_prog);
            },
            Jmp(operand) => {
                let mut new_prog = prog.clone();
                new_prog.insert(n, Nop(*operand));
                permutations.push(new_prog);
            },
            _ => {},
        }
    }
    permutations
}
