lib::day!(11, part1, example => 374, answer => 10885634);

type Image = [Vec<char>];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

fn part1(input: &str) -> isize {
    let image = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let expanded_image = expand_universe(&image);

    let galaxies = find_galaxies(&expanded_image);

    let mut total_distance = 0;
    for galaxy_id in 0..galaxies.len() {
        for next_galaxy_id in (galaxy_id + 1)..galaxies.len() {
            total_distance += get_distance(&galaxies[galaxy_id], &galaxies[next_galaxy_id])
        }
    }

    total_distance
}

fn expand_universe(image: &[Vec<char>]) -> Vec<Vec<char>> {
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

    for row in expanded.iter_mut() {
        for expand_column in expand_columns.iter().rev() {
            row.insert(*expand_column, '.');
        }
    }

    for expand_row in expand_rows.rev() {
        expanded.insert(expand_row, ['.'].repeat(columns));
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
