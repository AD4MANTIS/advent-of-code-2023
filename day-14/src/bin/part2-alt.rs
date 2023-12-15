#![allow(unused)]

/*
    let map = Map::from(input);
    // let positions = AllPositions::from(&map);
    let mut rock_pos = get_all_round_rocks(&map);

    let all_pos = map.all_pos();

    let mut flat_map = FlatMap::from(map);

    let a = all_pos
        .iter()
        .map(|pos| (flat_map.get_index(pos), pos.y * flat_map.width + pos.x))
        .collect::<HashMap<_, _>>();
    let b = all_pos
        .iter()
        .map(|pos| (flat_map.get_index(pos), pos.x * flat_map.height + pos.y))
        .collect::<HashMap<_, _>>();
    let c = all_pos
        .iter()
        .map(|pos| {
            (
                flat_map.get_index(pos),
                (flat_map.width - pos.y) * flat_map.width + pos.x,
            )
        })
        .collect::<HashMap<_, _>>();
    let d = all_pos
        .iter()
        .map(|pos| {
            (
                flat_map.get_index(pos),
                (flat_map.height - pos.x) * flat_map.height + pos.y,
            )
        })
        .collect::<HashMap<_, _>>();

    let t = SystemTime::now();

    for i in 0..N {
        // println!("{:#?}", Map::from(flat_map.clone()));

        rock_pos.sort_unstable_by_key(|pos| a[&flat_map.get_index(pos)]);
        tilt::<0, -1>(&mut flat_map, &mut rock_pos);

        rock_pos.sort_unstable_by_key(|pos| b[&flat_map.get_index(pos)]);
        tilt::<-1, 0>(&mut flat_map, &mut rock_pos);

        rock_pos.sort_unstable_by_key(|pos| c[&flat_map.get_index(pos)]);
        tilt::<0, 1>(&mut flat_map, &mut rock_pos);

        rock_pos.sort_unstable_by_key(|pos| d[&flat_map.get_index(pos)]);
        tilt::<1, 0>(&mut flat_map, &mut rock_pos);
*/

fn main() {}

use std::time::SystemTime;

use lib::map::{flat_map::FlatMap, prelude::*};

// lib::day!(14, part2, example => 64);

const ROUND_ROCK: char = 'O';
const SQUARE_ROCK: char = '#';
const EMPTY: char = '.';

fn part2(input: &str) -> usize {
    part2_nums::<1_000_000_000>(input)
}

fn part2_nums<const N: usize>(input: &str) -> usize {
    let map = spin_me_round::<N>(input);

    let height = map.height;

    get_all_round_rocks(&map.clone().into())
        .into_iter()
        .map(|pos| match map[&pos] {
            ROUND_ROCK => height - pos.y,
            _ => 0,
        })
        .sum()
}

fn spin_me_round<const N: usize>(input: &str) -> FlatMap {
    let map = Map::from(input);
    // let positions = AllPositions::from(&map);
    let mut flat_map = FlatMap::from(map);
    let mut rock_pos = get_all_round_rocks_flat_map(&flat_map);

    let width = flat_map.width;
    let t = SystemTime::now();

    for i in 0..N {
        // println!("{:#?}", Map::from(flat_map.clone()));

        rock_pos.sort_unstable_by_key(|pos| *pos);
        tilt(&mut flat_map, &mut rock_pos, -(width as isize));

        rock_pos.sort_unstable_by_key(|pos| pos % width);
        tilt(&mut flat_map, &mut rock_pos, -1);

        rock_pos.sort_unstable_by_key(|pos| -(*pos as isize));
        tilt(&mut flat_map, &mut rock_pos, width as isize);

        rock_pos.sort_unstable_by_key(|pos| width - (pos % width));
        tilt(&mut flat_map, &mut rock_pos, 1);

        if i % 1_000_000 == 0 {
            println!(
                "{}s {}%",
                t.elapsed().unwrap_or_default().as_secs_f64(),
                i as f64 / N as f64
            );
        }
    }

    flat_map
}

struct AllPositions {
    pos_for_north_tilt: Vec<Pos>,
    pos_for_west_tilt: Vec<Pos>,
    pos_for_south_tilt: Vec<Pos>,
    pos_for_east_tilt: Vec<Pos>,
}

impl From<&Map> for AllPositions {
    fn from(map: &Map) -> Self {
        let mut all_pos = map.all_pos();
        all_pos.retain(|pos| map.get(pos) != Some(&SQUARE_ROCK));
        let mut all_pos_horizontal = all_pos.clone();

        all_pos_horizontal.sort_by_key(|pos| pos.x * 1000 + pos.y);

        let mut pos = Self {
            pos_for_north_tilt: all_pos.clone(),
            pos_for_south_tilt: {
                all_pos.reverse();
                all_pos
            },
            pos_for_west_tilt: all_pos_horizontal.clone(),
            pos_for_east_tilt: {
                all_pos_horizontal.reverse();
                all_pos_horizontal
            },
        };

        // Remove first lines in each Vec because nothing can move further;
        pos.pos_for_north_tilt.retain(|pos| pos.y != 0);
        pos.pos_for_west_tilt.retain(|pos| pos.x != 0);
        pos.pos_for_south_tilt
            .retain(|pos| pos.y != map.height() - 1);
        pos.pos_for_east_tilt.retain(|pos| pos.x != map.width() - 1);

        pos
    }
}

fn tilt(map: &mut FlatMap, rock_pos: &mut [usize], offset: isize) {
    for pos in rock_pos {
        let mut current_pos = *pos;

        loop {
            let mut next_pos = current_pos;

            if offset < 0 && (-offset) as usize > next_pos {
                break;
            }

            if offset > 0 {
                next_pos += offset as usize;
            } else {
                next_pos -= -offset as usize;
            }

            if next_pos >= map.elements.len() || map.elements[next_pos] != EMPTY {
                break;
            }

            current_pos = next_pos;
        }

        map.elements.swap(current_pos, *pos);

        *pos = current_pos;
    }
}

fn get_all_round_rocks(map: &Map) -> Vec<Pos> {
    map.all_pos()
        .into_iter()
        .filter(|pos| map.get(pos) == Some(&ROUND_ROCK))
        .collect()
}

fn get_all_round_rocks_flat_map(map: &FlatMap) -> Vec<usize> {
    map.elements
        .iter()
        .enumerate()
        .filter(|pos| *pos.1 == ROUND_ROCK)
        .map(|pos| pos.0)
        .collect()
}
