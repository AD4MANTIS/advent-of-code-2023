use lib::map::prelude::*;

lib::day!(14, part1, example => 136, answer => 113456);

const ROUND_ROCK: char = 'O';
#[allow(unused)]
const SQUARE_ROCK: char = '#';
const EMPTY: char = '.';

fn part1(input: &str) -> usize {
    let mut map = Map::from(input);

    for pos in get_all_round_rocks(&map) {
        if pos.y == 0 || map.get(&pos) != Some(&ROUND_ROCK) {
            continue;
        }

        let mut current_pos = pos.clone();

        while let Some(next_pos) = current_pos.try_add(&Offset::y(-1)) {
            if map.get(&next_pos) != Some(&EMPTY) {
                break;
            }

            map.swap(&current_pos, &next_pos);

            current_pos = next_pos;
        }
    }

    let height = map.height();

    get_all_round_rocks(&map)
        .into_iter()
        .map(|pos| match map.get(&pos) {
            Some(&ROUND_ROCK) => height - pos.y,
            _ => 0,
        })
        .sum()
}

fn get_all_round_rocks(map: &Map) -> Vec<Pos> {
    map.all_pos()
        .into_iter()
        .filter(|pos| map.get(pos) == Some(&ROUND_ROCK))
        .collect()
}
