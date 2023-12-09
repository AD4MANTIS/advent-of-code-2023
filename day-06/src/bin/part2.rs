lib::day!(06, part2, test => 71503, answer => 28973936);

#[allow(unused_variables)]
fn part2(input: &str) -> usize {
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
