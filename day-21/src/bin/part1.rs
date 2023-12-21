use ahash::AHashSet;

use lib::map::prelude::*;

lib::day!(21, part1, answer => 3716);

lib::day_test!(21, steps_6, example => 16);

#[cfg(test)]
fn steps_6(input: &str) -> usize {
    steps::<6>(input)
}

const STARTING_POSITION: char = 'S';
// const GARDEN_PLOTS: char = '.';
const ROCK: char = '#';

fn part1(input: &str) -> usize {
    steps::<64>(input)
}

fn steps<const N: usize>(input: &str) -> usize {
    let map = FlatMap::from(Map::from(input));

    let start = map
        .elements
        .iter()
        .enumerate()
        .find(|x| *x.1 == STARTING_POSITION)
        .unwrap()
        .0;

    let width = map.width;

    let mut steps = AHashSet::from_iter([start]);

    for _ in 0..N {
        let mut next_steps = Vec::with_capacity(steps.len());

        for pos in steps.into_iter() {
            if (pos + 1) % width != 0 {
                next_steps.push(pos + 1);
            }

            next_steps.push(pos + width);

            if pos % width != 0 {
                if let Some(pos) = pos.checked_sub(1) {
                    next_steps.push(pos);
                }
            }

            if let Some(pos) = pos.checked_sub(width) {
                next_steps.push(pos);
            }
        }

        steps = next_steps
            .into_iter()
            .filter(|pos| filter_rocks(&map, *pos))
            .collect::<AHashSet<_>>();
    }

    steps.len()
}

fn filter_rocks(map: &FlatMap, pos: usize) -> bool {
    map.elements
        .get(pos)
        .map_or(false, |element| *element != ROCK)
}
