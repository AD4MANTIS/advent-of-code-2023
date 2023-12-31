lib::day!(06, part1, test => 288);

fn part1(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| {
            line.split(' ')
                .flat_map(str::parse::<u32>)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let games = lines[0].iter().zip(lines[1].iter()).collect::<Vec<_>>();

    let mut number_of_ways_you_can_beat_the_record_per_game = Vec::with_capacity(games.len());

    for game in games {
        let time = *game.0;
        let best_distance = *game.1;

        number_of_ways_you_can_beat_the_record_per_game.push(0);

        for hold_time in 1..time {
            let speed = hold_time;

            if (time - hold_time) * speed > best_distance {
                *number_of_ways_you_can_beat_the_record_per_game
                    .last_mut()
                    .unwrap() += 1;
            }
        }
    }

    number_of_ways_you_can_beat_the_record_per_game
        .iter()
        .product()
}
