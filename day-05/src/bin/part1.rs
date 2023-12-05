use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[allow(unused_variables)]
fn part1(input: &str) -> usize {
    let mut blocks = input.split("\n\n");

    let mut seeds = blocks
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .flat_map(str::parse::<usize>)
        .collect::<Vec<_>>();

    for block in blocks {
        let map = parse_map_block(block);

        dbg!(seeds.clone());

        for seed in seeds.iter_mut() {
            *seed = *map.get(seed).unwrap_or(seed);
        }
    }

    seeds.iter().min().cloned().unwrap_or_default()
}

fn parse_map_block(block: &str) -> HashMap<usize, usize> {
    block
        .lines()
        // first line describes what map this is
        .skip(1)
        .map(|line| {
            line.split(' ')
                .flat_map(str::parse::<usize>)
                .collect::<Vec<_>>()
        })
        .flat_map(|line| {
            assert_eq!(line.len(), 3);

            let destination_range_start = line[0];
            let source_range_start = line[1];
            let range_length = line[2];

            (0..range_length).map(move |number_offset| {
                dbg!((
                    source_range_start + number_offset,
                    destination_range_start + number_offset,
                ))
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(include_str!("./test-input.txt"));
        assert_eq!(result, 35);
    }
}
