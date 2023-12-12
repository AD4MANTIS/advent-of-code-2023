use rayon::iter::{ParallelBridge, ParallelIterator};

lib::day!(12, part1,
    row_1 raw("???.### 1,1,3") => 1,
    row_2 raw(".??..??...?##. 1,1,3") => 4,
    row_3 raw("?#?#?#?#?#?#?#? 1,3,1,6") => 1,
    row_4 raw("????.#...#... 4,1,1") => 1,
    row_5 raw("????.######..#####. 1,6,5") => 4,
    row_6 raw("?###???????? 3,2,1") => 10,

    row_64 raw("?##??.?#?? 2,1") => 1,

    example => 21,
    answer => 7490
);

fn part1(input: &'static str) -> usize {
    input
        .lines()
        .par_bridge()
        .map(SpringRowRecord::from)
        .flat_map(|row| with_replaced_unknown_springs(&row))
        .filter(is_row_possible)
        .count()
}

#[derive(Debug, Clone)]
struct SpringRowRecord {
    springs: String,
    groups_of_damaged_springs: Vec<u32>,
}

impl From<&'static str> for SpringRowRecord {
    fn from(value: &'static str) -> Self {
        let [springs, groups_of_damaged_springs]: [&str; 2] = value
            .split(' ')
            .collect::<Vec<_>>()
            .try_into()
            .expect("Each row should have 2 parts");

        let groups_of_damaged_springs = groups_of_damaged_springs
            .split(',')
            .flat_map(|group_length| group_length.parse::<u32>());

        Self {
            springs: springs.to_owned(),
            groups_of_damaged_springs: groups_of_damaged_springs.collect(),
        }
    }
}

fn with_replaced_unknown_springs(spring: &SpringRowRecord) -> Vec<SpringRowRecord> {
    let damaged_springs = spring.groups_of_damaged_springs.iter().sum::<u32>() as usize;

    let remaining_damaged_springs = damaged_springs
        - spring
            .springs
            .chars()
            .filter(|spring| *spring == '#')
            .count();

    let unknown_spring_ids = spring
        .springs
        .chars()
        .enumerate()
        .filter(|(_, spring)| *spring == '?')
        .map(|(spring_index, _)| spring_index)
        .collect::<Vec<_>>();

    let all_possible_damaged_spring_positions = get_all_possible_damaged_spring_positions(
        remaining_damaged_springs,
        unknown_spring_ids.len(),
    );

    all_possible_damaged_spring_positions
        .iter()
        .map(|possible_damaged_spring_position| {
            let mut replaced_springs = spring.springs.to_owned();

            for x in possible_damaged_spring_position {
                let unknown_spring_index = unknown_spring_ids[*x];
                replaced_springs.replace_range(unknown_spring_index..=unknown_spring_index, "#");
            }

            replaced_springs = replaced_springs.replace('?', ".");

            SpringRowRecord {
                springs: replaced_springs,
                groups_of_damaged_springs: spring.groups_of_damaged_springs.clone(),
            }
        })
        .chain([SpringRowRecord {
            springs: spring.springs.replace('?', "."),
            groups_of_damaged_springs: spring.groups_of_damaged_springs.clone(),
        }])
        .collect()
}

fn get_all_possible_damaged_spring_positions(
    remaining_damaged_springs: usize,
    unknown_springs_count: usize,
) -> Vec<Vec<usize>> {
    assert!(remaining_damaged_springs <= unknown_springs_count);

    get_possible_damaged_spring_positions_recursive(
        0,
        remaining_damaged_springs,
        unknown_springs_count,
    )
}

fn get_possible_damaged_spring_positions_recursive(
    start_index: usize,
    remaining_damaged_springs: usize,
    unknown_springs_count: usize,
) -> Vec<Vec<usize>> {
    if start_index == unknown_springs_count {
        return vec![vec![]];
    }

    let possible_spring_positions = (start_index..unknown_springs_count)
        .map(|index| vec![index])
        .collect::<Vec<_>>();

    if remaining_damaged_springs <= 1 {
        return possible_spring_positions;
    }

    let next_positions = get_possible_damaged_spring_positions_recursive(
        start_index + 1,
        remaining_damaged_springs - 1,
        unknown_springs_count,
    );

    let mut result: Vec<Vec<usize>> = vec![];

    for possible_spring_position in possible_spring_positions.iter().enumerate() {
        'next_pos_loop: for next_position in next_positions.iter().skip(possible_spring_position.0)
        {
            let value = [possible_spring_position.1.clone(), next_position.clone()].concat();

            for pair in value.windows(2) {
                if pair[0] >= pair[1] {
                    continue 'next_pos_loop;
                }
            }

            result.push(value);
        }
    }

    result
}

