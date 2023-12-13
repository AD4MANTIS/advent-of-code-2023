use std::str::FromStr;

use lib::map::prelude::*;

lib::day!(00, part1,
    a raw(r"##...........
..###########
##..###..###.
##.##########
..#..........
##...#....#..
..###..##..##
##.##.#..#.##
..#.###.####.
###.###..###.
....#..##..#.
##.#.#....#.#
...####..####
....#.####.#.
##.##......##") => 1,
    example => 405,
    answer => 33047
);

fn part1(input: &str) -> usize {
    let maps = input
        .split("\n\n")
        .flat_map(Map::from_str)
        .collect::<Vec<_>>();

    maps.iter()
        .flat_map(|map| {
            get_vertical_reflection(map)
                .map(|x| x + 1)
                .or_else(|| get_horizontal_reflection(map).map(|x| (x + 1) * 100))
        })
        .sum()
}

fn get_vertical_reflection(map: &Map) -> Option<usize> {
    let mut possible_cols = (0..(map.rows[0].len() - 1)).collect::<Vec<_>>();

    for row in map.rows.iter() {
        possible_cols
            .retain(|reflect_after_col| are_chars_reflected_after_position(reflect_after_col, row));
    }

    single(&possible_cols)
}

fn get_horizontal_reflection(map: &Map) -> Option<usize> {
    let mut possible_rows = (0..(map.rows.len() - 1)).collect::<Vec<_>>();

    for col in (0..map.rows[0].len()).map(|col_id| map.column_iter(col_id).collect::<Vec<_>>()) {
        possible_rows.retain(|reflect_after_row| {
            are_chars_reflected_after_position(reflect_after_row, &col)
        });
    }

    single(&possible_rows)
}

const fn single<T: Copy>(collection: &[T]) -> Option<T> {
    match collection.len() == 1 {
        true => Some(collection[0]),
        false => None,
    }
}

fn are_chars_reflected_after_position(reflect_after_pos: &usize, chars: &[char]) -> bool {
    let mut original_row = *reflect_after_pos;
    let mut reflected_row = reflect_after_pos + 1;

    while chars.get(original_row) == chars.get(reflected_row) {
        original_row = original_row.checked_sub(1).unwrap_or(usize::MAX);
        reflected_row += 1;
    }

    chars.get(original_row).is_none() || chars.get(reflected_row).is_none()
}
