use std::time::SystemTime;

fn main() {
    let start_time = SystemTime::now();

    let input = include_str!("./input.txt");
    let output = part1(input);

    println!(
        "Output = {output} (Duration: {})",
        start_time.elapsed().unwrap_or_default().as_secs_f64()
    );
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
        .collect::<Vec<_>>()
        .chunks(2)
        .flat_map(|seed_range| seed_range[0]..(seed_range[0] + seed_range[1]))
        .collect::<Vec<_>>();

    for block in blocks {
        map_numbers(&mut seeds, &parse_map_block(block));
    }

    seeds.iter().min().cloned().unwrap_or_default()
}

struct MapLine {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl MapLine {
    pub fn contains_source(&self, source: usize) -> bool {
        source >= self.source_range_start && source < self.source_range_start + self.range_length
    }

    pub fn map(&self, source: usize) -> usize {
        source - self.source_range_start + self.destination_range_start
    }
}

fn parse_map_block(block: &str) -> Vec<MapLine> {
    block
        .lines()
        // first line describes what map this is
        .skip(1)
        .map(|line| {
            line.split(' ')
                .flat_map(str::parse::<usize>)
                .collect::<Vec<_>>()
        })
        .map(|line| {
            assert_eq!(line.len(), 3);

            MapLine {
                destination_range_start: line[0],
                source_range_start: line[1],
                range_length: line[2],
            }
        })
        .collect()
}

fn map_numbers(source_numbers: &mut [usize], lines: &[MapLine]) {
    for source_number in source_numbers {
        for line in lines {
            if line.contains_source(*source_number) {
                *source_number = line.map(*source_number);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(include_str!("./test-input.txt"));
        assert_eq!(result, 46);
    }
}