fn is_row_possible(record: &SpringRowRecord) -> bool {
    let mut current_group = 0;

    let Some(mut current_groups_of_damaged_springs) =
        record.groups_of_damaged_springs.get(current_group).copied()
    else {
        return record.springs.chars().all(|spring| spring != '#');
    };

    let mut peekable_springs = record.springs.chars().peekable();
    while let Some(spring) = peekable_springs.next() {
        if spring != '#' {
            continue;
        }

        current_groups_of_damaged_springs -= 1;

        if current_groups_of_damaged_springs > 0 && peekable_springs.peek() != Some(&'#') {
            // there is still a broken spring left in this group but the next spring isn't broken
            return false;
        }

        if current_groups_of_damaged_springs == 0 {
            if peekable_springs.next() == Some('#') {
                // there aren't any broken springs in this group left but the next spring is broken
                return false;
            }

            current_group += 1;

            current_groups_of_damaged_springs =
                match record.groups_of_damaged_springs.get(current_group) {
                    Some(x) => *x,
                    None => return peekable_springs.all(|remaining| remaining != '#'),
                };
        }
    }

    false
}

lib::tests! {
    is_row_possible

    init:
    use super::*;

    test:
    fn is_row_not_possible_1() {
        assert!(!is_row_possible(&"....### 1,1,3".into()));
    }

    fn is_row_not_possible_2() {
        assert!(!is_row_possible(&"###.### 1,1,3".into()));
    }

    fn is_row_not_possible_3() {
        assert!(!is_row_possible(&"##..### 1,1,3".into()));
    }

    fn is_row_not_possible_4() {
        assert!(is_row_possible(&"#.#.### 1,1,3".into()));
    }
}

lib::tests! {
    get_possible_spring_positions

    init:
    use super::*;

    test:
    fn get_possible_spring_positions_1_4() {
        let expected = vec![
            vec![0],
            vec![1],
            vec![2],
            vec![3],
        ];

        assert_eq!(get_all_possible_damaged_spring_positions(1, 4), expected);
    }

    fn get_possible_spring_positions_2_4() {
        let expected = vec![
            vec![0,1],
            vec![0,2],
            vec![0,3],
            vec![1,2],
            vec![1,3],
            vec![2,3],
        ];

        assert_eq!(get_all_possible_damaged_spring_positions(2, 4), expected);
    }

    fn get_possible_spring_positions_3_3() {
        let expected = vec![
            vec![0,1,2],
        ];

        assert_eq!(get_all_possible_damaged_spring_positions(3, 3), expected);
    }

    fn get_possible_spring_positions_3_4() {
        let expected = vec![
            vec![0,1,2],
            vec![0,1,3],
            vec![0,2,3],
            vec![1,2,3],
        ];

        assert_eq!(get_all_possible_damaged_spring_positions(3, 4), expected);
    }

    fn get_possible_spring_positions_4_7() {
        let expected = vec![
            vec![0,1,2,3],
            vec![0,1,2,4],
            vec![0,1,2,5],
            vec![0,1,2,6],

            vec![0,1,3,4],
            vec![0,1,3,5],
            vec![0,1,3,6],

            vec![0,1,4,5],
            vec![0,1,4,6],

            vec![0,1,5,6],

            vec![0,2,3,4],
            vec![0,2,3,5],
            vec![0,2,3,6],
            vec![0,2,4,5],
            vec![0,2,4,6],
            vec![0,2,5,6],

            vec![0,3,4,5],
            vec![0,3,4,6],
            vec![0,3,5,6],

            vec![0,4,5,6],
            //
            vec![1,2,3,4],
            vec![1,2,3,5],
            vec![1,2,3,6],
            vec![1,2,4,5],
            vec![1,2,4,6],
            vec![1,2,5,6],
            vec![1,3,4,5],
            vec![1,3,4,6],
            vec![1,3,5,6],
            vec![1,4,5,6],
            //
            vec![2,3,4,5],
            vec![2,3,4,6],
            vec![2,3,5,6],
            vec![2,4,5,6],
            //
            vec![3,4,5,6],
        ];

        assert_eq!(get_all_possible_damaged_spring_positions(4, 7), expected);
    }
}
