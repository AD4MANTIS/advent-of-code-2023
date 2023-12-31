use std::{collections::HashSet, fmt::Display, iter};

use lib::map::prelude::*;

lib::day!(18, part1, example => 62, answer => 53300);

#[derive(Debug, Clone)]
pub struct DigInstruction {
    #[allow(dead_code)]
    color: String,
    direction: Direction,
    moves: u32,
}

#[derive(Debug, Clone)]
pub enum Tile {
    Trench(),
    Terrain,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Trench() => "#",
            Self::Terrain => ".",
        })
    }
}

fn part1(input: &str) -> usize {
    let dig_instructions = input
        .lines()
        .map(|line| line.split(' '))
        .map(|mut parts| DigInstruction {
            direction: match parts.next().unwrap() {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Top,
                "D" => Direction::Bottom,
                _ => panic!(),
            },
            moves: parts.next().unwrap().parse().unwrap(),
            color: parts
                .next()
                .unwrap()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .to_string(),
        })
        .collect::<Vec<_>>();

    const MAP_SIZE: Pos = Pos { x: 360, y: 440 };

    let mut map = Map {
        rows: iter::once(iter::once(Tile::Terrain).cycle().take(MAP_SIZE.x).collect())
            .cycle()
            .take(MAP_SIZE.y)
            .collect(),
    };

    const START_POS: Pos = Pos {
        x: 140,
        y: MAP_SIZE.y / 2,
    };

    let mut current_pos = START_POS.clone();

    *map.get_mut(&current_pos).unwrap() = Tile::Trench();

    for inst in dig_instructions {
        let move_direction = inst.direction.to_offset();

        for _ in 0..inst.moves {
            current_pos = current_pos.try_add(&move_direction).unwrap_or_else(|| {
                panic!("{:?} + {:?}", current_pos, move_direction);
            });

            *map.get_mut(&current_pos).expect("map is to small") = Tile::Trench();
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

        match tile {
            Tile::Terrain => {
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
            Tile::Trench() => {
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
    pub fn transition(&mut self, tile: &Tile) {
        *self = match *self {
            Self::Outside => match *tile {
                Tile::Trench() => Self::BorderStart,
                _ => *self,
            },
            Self::BorderStart => match *tile {
                Tile::Terrain => Self::Inside,
                _ => *self,
            },
            Self::Inside => match *tile {
                Tile::Trench() => Self::BorderEnd,
                _ => *self,
            },
            Self::BorderEnd => match *tile {
                Tile::Terrain => Self::Outside,
                _ => *self,
            },
        };
    }
}
