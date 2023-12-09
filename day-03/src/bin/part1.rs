lib::day!(03, part1, test => 4361);

fn part1(input: &str) -> u32 {
    let map = input
        .split_inclusive('\n')
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut number_is_adjacent_to_symbol = false;
    let mut current_number_chars = vec![];
    let mut sum = 0;

    for (current_y, current_line) in map.iter().enumerate() {
        for (current_x, current_char) in current_line.iter().enumerate() {
            if current_char.is_ascii_digit() {
                current_number_chars.push(current_char);

                number_is_adjacent_to_symbol |= check_adjacent(&map, current_x, current_y);
            } else {
                if number_is_adjacent_to_symbol && !current_number_chars.is_empty() {
                    let current_part_number = current_number_chars
                        .iter()
                        .cloned()
                        .collect::<String>()
                        .parse::<u32>()
                        .expect("should contain only ascii numbers");

                    sum += current_part_number;

                    number_is_adjacent_to_symbol = false;
                }

                current_number_chars.clear();
            }
        }
    }

    sum
}

fn check_adjacent(map: &[Vec<char>], current_position_x: usize, current_position_y: usize) -> bool {
    for y_offset in -1..=1_isize {
        for x_offset in -1..=1_isize {
            let Some(y) = current_position_y.checked_add_signed(y_offset) else {
                continue;
            };

            let Some(x) = current_position_x.checked_add_signed(x_offset) else {
                continue;
            };

            let Some(adjacent_char) = map.get(y).and_then(|line| line.get(x)) else {
                continue;
            };

            if *adjacent_char != '.' && *adjacent_char != '\n' && !adjacent_char.is_ascii_digit() {
                return true;
            }
        }
    }

    false
}
