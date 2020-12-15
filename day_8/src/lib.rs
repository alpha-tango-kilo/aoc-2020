use anyhow::{Result, Error, Context};
use std::str::FromStr;
use Operation::*;
use std::convert::TryFrom;

pub type Program = Vec<Operation>;

#[derive(Debug)]
pub enum Operation {
    Nop,
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
            "nop" => Nop,
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

    pub fn run(&mut self) -> Result<()> {
        loop {
            self.do_op()?;
        }
    }

    pub fn debug_run(&mut self) -> Result<isize> {
        loop {
            self.do_op()?;
            if self.trace.contains(&self.program_counter) {
                return Ok(self.accumulator);
            }
        }
    }

    fn do_op(&mut self) -> Result<()> {
        if let Some(op) = self.program.get(self.program_counter) {
            self.trace.push(self.program_counter);
            match op {
                Nop => self.program_counter += 1,
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
