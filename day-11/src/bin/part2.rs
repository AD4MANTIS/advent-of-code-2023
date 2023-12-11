lib::day!(11, part2_1000000, answer => 707505470642);

lib::day_test!(11, part2_2, example => 374);
lib::day_test!(11, part2_10, example => 1030);
lib::day_test!(11, part2_100, example => 8410);

type Image = [Vec<char>];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

#[allow(dead_code)]
fn part2_2(input: &str) -> isize {
    part2::<2>(input)
}

#[allow(dead_code)]
fn part2_10(input: &str) -> isize {
    part2::<10>(input)
}

#[allow(dead_code)]
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

    (0..galaxies.len())
        .flat_map(|galaxy_id| {
            ((galaxy_id + 1)..galaxies.len()).map(move |next_galaxy_id| (galaxy_id, next_galaxy_id))
        })
        .map(|galaxy_ids| get_distance(&galaxies[galaxy_ids.0], &galaxies[galaxy_ids.1]))
        .sum()
}

fn expand_universe<const FACTOR: usize>(image: &[Vec<char>]) -> Vec<Vec<char>> {
    let expand_rows = image
        .iter()
        .enumerate()
        .filter_map(|(id, row)| row.iter().all(|space| *space == '.').then_some(id));

    let columns = image[0].len();
    let expand_columns = (0..columns)
        .filter(|column_id| image.iter().all(|row| row[*column_id] == '.'))
        .collect::<Vec<_>>();

    let mut expanded = image.to_vec();

    for row in expanded.iter_mut() {
        for expand_column in expand_columns.iter().rev() {
            row.splice(expand_column..=expand_column, (0..FACTOR).map(|_| ' '));
        }
    }

    // for the distance calculation the row doesn't need to be filled and it can't contain galaxies (#)
    let new_rows = (0..FACTOR).map(|_| vec![]);

    for expand_row in expand_rows.rev() {
        expanded.splice(expand_row..=expand_row, new_rows.clone());
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
