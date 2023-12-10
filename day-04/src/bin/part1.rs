use std::collections::HashSet;

lib::day!(04, part1, test => 13);

#[allow(unused_variables)]
fn part1(input: &str) -> usize {
    input.lines().flat_map(Card::parse).map(Card::points).sum()
}

struct Card {
    winning_numbers: HashSet<u64>,
    having_numbers: Vec<u64>,
}

impl Card {
    pub fn parse(input: &str) -> Option<Self> {
        let parts: Vec<_> = input
            .split([':', '|'])
            .map(|part| part.split(' ').collect::<Vec<_>>())
            .collect();

        Some(Self {
            winning_numbers: HashSet::from_iter(
                parts
                    .get(1)?
                    .iter()
                    .filter(|winning_number| !winning_number.is_empty())
                    .map(|winning_number| {
                        winning_number.trim().parse().expect("Should be a number")
                    }),
            ),
            having_numbers: parts
                .get(2)?
                .iter()
                .filter(|winning_number| !winning_number.is_empty())
                .map(|winning_number| winning_number.parse().expect("Should be a number"))
                .collect(),
        })
    }

    pub fn points(self) -> usize {
        let matching_numbers = self
            .having_numbers
            .into_iter()
            .filter(|current_number| self.winning_numbers.contains(current_number))
            .count();

        match matching_numbers {
            0 => 0,
            _ => 2_usize.pow((matching_numbers - 1) as u32),
        }
    }
}
