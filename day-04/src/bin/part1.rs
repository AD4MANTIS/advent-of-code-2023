use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[allow(unused_variables)]
fn part1(input: &str) -> usize {
    input.lines().flat_map(Card::parse).map(Card::points).sum()
}

struct Card {
    winning_numbers: HashSet<u64>,
    having_numbers: Vec<u64>,
}

impl Card {
    pub fn parse(input: &str) -> Option<Card> {
        let parts: Vec<_> = input
            .split([':', '|'])
            .map(|part| part.split(' ').collect::<Vec<_>>())
            .collect();

        Some(Card {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(include_str!("./test-input.txt"));
        assert_eq!(result, 13);
    }
}
