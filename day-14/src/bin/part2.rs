use std::time::SystemTime;

use lib::map::{flat_map::FlatMap, prelude::*};

// lib::day!(14, part2, example => 64);
// to slow again :(

fn main() {
    part2("");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Map::from(spin_me_round::<3>(include_str!("./example-input.txt"))),
            Map::from(
                r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
            )
        );
    }
}

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
    let mut rock_pos = get_all_round_rocks(&map);
    let mut flat_map = FlatMap::from(map);

    let t = SystemTime::now();

    for i in 0..N {
        // println!("{:#?}", Map::from(flat_map.clone()));

        rock_pos.sort_unstable_by_key(|pos| pos.y * flat_map.width + pos.x);
        tilt::<0, -1>(&mut flat_map, &mut rock_pos);

        rock_pos.sort_unstable_by_key(|pos| pos.x * flat_map.height + pos.y);
        tilt::<-1, 0>(&mut flat_map, &mut rock_pos);

        rock_pos.sort_unstable_by_key(|pos| (flat_map.width - pos.y) * flat_map.width + pos.x);
        tilt::<0, 1>(&mut flat_map, &mut rock_pos);

        rock_pos.sort_unstable_by_key(|pos| (flat_map.height - pos.x) * flat_map.height + pos.y);
        tilt::<1, 0>(&mut flat_map, &mut rock_pos);

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

fn tilt<const X: isize, const Y: isize>(map: &mut FlatMap, rock_pos: &mut [Pos]) {
    let width = map.width;
    let height = map.height;

    for pos in rock_pos {
        let mut current_pos = pos.clone();

        loop {
            let mut next_pos = current_pos.clone();

            if X > 0 {
                next_pos.x += 1;
            } else if X < 0 {
                if current_pos.x == 0 {
                    break;
                }
                next_pos.x -= 1;
            } else if Y > 0 {
                next_pos.y += 1;
            } else {
                if current_pos.y == 0 {
                    break;
                }
                next_pos.y -= 1;
            }

            if next_pos.y >= height || next_pos.x >= width || map[&next_pos] != EMPTY {
                break;
            }

            current_pos = next_pos;
        }

        map.swap(&current_pos, pos);

        pos.x = current_pos.x;
        pos.y = current_pos.y;
    }
}

fn get_all_round_rocks(map: &Map) -> Vec<Pos> {
    map.all_pos()
        .into_iter()
        .filter(|pos| map.get(pos) == Some(&ROUND_ROCK))
        .collect()
}
