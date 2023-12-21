use ahash::AHashSet;

use lib::map::prelude::*;

lib::day!(21, part2, example => 16);

const STARTING_POSITION: char = 'S';
// const GARDEN_PLOTS: char = '.';
const ROCK: char = '#';

#[cfg(test)]
const STEP_COUNT: u32 = 6;
#[cfg(not(test))]
const STEP_COUNT: u32 = 64;

fn part2(input: &str) -> usize {
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

    for s in 0..STEP_COUNT {
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

        dbg!(s);

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
