fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let numbers = replace_text_with_number(line);

            let number_raw = format!(
                "{}{}",
                numbers[0],
                numbers
                    .last()
                    .expect("line should contain at lease one 'number'")
            );

            number_raw.parse::<u32>().ok()
        })
        .sum()
}

#[allow(dead_code)]
fn replace_text_with_number_first_try(mut line: String) -> String {
    let mut index = 0;

    while index < line.len() {
        let truncated_line = line[index..line.len()].to_string();

        for text_num in [
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ] {
            if truncated_line.starts_with(text_num.0) {
                line = line.replacen(text_num.0, text_num.1, 1);
                break;
            }
        }

        index += 1;
    }

    line
}

fn replace_text_with_number(line: &str) -> Vec<u32> {
    (0..line.len())
        .filter_map(|index| {
            if let Ok(number) = line[index..=index].parse::<u32>() {
                return Some(number);
            }

            for text_num in [
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ] {
                if line[index..line.len()].starts_with(text_num.0) {
                    return Some(text_num.1);
                }
            }

            None
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }

    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = part2(input);
        assert_eq!(result, 54530);
    }
}
