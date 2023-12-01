fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let numbers = line
                .chars()
                .filter(|char| char.is_numeric())
                .collect::<Vec<_>>();

            let number_raw = match numbers.len() {
                1 => format!("{0}{0}", numbers[0]),
                2.. => format!("{}{}", numbers[0], numbers.last().unwrap()),
                _ => panic!(),
            };

            number_raw.parse::<u32>().ok()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
