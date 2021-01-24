use std::fmt::{self, Display};
use std::str::FromStr;

use anyhow::{Context, Error, Result};

use crate::Direction::*;
use crate::Tile::*;

const DIRECTIONS: [Direction; 8] = [Up, Down, Left, Right, UpLeft, UpRight, DownLeft, DownRight];

#[derive(Debug, Clone)]
pub struct Area {
    width: usize,
    height: usize,
    pub tiles: Vec<Vec<Tile>>,
    stable: bool,
}

impl Area {
    fn iterate(&mut self, method: Method) {
        let mut to_toggle = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                match method {
                    Method::NextTo => {
                        match self.get_at(x, y) {
                            Some(&EmptySeat) => if self.adjacent_occupied(x, y) == 0 {
                                to_toggle.push((x, y));
                            },
                            Some(&OccupiedSeat) => if self.adjacent_occupied(x, y) >= 4 {
                                to_toggle.push((x, y));
                            },
                            _ => {},
                        }
                    },
                    Method::LineOfSight => {
                        match self.get_at(x, y) {
                            Some(&EmptySeat) => if self.line_of_sight_occupied(x, y) == 0 {
                                to_toggle.push((x, y));
                            },
                            Some(&OccupiedSeat) => if self.line_of_sight_occupied(x, y) >= 5 {
                                to_toggle.push((x, y));
                            },
                            _ => {},
                        }
                    },
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

    pub fn iterate_until_stable(&mut self, method: Method) {
        //let mut count = 0u32;
        while !self.stable {
            self.iterate(method);
            //count += 1;
            //println!("Iteration {}", count);
            //println!("{}", self);
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

    fn line_of_sight_occupied(&self, x: usize, y: usize) -> usize {
        DIRECTIONS.iter()
            .map(|dir| self.direction_occupied((x, y), *dir) as usize)
            .sum()
    }

    fn direction_occupied(&self, origin: (usize, usize), direction: Direction) -> u8 {
        let (mut current_x, mut current_y) = origin;

        // Assertions
        assert!(current_x < self.width, "Invalid origin");
        assert!(current_y < self.height, "Invalid origin");

        match direction {
            Up => current_y -= 1,
            Down => current_y += 1,
            Left => current_x -= 1,
            Right => current_x += 1,
            UpLeft => { current_x -= 1; current_y -= 1; },
            UpRight => { current_x += 1; current_y -= 1; },
            DownLeft => { current_x -= 1; current_y += 1; },
            DownRight => { current_x += 1; current_y += 1; },
        }

        // Traverse along
        while current_x < self.width && current_y < self.height {
            let t = self.get_at(current_x, current_y)
                .expect("Didn't get tile while traversing");
            match t {
                Floor => {},
                EmptySeat => return 0,
                OccupiedSeat => return 1,
            }
            match direction {
                Up => current_y -= 1,
                Down => current_y += 1,
                Left => current_x -= 1,
                Right => current_x += 1,
                UpLeft => { current_x -= 1; current_y -= 1; },
                UpRight => { current_x += 1; current_y -= 1; },
                DownLeft => { current_x -= 1; current_y += 1; },
                DownRight => { current_x += 1; current_y += 1; },
            }
        }
        0
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

    #[inline]
    fn get_at(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y).and_then(|row| row.get(x))
    }

    #[inline]
    fn get_at_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(y).and_then(|row| row.get_mut(x))
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.tiles.iter()
            .map(|row| {
                let a = row.iter()
                    .map(|t| {
                        write!(f, "{}", t)
                    }).collect::<Result<_, _>>();
                println!();
                a
            }).collect::<Result<_, _>>()
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

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Floor => write!(f, "."),
            EmptySeat => write!(f, "L"),
            OccupiedSeat => write!(f, "#"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Method {
    NextTo,
    LineOfSight,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}
