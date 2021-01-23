use std::str::FromStr;

use anyhow::{Context, Error, Result};

use crate::Tile::*;

#[derive(Debug)]
pub struct Area {
    width: usize,
    height: usize,
    pub tiles: Vec<Vec<Tile>>,
    stable: bool,
}

impl Area {
    fn iterate(&mut self) {
        let mut to_toggle = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_at(x, y) {
                    Some(&EmptySeat) => if self.adjacent_occupied(x, y) == 0 {
                        to_toggle.push((x, y));
                    },
                    Some(&OccupiedSeat) => if self.adjacent_occupied(x, y) >= 4 {
                        to_toggle.push((x, y));
                    },
                    _ => {},
                }
            }
        }
        self.stable = to_toggle.is_empty();
        to_toggle.into_iter()
            .for_each(|(x, y)| {
                self.get_at_mut(x, y)
                    .unwrap()
                    .toggle()
            });
    }

    pub fn iterate_until_stable(&mut self) {
        while !self.stable {
            self.iterate();
        }
    }

    fn adjacent_occupied(&self, x: usize, y: usize) -> u8 {
        assert!(x < self.width, "x too large for width of area!");
        assert!(y < self.height, "y too large for height of area!");
        let mut to_count = Vec::with_capacity(8);
        if x > 0 {
            to_count.push(self.get_at(x - 1, y));
            to_count.push(self.get_at(x - 1, y + 1));
        }
        if y > 0 {
            to_count.push(self.get_at(x, y - 1));
            to_count.push(self.get_at(x + 1, y - 1));
        }
        if x > 0 && y > 0 {
            to_count.push(self.get_at(x - 1, y - 1));
        }
        to_count.push(self.get_at(x + 1, y));
        to_count.push(self.get_at(x + 1, y + 1));
        to_count.push(self.get_at(x, y + 1));

        to_count.into_iter()
            .map(|opt| {
                if opt == Some(&OccupiedSeat) {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    pub fn occupied(&self) -> usize {
        self.tiles.iter()
            .map(|row| {
                row.iter()
                    .map(|t| {
                    if *t == OccupiedSeat {
                        1
                    } else {
                        0
                    }
                }).sum::<usize>()
            }).sum()
    }

    fn get_at(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y).and_then(|row| row.get(x))
    }

    fn get_at_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(y).and_then(|row| row.get_mut(x))
    }
}

impl FromStr for Area {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let width = s.lines()
            .next()
            .context("Empty string")?
            .len();
        let height = s.lines().count();

        let tiles = s.lines()
            .map(|line| {
                line.chars()
                    .map(|c| Tile::from_char(&c))
                    .collect()
            }).collect::<Result<Vec<Vec<_>>>>()?;

        Ok(Area {
            width,
            height,
            tiles,
            stable: false,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Tile {
    fn toggle(&mut self) {
        match self {
            Floor => {},
            EmptySeat => *self = OccupiedSeat,
            OccupiedSeat => *self = EmptySeat,
        }
    }

    fn from_char(c: &char) -> Result<Self> {
        match c {
            '.' => Ok(Floor),
            'L' => Ok(EmptySeat),
            '#' => Ok(OccupiedSeat),
            _   => Err(Error::msg("Invalid character for tile")),
        }
    }
}
