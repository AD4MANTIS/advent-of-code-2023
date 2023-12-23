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

impl MapPos {
    const fn get_pos(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y,
        }
    }
}

fn part2(input: &'static str) -> usize {
    steps::<26501365>(input)
}

fn steps<const N: usize>(input: &'static str) -> usize {
    let map = Map::from(input);
    let width = map.width();
    let height = map.height();

    // let map = FlatMap::from(map);

    let start = get_start(&map);

    // let width = map.width() as u32;

    let mut steps = HashSet::from_iter([start]);

    for _ in 0..N {
        let mut next_steps = Vec::with_capacity(steps.len());

        for map_pos in steps.into_iter() {
            // Right
            if map_pos.x != width - 1 {
                next_steps.push(MapPos {
                    x: map_pos.x + 1,
                    ..map_pos.clone()
                });
            } else {
                next_steps.push(MapPos {
                    map_x: map_pos.map_x + 1,
                    x: 0,
                    ..map_pos.clone()
                });
            }

            // Left
            if map_pos.x != 0 {
                next_steps.push(MapPos {
                    x: map_pos.x - 1,
                    ..map_pos.clone()
                });
            } else {
                next_steps.push(MapPos {
                    x: width - 1,
                    map_x: map_pos.map_x - 1,
                    ..map_pos.clone()
                });
            }

            // Up
            if map_pos.y != 0 {
                next_steps.push(MapPos {
                    y: map_pos.y - 1,
                    ..map_pos.clone()
                });
            } else {
                next_steps.push(MapPos {
                    y: height - 1,
                    map_y: map_pos.map_y - 1,
                    ..map_pos.clone()
                });
            }

            // Down
            if map_pos.y != height - 1 {
                next_steps.push(MapPos {
                    y: map_pos.y + 1,
                    ..map_pos.clone()
                });
            } else {
                next_steps.push(MapPos {
                    y: 0,
                    map_y: map_pos.map_y + 1,
                    ..map_pos.clone()
                });
            }
        }

        steps = next_steps
            .into_iter()
            .filter(|map_pos| filter_rocks(&map, &map_pos.get_pos()))
            .collect::<HashSet<_>>();
    }

    steps.len()
}

fn get_start(map: &Map) -> MapPos {
    let pos = map
        .all_pos_iter()
        .find(|x| *map.get(x).unwrap() == STARTING_POSITION)
        .expect("Start pos should be found");

    MapPos {
        map_x: 0,
        map_y: 0,
        x: pos.x,
        y: pos.y,
    }
}

fn filter_rocks(map: &Map, pos: &Pos) -> bool {
    map.get(pos).map_or(false, |element| *element != ROCK)
}
