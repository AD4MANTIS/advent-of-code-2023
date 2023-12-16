use std::collections::{hash_map::RandomState, HashSet};

use lib::map::prelude::*;

lib::day!(16, part1, example => 46, answer => 8125);

fn part1(input: &str) -> usize {
    let map = Map::from(input);

    let mut beams = vec![Beam {
        pos: Pos::default(),
        direction: Direction::Right,
    }];
    let mut visited_field: HashSet<Beam, RandomState> = HashSet::from_iter(beams.iter().cloned());

    while !beams.is_empty() {
        for beam_id in (0..beams.len()).rev() {
            let beam = beams.get_mut(beam_id).unwrap();

            let result = move_beam(beam, &map);

            if result.beam_ended || visited_field.contains(beam) {
                beams.remove(beam_id);
            } else {
                visited_field.insert(beam.clone());
            }

            if let Some(new_beam) = result.new_beam {
                visited_field.insert(new_beam.clone());
                beams.push(new_beam);
            }
        }

        dbg!(&beams);
    }

    let set = visited_field
        .into_iter()
        .map(|field| field.pos)
        .filter(|pos| pos.y < map.height() && pos.x < map.width())
        .collect::<HashSet<_>>();

    set.len()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Beam {
    pos: Pos,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct MoveResult {
    new_beam: Option<Beam>,
    beam_ended: bool,
}

fn move_beam(beam: &mut Beam, map: &Map) -> MoveResult {
    let mut result = MoveResult {
        new_beam: None,
        beam_ended: false,
    };

    let mut new_beam_direction = None::<Direction>;

    let current = map.get(&beam.pos);

    match current {
        Some('/') => {
            beam.direction = match beam.direction {
                Direction::Top => Direction::Right,
                Direction::Left => Direction::Bottom,
                Direction::Right => Direction::Top,
                Direction::Bottom => Direction::Left,
            }
        }
        Some('\\') => {
            beam.direction = match beam.direction {
                Direction::Top => Direction::Left,
                Direction::Left => Direction::Top,
                Direction::Right => Direction::Bottom,
                Direction::Bottom => Direction::Right,
            }
        }
        Some('|') => {
            match beam.direction {
                Direction::Left | Direction::Right => {
                    beam.direction = Direction::Top;
                    new_beam_direction = Some(Direction::Bottom);
                }
                _ => {}
            };
        }
        Some('-') => {
            match beam.direction {
                Direction::Top | Direction::Bottom => {
                    beam.direction = Direction::Right;
                    new_beam_direction = Some(Direction::Left);
                }
                _ => {}
            };
        }
        Some(_) => {}
        None => {
            result.beam_ended = true;
        }
    }

    if let Some(new_beam_direction) = new_beam_direction {
        if let Some(new_beam_pos) = beam.pos.try_add(&new_beam_direction.to_offset()) {
            result.new_beam = Some(Beam {
                pos: new_beam_pos,
                direction: new_beam_direction,
            });
        }
    }

    if let Some(next_pos) = beam.pos.try_add(&beam.direction.to_offset()) {
        beam.pos = next_pos;
    } else {
        result.beam_ended = true;
    }

    result
}
