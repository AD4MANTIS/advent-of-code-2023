use itertools::Itertools;

lib::day!(
    09,
    part2,
    example => 2,
    example_3rd raw("10  13  16  21  30  45") => 5,
    answer => 1050
);

#[allow(unused_variables)]
fn part2(input: &str) -> isize {
    let mut data_histories = input
        .lines()
        .map(|line| line.split(' ').flat_map(str::parse::<isize>))
        .map(|history| {
            let mut histories: Vec<Vec<_>> = vec![history.collect()];

            while histories
                .last()
                .unwrap()
                .iter()
                .any(|difference| *difference != 0)
            {
                histories.push(get_history_difference(histories.last().unwrap()));
            }

            histories
        })
        .collect::<Vec<_>>();

    for histories in data_histories.iter_mut() {
        for history in (1..histories.len()).rev() {
            let new_value = histories[history - 1][0] - histories[history][0];

            histories[history - 1].insert(0, new_value);
        }
    }

    data_histories.iter().map(|histories| histories[0][0]).sum()
}

fn get_history_difference(history: &[isize]) -> Vec<isize> {
    history
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|pair| pair.1 - pair.0)
        .collect()
}
