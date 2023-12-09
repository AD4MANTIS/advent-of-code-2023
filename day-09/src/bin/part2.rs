use itertools::Itertools;

lib::day!(
    "09",
    part2,
    test => 2,
    mixed_numbers("./test-mixed-numbers.txt") => 21164658,
    neg_numbers("./test-negative-numbers.txt") => -106
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
            let new_value =
                histories[history - 1].last().unwrap() + histories[history].last().unwrap();

            histories[history - 1].push(new_value);
        }
    }

    data_histories
        .iter()
        .map(|histories| histories[0].last().unwrap())
        .cloned()
        .sum()
}

fn get_history_difference(history: &[isize]) -> Vec<isize> {
    history
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|pair| pair.1 - pair.0)
        .collect()
}

// fn get_history_difference_old(history: impl Iterator<Item = isize>) -> Vec<isize> {
//     let mut differences = Vec::new();
//     let mut history = history.peekable();

//     while let Some(hist) = history.next() {
//         if let Some(next) = history.peek() {
//             differences.push(next - hist);
//         }
//     }

//     differences
// }
