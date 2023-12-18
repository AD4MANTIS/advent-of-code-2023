#![allow(unused)]
use std::{collections::HashSet, fmt::Display, iter};

use lib::map::prelude::*;

// to much RAM
// lib::day!(18, part2, example => 952408144115);

fn main() {}

#[derive(Debug, Clone)]
pub struct DigInstruction {
    direction: Direction,
    moves: u32,
}

const TRENCH: bool = true;
const TERRAIN: bool = false;

fn part2(input: &str) -> usize {
    let dig_instructions = input
        .lines()
        .map(|line| line.split(' '))
        .map(|mut x| x.nth(2).unwrap())
        .map(|code| DigInstruction {
            direction: match code.trim_end_matches(')').chars().last().unwrap() {
                '0' => Direction::Right,
                '2' => Direction::Left,
                '3' => Direction::Top,
                '1' => Direction::Bottom,
                _ => panic!("{:?}", code.chars().last()),
            },
            moves: u32::from_str_radix(&code.chars().skip(2).take(5).collect::<String>(), 16)
                .unwrap(),
        })
        .collect::<Vec<_>>();

    const MAP_SIZE: Pos = Pos { x: 1000, y: 1000 };

    let mut map = Map {
        rows: iter::once(iter::once(TERRAIN).cycle().take(MAP_SIZE.x).collect())
            .cycle()
            .take(MAP_SIZE.y)
            .collect(),
    };

    const START_POS: Pos = Pos {
        x: MAP_SIZE.x / 2,
        y: MAP_SIZE.y / 2,
    };

    let mut current_pos = START_POS.clone();

    *map.get_mut(&current_pos).unwrap() = TRENCH;

    for inst in dig_instructions {
        let move_direction = inst.direction.to_offset();

        for _ in 0..inst.moves {
            current_pos = current_pos.try_add(&move_direction).unwrap_or_else(|| {
                panic!("{:?} + {:?}", current_pos, move_direction);
            });

            *map.get_mut(&current_pos).expect("map is to small") = TRENCH;
        }
    }

    let mut hole_size = 0;

    let mut inside_positions = std::collections::VecDeque::from([START_POS + Pos { x: 1, y: 1 }]);
    let mut traversed_positions = HashSet::new();

    while let Some(pos) = inside_positions.pop_front() {
        if traversed_positions.contains(&pos) {
            continue;
        }

        let Some(tile) = map.get(&pos) else {
            continue;
        };

        match *tile {
            TERRAIN => {
                hole_size += 1;

                for offset in [
                    Offset::x(-1),
                    Offset::x(1),
                    Offset::y(-1),
                    Offset::y(1),
                    Offset { x: -1, y: -1 },
                    Offset { x: -1, y: 1 },
                    Offset { x: 1, y: -1 },
                    Offset { x: 1, y: 1 },
                ] {
                    if let Some(next_pos) = pos.try_add(&offset) {
                        inside_positions.push_back(next_pos);
                    }
                }
            }
            TRENCH => {
                hole_size += 1;
            }
        };

        traversed_positions.insert(pos);
    }

    hole_size
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Outside,
    BorderStart,
    Inside,
    BorderEnd,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::Outside => ".",
            Self::BorderStart => "<",
            Self::Inside => "X",
            Self::BorderEnd => ">",
        })
    }
}

impl State {
    pub fn transition(&mut self, tile: &bool) {
        *self = match *self {
            Self::Outside => match *tile {
                TRENCH => Self::BorderStart,
                _ => *self,
            },
            Self::BorderStart => match *tile {
                TERRAIN => Self::Inside,
                _ => *self,
            },
            Self::Inside => match *tile {
                TRENCH => Self::BorderEnd,
                _ => *self,
            },
            Self::BorderEnd => match *tile {
                TERRAIN => Self::Outside,
                _ => *self,
            },
        };
    }
}
