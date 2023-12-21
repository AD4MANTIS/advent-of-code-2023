use std::collections::HashSet;

use lib::map::prelude::*;

lib::day!(21, part1, example => 16);

const STARTING_POSITION: char = 'S';
const GARDEN_PLOTS: char = '.';
const ROCK: char = '#';

const STEP_COUNT: u32 = 64;

fn part1(input: &str) -> usize {
    let map = FlatMap::from(Map::from(input));

    let start = map
        .elements
        .iter()
        .enumerate()
        .find(|x| *x.1 == STARTING_POSITION)
        .unwrap()
        .0;

    let width = map.width;

    let mut steps = vec![start];

    for s in 0..STEP_COUNT {
        let mut next_steps = Vec::with_capacity(steps.len());

        for pos in steps.into_iter() {
            match map.elements.get(pos) {
                Some(element) => {
                    if *element == ROCK {
                        continue;
                    }
                }
                None => continue,
            };

            next_steps.push(pos + 1);
            next_steps.push(pos + width);

            if let Some(pos) = pos.checked_sub(1) {
                next_steps.push(pos);
            }

            if let Some(pos) = pos.checked_sub(1) {
                next_steps.push(pos);
            }
        }
        dbg!(s);

        steps = next_steps;
    }
    steps.into_iter().collect::<HashSet<_>>().len()
}
