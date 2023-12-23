#![allow(unused)]

use lib::map::prelude::*;

lib::day_main!(21, part2);

#[cfg(test)]
mod tests {
    use lib::get_test_file;

    use super::*;

    // #[test]
    // fn steps_6() {
    //     let input = get_test_file!(example);
    //     assert_eq!(steps::<6>(input), 16);
    // }

    // #[test]
    // fn steps_10() {
    //     let input = get_test_file!(example);
    //     assert_eq!(steps::<10>(input), 50);
    // }

    // #[test]
    // fn steps_50() {
    //     let input = get_test_file!(example);
    //     assert_eq!(steps::<50>(input), 1594);
    // }

    // #[test]
    // fn steps_100() {
    //     let input = get_test_file!(example);
    //     assert_eq!(steps::<100>(input), 6536);
    // }

    // #[test]
    // fn steps_500() {
    //     let input = get_test_file!(example);
    //     assert_eq!(steps::<500>(input), 167004);
    // }

    // #[test]
    // fn steps_1000() {
    //     let input = get_test_file!(example);
    //     assert_eq!(steps::<1000>(input), 668697);
    // }

    // #[test]
    // fn steps_5000() {
    //     let input = get_test_file!(example);
    //     assert_eq!(steps::<5000>(input), 16733044);
    // }
}

type HashSet<T> = ahash::AHashSet<T>;

const STARTING_POSITION: char = 'S';
// const GARDEN_PLOTS: char = '.';
const ROCK: char = '#';

fn part2(input: &'static str) -> usize {
    steps::<26501365>(input)
}

fn steps<const N: usize>(input: &'static str) -> usize {
    let repeat_map = match N {
        _ if N > 500 => 500,
        _ if N > 100 => 100,
        _ if N > 10 => 40,
        _ => 10,
    };

    let mut map = Map::from(input);
    let initial_width = map.width();
    let initial_height = map.height();

    let row = map.rows.clone();
    for _ in 0..repeat_map {
        map.rows.append(&mut row.clone());
    }

    map.rows
        .iter_mut()
        .for_each(|row| *row = row.repeat(repeat_map));

    // let map = FlatMap::from(map);

    let start = get_start(&map, initial_width, repeat_map, initial_height);

    // let width = map.width() as u32;

    let mut steps = HashSet::from_iter([start]);

    let directions = [
        Direction::Top.to_offset(),
        Direction::Left.to_offset(),
        Direction::Right.to_offset(),
        Direction::Bottom.to_offset(),
    ];

    for _ in 0..N {
        let mut next_steps = Vec::with_capacity(steps.len());

        for pos in steps.into_iter() {
            for offset in directions.iter() {
                if let Some(pos) = pos.try_add(offset) {
                    next_steps.push(pos);
                }
            }
        }

        steps = next_steps
            .into_iter()
            .filter(|pos| filter_rocks(&map, pos))
            .collect::<HashSet<_>>();
    }

    steps.len()
}

fn get_start_flat(
    map: &FlatMap,
    initial_width: usize,
    repeat_map: usize,
    initial_height: usize,
) -> u32 {
    let start = map
        .elements
        .iter()
        .enumerate()
        .find(|x| *x.1 == STARTING_POSITION)
        .unwrap()
        .0;

    let mut start = map.get_pos(start);
    start.x += initial_width * repeat_map / 2;
    start.y += initial_height * repeat_map / 2;

    map.get_index(&start) as u32
}

fn get_start(map: &Map, initial_width: usize, initial_height: usize, repeat_map: usize) -> Pos {
    let mut start = map
        .all_pos_iter()
        .find(|x| *map.get(x).unwrap() == STARTING_POSITION)
        .unwrap();

    start.x += initial_width * repeat_map / 2;
    start.y += initial_height * repeat_map / 2;

    start
}

fn filter_rocks_flat(map: &FlatMap, pos: usize) -> bool {
    map.elements
        .get(pos)
        .map_or(false, |element| *element != ROCK)
}

fn filter_rocks(map: &Map, pos: &Pos) -> bool {
    map.get(pos).map_or(false, |element| *element != ROCK)
}
