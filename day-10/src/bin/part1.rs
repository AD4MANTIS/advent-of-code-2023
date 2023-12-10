use std::ops::Add;

lib::day!(10, part1, example => 8, answer => 6903);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Add<(isize, isize)> for Pos {
    type Output = Self;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Self {
            x: self.x.saturating_add_signed(rhs.0),
            y: self.y.saturating_add_signed(rhs.1),
        }
    }
}

fn part1(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = find_start(&map);

    let mut last_position = start.clone();
    let mut current_position = find_next_pos_from_start(&map, &start);

    let mut distance = 0;

    while current_position != start {
        let next_position = find_next_pos(&map, &last_position, &current_position);
        last_position = current_position;
        current_position = next_position;

        distance += 1;
    }

    (distance / 2) + (distance % 2)
}

type Map = [Vec<char>];

fn get_pos(map: &Map, pos: &Pos) -> Option<char> {
    let row = map.get(pos.y)?;
    return row.get(pos.x).cloned();
}

fn find_start(map: &Map) -> Pos {
    for row in map.iter().enumerate() {
        for char in row.1.iter().enumerate() {
            if *char.1 == 'S' {
                return Pos {
                    x: char.0,
                    y: row.0,
                };
            }
        }
    }

    panic!("start not found");
}

fn find_next_pos_from_start(map: &Map, current_pos: &Pos) -> Pos {
    macro_rules! try_find_next_pos {
        ($offset: expr, $valid_pipes: pat) => {
            let next_pos = current_pos.clone() + $offset;

            match get_pos(map, &next_pos) {
                Some($valid_pipes) => return next_pos,
                _ => {}
            }
        };
    }

    try_find_next_pos!((0, 1), '|' | 'J' | 'L');

    try_find_next_pos!((0, -1), '|' | 'F' | '7');

    try_find_next_pos!((1, 0), '-' | 'J' | '7');

    try_find_next_pos!((-1, 0), '-' | 'L' | 'F');

    panic!(
        "The current Pipe at {:?} doesn't lead anywhere!",
        current_pos
    );
}

fn find_next_pos(map: &Map, last_position: &Pos, current_pos: &Pos) -> Pos {
    let pos = get_pos(map, current_pos).unwrap();

    current_pos.clone()
        + match pos {
            '|' => (0, current_pos.y as isize - last_position.y as isize),
            'J' => (
                ternary!(current_pos.x == last_position.x => -1, 0),
                ternary!(current_pos.y == last_position.y => -1, 0),
            ),
            'L' => (
                ternary!(current_pos.x == last_position.x => 1, 0),
                ternary!(current_pos.y == last_position.y => -1, 0),
            ),
            'F' => (
                ternary!(current_pos.x == last_position.x => 1, 0),
                ternary!(current_pos.y == last_position.y => 1, 0),
            ),
            '7' => (
                ternary!(current_pos.x == last_position.x => -1, 0),
                ternary!(current_pos.y == last_position.y => 1, 0),
            ),
            '-' => (current_pos.x as isize - last_position.x as isize, 0),
            _ => panic!(),
        }
}

#[macro_export]
macro_rules! ternary {
    ($if: expr => $then: expr, $else: expr) => {
        if $if {
            $then
        } else {
            $else
        }
    };
}
