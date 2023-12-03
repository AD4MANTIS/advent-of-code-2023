use std::char;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    let map = Map(input
        .split_inclusive('\n')
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>());

    let mut sum = 0;

    for (current_y, current_line) in map.0.iter().enumerate() {
        for (current_x, current_char) in current_line.iter().enumerate() {
            if *current_char != '*' {
                continue;
            }

            let mut adjacent_numbers = Vec::<PartNumber>::new();

            for adjacent_pos in iter_adjacent(&Pos {
                x: current_x,
                y: current_y,
            }) {
                let Some(num) = get_number_from_pos(&map, &adjacent_pos) else {
                    continue;
                };

                if adjacent_numbers
                    .iter()
                    .all(|a| a.start_pos != num.start_pos)
                {
                    adjacent_numbers.push(num);
                }
            }

            if adjacent_numbers.len() == 2 {
                sum += adjacent_numbers[0].number * adjacent_numbers[1].number;
            }
        }
    }

    sum
}

struct Map(Vec<Vec<char>>);

impl Map {
    pub fn get_at(&self, pos: &Pos) -> Option<&char> {
        self.0.get(pos.y).and_then(|line| line.get(pos.x))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

fn iter_adjacent(current_position: &Pos) -> Vec<Pos> {
    let mut positions = vec![];

    for y_offset in -1..=1_isize {
        for x_offset in -1..=1_isize {
            let Some(y) = current_position.y.checked_add_signed(y_offset) else {
                continue;
            };

            let Some(x) = current_position.x.checked_add_signed(x_offset) else {
                continue;
            };

            let pos = Pos { x, y };

            positions.push(pos);
        }
    }

    positions
}

#[derive(Debug, Clone)]
struct PartNumber {
    start_pos: Pos,
    number: usize,
}

fn get_number_from_pos(map: &Map, pos: &Pos) -> Option<PartNumber> {
    let mut pos = pos.clone();

    let start_pos = {
        let Some(current_char) = map.get_at(&pos) else {
            return None;
        };

        if !current_char.is_ascii_digit() {
            return None;
        }
        pos.x -= 1;

        loop {
            if map.get_at(&pos).map(char::is_ascii_digit) != Some(true) {
                pos.x += 1;

                break;
            }

            let Some(x) = pos.x.checked_sub(1) else {
                break;
            };

            pos.x = x;
        }

        pos.clone()
    };

    let mut current_number_chars = vec![];

    while let Some(char) = map.get_at(&pos) {
        if !char.is_ascii_digit() {
            break;
        }

        current_number_chars.push(char);

        pos.x += 1;
    }

    if current_number_chars.is_empty() {
        return None;
    }

    Some(PartNumber {
        start_pos,
        number: parse_chars_to_usize(&current_number_chars),
    })
}

fn parse_chars_to_usize(chars: &[&char]) -> usize {
    chars
        .iter()
        .cloned()
        .collect::<String>()
        .parse()
        .expect("should contain only ascii numbers")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_works() {
        let result = part2(include_str!("./test-input1.txt"));
        assert_eq!(result, 467835);
    }

    #[test]
    fn it_works() {
        let result = part2(include_str!("./input1.txt"));
        assert_eq!(result, 91031374);
    }
}
