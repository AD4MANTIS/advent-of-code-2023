lib::day!(01, part1, test => 142);

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|char| char.is_numeric())
                .collect::<Vec<_>>()
        })
        .map(|numbers| match numbers.len() {
            1 => format!("{0}{0}", numbers[0]),
            2.. => format!("{}{}", numbers[0], numbers.last().unwrap()),
            _ => panic!(),
        })
        .filter_map(|number_raw| number_raw.parse::<u32>().ok())
        .sum()
}
