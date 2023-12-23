use lib::map::prelude::*;

lib::day_main!(21, part2);

#[cfg(test)]
mod tests {
    use lib::get_test_file;

    use super::*;

    #[test]
    fn steps_6() {
        let input = get_test_file!(example);
        assert_eq!(steps::<6>(input), 16);
    }

    #[test]
    fn steps_10() {
        let input = get_test_file!(example);
        assert_eq!(steps::<10>(input), 50);
    }

    #[test]
    fn steps_50() {
        let input = get_test_file!(example);
        assert_eq!(steps::<50>(input), 1594);
    }

    #[test]
    fn steps_100() {
        let input = get_test_file!(example);
        assert_eq!(steps::<100>(input), 6536);
    }

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MapPos {
    map_x: isize,
    map_y: isize,

    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FlatMapPos {
    map_x: isize,
    map_y: isize,

    pos: usize,
}

fn part2(input: &'static str) -> usize {
    steps::<26501365>(input)
}

fn steps<const N: usize>(input: &'static str) -> usize {
    let map = Map::from(input);
    let width = map.width();
    let height = map.height();

    let map = FlatMap::from(map);

    let start = get_start_flat(&map);

    let mut steps = HashSet::from_iter([start]);

    for _s in 0..N {
        // dbg!(_s);

        let mut next_steps = Vec::with_capacity(steps.len());

        for flat_map_pos in steps.into_iter() {
            let map_pos = map.get_pos(flat_map_pos.pos);

            // Right
            if map_pos.x != width - 1 {
                next_steps.push(FlatMapPos {
                    pos: flat_map_pos.pos + 1,
                    ..flat_map_pos.clone()
                });
            } else {
                next_steps.push(FlatMapPos {
                    map_x: flat_map_pos.map_x + 1,
                    pos: flat_map_pos.pos - (width - 1),
                    ..flat_map_pos.clone()
                });
            }

            // Left
            if map_pos.x != 0 {
                next_steps.push(FlatMapPos {
                    pos: flat_map_pos.pos - 1,
                    ..flat_map_pos.clone()
                });
            } else {
                next_steps.push(FlatMapPos {
                    pos: flat_map_pos.pos + (width - 1),
                    map_x: flat_map_pos.map_x - 1,
                    ..flat_map_pos.clone()
                });
            }

            // Up
            if map_pos.y != 0 {
                next_steps.push(FlatMapPos {
                    pos: flat_map_pos.pos - width,
                    ..flat_map_pos.clone()
                });
            } else {
                next_steps.push(FlatMapPos {
                    pos: flat_map_pos.pos + (width * (height - 1)),
                    map_y: flat_map_pos.map_y - 1,
                    ..flat_map_pos.clone()
                });
            }

            // Down
            if map_pos.y != height - 1 {
                next_steps.push(FlatMapPos {
                    pos: flat_map_pos.pos + width,
                    ..flat_map_pos.clone()
                });
            } else {
                next_steps.push(FlatMapPos {
                    pos: flat_map_pos.pos % width,
                    map_y: flat_map_pos.map_y + 1,
                    ..flat_map_pos.clone()
                });
            }
        }

        steps = next_steps
            .into_iter()
            .filter(|map_pos| filter_rocks_flat(&map, map_pos.pos))
            .collect::<HashSet<_>>();
    }

    steps.len()
}

fn get_start_flat(map: &FlatMap) -> FlatMapPos {
    FlatMapPos {
        map_x: 0,
        map_y: 0,
        pos: map
            .elements
            .iter()
            .enumerate()
            .find(|x| *x.1 == STARTING_POSITION)
            .unwrap()
            .0,
    }
}

fn filter_rocks_flat(map: &FlatMap, pos: usize) -> bool {
    map.elements
        .get(pos)
        .map_or(false, |element| *element != ROCK)
}
