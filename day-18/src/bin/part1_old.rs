use std::{fmt::Display, iter};

use lib::map::prelude::*;

lib::day!(18, part1, example => 62);

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

    #[cfg(test)]
    const MAP_SIZE: Pos = Pos { x: 10, y: 15 };

    #[cfg(not(test))]
    const MAP_SIZE: Pos = Pos { x: 360, y: 440 };

    let mut map = Map {
        rows: iter::once(iter::once(Tile::Terrain).cycle().take(MAP_SIZE.x).collect())
            .cycle()
            .take(MAP_SIZE.y)
            .collect(),
    };

    let mut state_map = Map {
        rows: iter::once(
            iter::once(State::Outside)
                .cycle()
                .take(MAP_SIZE.x)
                .collect(),
        )
        .cycle()
        .take(MAP_SIZE.y)
        .collect(),
    };

    #[cfg(test)]
    let mut current_pos = Pos { x: 1, y: 0 };

    #[cfg(not(test))]
    let mut current_pos = Pos {
        x: 140,
        y: MAP_SIZE.y / 2,
    };

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
    for (row_i, row) in map.rows.iter().enumerate() {
        let mut state = State::Outside;

        let mut inside_temp_size = 0;

        for (tile_i, tile) in row.iter().enumerate() {
            state.transition(tile);

            state_map.rows[row_i][tile_i] = state;

            if !matches!(state, State::Outside) {
                match state {
                    State::Outside => todo!(),

                    State::BorderStart => hole_size += 1,
                    State::Inside => inside_temp_size += 1,
                    State::BorderEnd => {
                        hole_size += inside_temp_size + 1;
                        inside_temp_size = 0;
                    }
                }
            }
        }
    }

    println!("{:#?}", map);
    println!("{:#?}", state_map);

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
