use lib::{map::prelude::*, PrintTimer};

type HashMap<K, V> = ahash::HashMap<K, V>;
type HashSet<T> = ahash::AHashSet<T>;

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

const STARTING_POSITION: char = 'S';
// const GARDEN_PLOTS: char = '.';
const ROCK: char = '#';

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MapPos {
    map_x: i16,
    map_y: i16,

    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct FlatMapPos {
    map_x: i16,
    map_y: i16,

    pos: usize,
}

// impl PartialEq for FlatMapPos {
//     fn eq(&self, other: &Self) -> bool {
//         self.pos == other.pos && self.map_x == other.map_x && self.map_y == other.map_y
//     }
// }

// impl std::hash::Hash for FlatMapPos {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         (((self.map_x as usize) << (64 - 16)) + ((self.map_y as usize) << (64 - 32)) + self.pos)
//             .hash(state);
//     }
// }

fn part2(input: &'static str) -> usize {
    steps::<26501365>(input)
}

#[allow(unused)]
fn steps<const N: usize>(input: &'static str) -> usize {
    let map = FlatMap::from(Map::from(input));

    let start = get_start_flat(&map);

    let mut steps_caches = init_cache(&map);

    let mut n = N;

    // let step_diff: usize = N / 5;
    // loop {
    //     if n % step_diff != 0 {
    //         break;
    //     }
    //
    //     n /= step_diff;
    // }

    // loop {
    for mut n in [4, N] {
        if n > N {
            break;
        }

        let timer = PrintTimer::new("");
        get_step_cache(
            &map,
            n,
            &mut steps_caches,
            if N == n { Some(start.pos) } else { None },
        );
        drop(timer);

        // n *= step_diff;
    }

    return steps_caches.get(&N).unwrap().get(&start.pos).unwrap().len();
}

type StepsCache = HashMap<usize, FieldsCache>;
type FieldsCache = HashMap<usize, HashSet<FlatMapPos>>;

fn get_step_cache(
    map: &FlatMap,
    cache_steps: usize,
    cache: &mut StepsCache,
    only_pos: Option<usize>,
) {
    let mut steps_cache = FieldsCache::default();

    let steps_count = cache
        .keys()
        .cloned()
        .filter(|steps_count| cache_steps % *steps_count == 0)
        .find(|steps_count| cache.get(&(cache_steps / *steps_count)).is_some())
        .unwrap_or(cache_steps);

    let smaller_cache = cache.get(&(cache_steps / steps_count)).unwrap();

    println!(
        "using smaller cache of {} (1/{}) steps to calculate {} steps",
        cache_steps / steps_count,
        steps_count,
        cache_steps
    );
    assert!(cache_steps % steps_count == 0);

    for pos in 0..map.elements.len() {
        if only_pos.is_some() && only_pos != Some(pos) {
            continue;
        }

        let mut steps = HashSet::from_iter([FlatMapPos {
            map_x: 0,
            map_y: 0,
            pos,
        }]);

        for _s in 0..steps_count {
            let mut next_steps = HashSet::with_capacity(steps.len());

            for step in steps {
                for cached_step in smaller_cache
                    .get(&step.pos)
                    .expect("Every field on the map should be cached")
                    .into_iter()
                {
                    next_steps.insert(FlatMapPos {
                        map_x: step.map_x + cached_step.map_x,
                        map_y: step.map_y + cached_step.map_y,
                        pos: cached_step.pos,
                    });
                }
            }

            steps = next_steps.into_iter().collect();
        }

        steps_cache.insert(pos, steps);
    }

    cache.insert(cache_steps, steps_cache);
}

fn init_cache(map: &FlatMap) -> StepsCache {
    let width = map.width;
    let height = map.height;
    let max_x = width - 1;
    let max_y = height - 1;

    let mut steps_cache: HashMap<usize, HashSet<FlatMapPos>> = Default::default();

    for pos in 0..map.elements.len() {
        let flat_map_pos = FlatMapPos {
            map_x: 0,
            map_y: 0,
            pos,
        };

        let mut next_steps = Vec::with_capacity(4);

        let map_pos = map.get_pos(flat_map_pos.pos);

        // Right
        if map_pos.x != max_x {
            next_steps.push(FlatMapPos {
                pos: flat_map_pos.pos + 1,
                ..flat_map_pos.clone()
            });
        } else {
            next_steps.push(FlatMapPos {
                map_x: flat_map_pos.map_x + 1,
                pos: flat_map_pos.pos - max_x,
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
                pos: flat_map_pos.pos + max_x,
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
        if map_pos.y != max_y {
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

        steps_cache.insert(
            pos,
            next_steps
                .into_iter()
                .filter(|map_pos| filter_rocks_flat(map, map_pos.pos))
                .collect::<HashSet<_>>(),
        );
    }

    StepsCache::from_iter([(1, steps_cache)])
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

#[cfg(test)]
mod cache_tests {
    use lib::get_test_file;

    use super::*;

    fn get_pos(
        map: &FlatMap,
        cache: &HashMap<usize, HashSet<FlatMapPos>>,
        pos: usize,
    ) -> Vec<MapPos> {
        let mut collect = cache
            .get(&pos)
            .unwrap()
            .iter()
            .map(|p| MapPos {
                map_x: p.map_x,
                map_y: p.map_y,
                x: map.get_pos(p.pos).x,
                y: map.get_pos(p.pos).y,
            })
            .collect::<Vec<_>>();

        collect.sort();

        collect
    }

    fn test<const N: usize>(pos: Option<Pos>) -> Vec<MapPos> {
        let input = get_test_file!(example);

        let map = FlatMap::from(Map::from(input));

        let mut cache = init_cache(&map);
        get_step_cache(&map, N, &mut cache, None);

        let pos = pos.map_or_else(|| get_start_flat(&map).pos, |pos| map.get_index(&pos));

        get_pos(&map, cache.get(&N).unwrap(), pos)
    }

    #[test]
    fn top_left() {
        assert_eq!(
            test::<1>(Some(Pos { x: 0, y: 0 })),
            vec![
                MapPos {
                    map_x: -1,
                    map_y: 0,
                    x: 10,
                    y: 0
                },
                MapPos {
                    map_x: 0,
                    map_y: -1,
                    x: 0,
                    y: 10
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 0,
                    y: 1
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 1,
                    y: 0
                },
            ]
        );
    }

    #[test]
    fn bottom_right() {
        assert_eq!(
            test::<1>(Some(Pos { x: 10, y: 10 })),
            vec![
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 9,
                    y: 10
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 10,
                    y: 9
                },
                MapPos {
                    map_x: 0,
                    map_y: 1,
                    x: 10,
                    y: 0
                },
                MapPos {
                    map_x: 1,
                    map_y: 0,
                    x: 0,
                    y: 10
                },
            ]
        );
    }

    #[test]
    fn step_cache_3() {
        assert_eq!(
            test::<3>(None),
            vec![
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 3,
                    y: 4
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 3,
                    y: 6
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 4,
                    y: 5
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 4,
                    y: 7
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 5,
                    y: 4
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 6,
                    y: 3
                },
            ]
        );
    }

    #[test]
    fn step_cache_6() {
        assert_eq!(
            test::<6>(None),
            vec![
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 0,
                    y: 4
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 1,
                    y: 3
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 1,
                    y: 7
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 2,
                    y: 4
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 3,
                    y: 3
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 3,
                    y: 5
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 3,
                    y: 7
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 3,
                    y: 9
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 4,
                    y: 6
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 5,
                    y: 3
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 5,
                    y: 5
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 5,
                    y: 7
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 6,
                    y: 6
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 7,
                    y: 3
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 8,
                    y: 2
                },
                MapPos {
                    map_x: 0,
                    map_y: 0,
                    x: 8,
                    y: 4
                },
            ]
        );
    }
}
