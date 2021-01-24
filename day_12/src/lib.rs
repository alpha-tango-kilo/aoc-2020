use std::convert::TryFrom;
use std::ops::{AddAssign, SubAssign};
use std::str::FromStr;

use anyhow::{Error, Result};

use crate::Direction::*;
use crate::Instruction::*;

#[derive(Debug)]
pub struct Ship {
    x: i32,
    y: i32,
    facing: Direction
}

impl Ship {
    pub fn new() -> Self {
        Ship::default()
    }

    pub fn apply_instructions(&mut self, ops: &Vec<Instruction>) {
        ops.iter().for_each(|op| {
            //println!("Ship at {}, facing {:?}, doing {:?}", self.location(), self.facing, op);
            match op {
                Compass(d, operand) => self.move_by_compass(d, *operand),
                Left(operand) => self.facing -= *operand,
                Right(operand) => self.facing += *operand,
                Forward(operand) => self.move_by_compass(&self.facing.clone(), *operand),
            }
        })
    }

    fn move_by_compass(&mut self, direction: &Direction, distance: u16) {
        let distance = distance as i32;
        match direction {
            North => self.y += distance,
            South => self.y -= distance,
            East  => self.x += distance,
            West  => self.x -= distance,
        }
    }

    pub fn manhatten_from_origin(&self) -> u32 {
        (i32::abs(self.x) + i32::abs(self.y)) as u32
    }

    pub fn location(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            x: 0,
            y: 0,
            facing: East,
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Compass(Direction, u16),
    Left(u16),
    Right(u16),
    Forward(u16),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let operand = s[1..].parse()?;
        match s.chars().next().unwrap() {
            'F' => Ok(Forward(operand)),
            'L' => Ok(Left(operand)),
            'R' => Ok(Right(operand)),
            c => Ok(Compass(Direction::try_from(c)?, operand)),
        }
    }
}

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    // This doesn't need to be explicitly stated
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl AddAssign<u16> for Direction {
    fn add_assign(&mut self, rhs: u16) {
        assert_eq!(rhs % 90, 0, "Unable to account for more precise rotation");
        let rotations = rhs / 90;
        *self = Direction::from(*self as u16 + rotations);
    }
}

impl SubAssign<u16> for Direction {
    fn sub_assign(&mut self, rhs: u16) {
        assert_eq!(rhs % 90, 0, "Unable to account for more precise rotation");
        let rotations = rhs / 90;
        *self = Direction::from(*self as u16 - rotations);
    }
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(North),
            'S' => Ok(South),
            'E' => Ok(East),
            'W' => Ok(West),
            _   => Err(Error::msg("Not a compass point")),
        }
    }
}

impl From<u16> for Direction {
    fn from(n: u16) -> Self {
        match n % 4 {
            0 => North,
            1 => East,
            2 => South,
            3 => West,
            _ => panic!("Can't coerce {} to a Direction", n),
        }
    }
}
