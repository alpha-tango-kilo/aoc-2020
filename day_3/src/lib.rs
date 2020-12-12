use anyhow::{Result, Context, Error};
use std::str::FromStr;
use std::convert::{TryInto, TryFrom};
use std::ops::{Sub, AddAssign, Add};
use Tile::*;

pub fn trees_hit(slope: &Slope, start: &Vec2D, vector: &Vec2D) -> Result<usize> {
    let mut current = start.clone();
    let mut trees_hit = 0;

    while usize::try_from(current.y)? < slope.height {
        if slope.get_tile_at(&current)? == &Tree {
            trees_hit += 1;
        }
        current += *vector;
    }

    Ok(trees_hit)
}

#[derive(Debug)]
pub struct Slope {
    area: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Slope {
    // Alias new to from_str
    pub fn new(s: &str) -> Result<Self> {
        Slope::from_str(s)
    }

    // Top left considered to be (0, 0)
    pub fn get_tile_at(&self, loc: &Vec2D) -> Result<&Tile> {
        let x: usize = loc.x.try_into().context("x co-ordinate negative")?;
        let x = x % self.width; // wrap x to mimic horizontal tiling
        let y: usize = loc.y.try_into().context("y co-ordinate negative")?;

        Ok(
            self.area.get(y)
                .context(format!("y co-ordinate out of range"))?
                .get(x)
                .expect(&format!("x co-ordinate out of range!"))
        )
    }
}

impl FromStr for Slope {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let area = s.lines()
            .map(|line| {
                line.chars()
                    .filter_map(|c| { Tile::from_char(&c).ok() })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<_>>();
        let height = area.len();
        let width = area.get(0)
            .context("Slope has 0 height")?
            .len();

        Ok(Slope {
            area,
            width,
            height,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Vec2D {
    pub x: isize,
    pub y: isize,
}

// Unused
impl Add for Vec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// Unused
impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Tile {
    Open,
    Tree,
}

impl Tile {
    fn from_char(s: &char) -> Result<Self> {
        if *s == '.' {
            Ok(Open)
        } else if *s == '#' {
            Ok(Tree)
        } else {
            Err(Error::msg(format!("Invalid string for Tile: {:?}", s)))
        }
    }
}
