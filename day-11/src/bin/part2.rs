lib::day!(11, part2_1000000, answer => 707505470642);
// fn main() {}

lib::day_test!(11, part2_2, example => 374);
lib::day_test!(11, part2_10, example => 1030);
lib::day_test!(11, part2_100, example => 8410);

// lib::tests! {expansion_tests
//     init:
//     use super::*;

//     test:
//     fn expand_by_10() {
//         let input = include_str!("./example-input.txt");

//         assert_eq!(part2<10>(input), 1030);
//     }

//     fn expand_by_100() {
//         let input = include_str!("./example-input.txt");

//         assert_eq!(part2<100>(input), 8410);
//     }
// }

type Image = [Vec<char>];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

fn part2_2(input: &str) -> isize {
    part2::<2>(input)
}

fn part2_10(input: &str) -> isize {
    part2::<10>(input)
}

fn part2_100(input: &str) -> isize {
    part2::<100>(input)
}

fn part2_1000000(input: &str) -> isize {
    part2::<1000000>(input)
}

fn part2<const N: usize>(input: &str) -> isize {
    let image = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let expanded_image = expand_universe::<N>(&image);

    let galaxies = find_galaxies(&expanded_image);

    let mut total_distance = 0;
    for galaxy_id in 0..galaxies.len() {
        for next_galaxy_id in (galaxy_id + 1)..galaxies.len() {
            total_distance += get_distance(&galaxies[galaxy_id], &galaxies[next_galaxy_id])
        }
    }

    total_distance
}

fn expand_universe<const FACTOR: usize>(image: &[Vec<char>]) -> Vec<Vec<char>> {
    let expand_rows = image.iter().enumerate().filter_map(|(id, row)| {
        if row.iter().all(|space| *space == '.') {
            Some(id)
        } else {
            None
        }
    });

    let columns = image[0].len();
    let expand_columns = (0..columns)
        .filter(|column_id| image.iter().all(|row| row[*column_id] == '.'))
        .collect::<Vec<_>>();

    let mut expanded = image.to_vec();

    for row in expanded.iter_mut().enumerate() {
        println!("{}", row.0);

        for expand_column in expand_columns.iter().rev() {
            // for _ in 1..FACTOR {
            row.1
                .splice(expand_column..=expand_column, ['.'].repeat(FACTOR));
            // }
        }
    }

    let a = ['.'].repeat(columns);
    let b = (0..FACTOR).map(|_| a.clone());

    // let columns = expanded[0].len();
    for expand_row in expand_rows.rev() {
        // for _ in 1..FACTOR {
        expanded.splice(expand_row..=expand_row, b.clone());
        // }
    }

    expanded
}

fn find_galaxies(map: &Image) -> Vec<Pos> {
    map.iter()
        .enumerate()
        .flat_map(|row| {
            row.1
                .iter()
                .enumerate()
                .filter_map(move |space| match *space.1 {
                    '.' => None,
                    '#' => Some(Pos {
                        x: space.0,
                        y: row.0,
                    }),
                    _ => None,
                })
        })
        .collect()
}

const fn get_distance(pos: &Pos, pos2: &Pos) -> isize {
    (pos.x.abs_diff(pos2.x) + pos.y.abs_diff(pos2.y)) as isize
}
