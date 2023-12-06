fn main() {
    let _timer = lib::PrintTimer::new("");

    let input = include_str!("./input.txt");
    let output = part1(input);

    dbg!(output);
}

#[allow(unused_variables)]
fn part1(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| {
            line.split(' ')
                .flat_map(str::parse::<u32>)
                .fold("".to_owned(), |acc, num| acc + &num.to_string())
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let game = (lines[0], lines[1]);

    let mut number_of_ways_you_can_beat_the_record = 0;

    let time = game.0;
    let best_distance = game.1;

    for hold_time in 1..time {
        let speed = hold_time;

        if (time - hold_time) * speed > best_distance {
            number_of_ways_you_can_beat_the_record += 1;
        }
    }

    number_of_ways_you_can_beat_the_record
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(include_str!("./test-input.txt"));
        assert_eq!(result, 71503);
    }
}
