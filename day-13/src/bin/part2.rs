use std::str::FromStr;

use lib::map::prelude::*;

lib::day!(13, part2,
    example => 400,
    answer => 28806
);

fn part2(input: &str) -> usize {
    let mut maps = input
        .split("\n\n")
        .flat_map(Map::from_str)
        .collect::<Vec<_>>();

    maps.iter_mut()
        .enumerate()
        .map(|(id, map)| {
            let positions = map.all_pos();

            let initial_reflection_value = dbg!(*get_reflection_value(map).first().unwrap());

            let mut last_pos = None;
            for pos in positions {
                if let Some(last_pos) = last_pos {
                    swap_at(map, &last_pos);
                }

                swap_at(map, &pos);

                last_pos = Some(pos);

                if let Some(value) = get_reflection_value(map)
                    .into_iter()
                    .find(|value| *value != initial_reflection_value)
                {
                    return value;
                }
            }

            panic!("{id}");
        })
        .sum()
}

fn get_reflection_value(map: &Map) -> Vec<usize> {
    get_vertical_reflection(map)
        .iter()
        .map(|x| x + 1)
        .chain(get_horizontal_reflection(map).iter().map(|x| (x + 1) * 100))
        .collect::<Vec<_>>()
}

fn swap_at(map: &mut Map, pos: &Pos) {
    if let Some(elem) = map.get_mut(pos) {
        *elem = match *elem {
            '.' => '#',
            _ => '.',
        }
    }
}

fn get_vertical_reflection(map: &Map) -> Vec<usize> {
    let mut possible_cols = (0..(map.rows[0].len() - 1)).collect::<Vec<_>>();

    for row in map.rows.iter() {
        possible_cols
            .retain(|reflect_after_col| are_chars_reflected_after_position(reflect_after_col, row));
    }

    possible_cols
}

fn get_horizontal_reflection(map: &Map) -> Vec<usize> {
    let mut possible_rows = (0..(map.rows.len() - 1)).collect::<Vec<_>>();

    for col in (0..map.rows[0].len()).map(|col_id| map.column_iter(col_id).collect::<Vec<_>>()) {
        possible_rows.retain(|reflect_after_row| {
            are_chars_reflected_after_position(reflect_after_row, &col)
        });
    }

    possible_rows
}

fn are_chars_reflected_after_position(reflect_after_pos: &usize, chars: &[char]) -> bool {
    for offset in 0.. {
        let original = chars.get(reflect_after_pos.checked_sub(offset).unwrap_or(usize::MAX));
        let reflection = chars.get(reflect_after_pos + offset + 1);

        if original != reflection {
            return original.is_none() || reflection.is_none();
        }
    }

    true
}
