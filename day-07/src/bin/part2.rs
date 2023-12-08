use std::{cmp::Ordering, collections::HashMap, ops::AddAssign};

use strum::EnumIter;

/*
    Thanks to this Solution for some mental help: https://github.com/Kezzryn/Advent-of-Code/blob/84c19cf693eff24f312dbb0c84c9df0d017dbede/2023/Day%2007/Program.cs

    I had the following problems:
    1. `get_score` would return random results because iterating over `cards_count.values()` was in random order and my old algorithm couldn't handle that
    2. While searching for a `FullHouse` and finding 3 of a kind I first used a normal `break` which only ended the inner loop and continued to use the same cards for the pair.
      `continue 'outer;` was exactly what I needed here.
*/

fn main() {
    let _timer = lib::PrintTimer::new("");

    let input = include_str!("./input.txt");
    let output = part2(input);

    dbg!(output);
    assert_eq!(output, 253718286);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter, Clone, Copy)]
enum Card {
    J,
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Q,
    K,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::Ace,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    hand: [Card; 5],
    bid: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GameScores {
    score: Score,
    game: Game,
}

impl Ord for GameScores {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score.cmp(&other.score) {
            Ordering::Equal => {}
            ord => return ord,
        }

        for cards in self.game.hand.iter().zip(other.game.hand.iter()) {
            match cards.0.cmp(cards.1) {
                Ordering::Equal => {}
                ord => return ord,
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for GameScores {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Game {
    pub fn get_score(&self) -> Score {
        let mut cards_count = self.hand.iter().fold(HashMap::new(), |mut acc, x| {
            acc.entry(*x).or_insert(0).add_assign(1);
            acc
        });

        let joker_count = cards_count.remove(&Card::J).unwrap_or(0);

        let max_same = *cards_count.values().max().unwrap_or(&0) + joker_count;

        match max_same {
            5 => return Score::FiveOfAKind,
            4 => return Score::FourOfAKind,
            _ => {}
        }

        {
            let mut used_jokers = 0;

            let mut has_two = false;
            let mut has_three = false;

            'outer: for count in cards_count.values() {
                if !has_three {
                    for use_jokers in 0..=(joker_count - used_jokers) {
                        if *count + use_jokers >= 3 {
                            has_three = true;
                            used_jokers += use_jokers;
                            continue 'outer;
                        }
                    }
                }

                if !has_two {
                    for use_jokers in 0..=(joker_count - used_jokers) {
                        if *count + use_jokers >= 2 {
                            has_two = true;
                            used_jokers += use_jokers;
                            continue 'outer;
                        }
                    }
                }
            }

            if has_two && has_three {
                return Score::FullHouse;
            }
        }

        if max_same == 3 {
            return Score::ThreeOfAKind;
        }

        let mut used_jokers = 0;

        match cards_count
            .values()
            .copied()
            .filter(|count| {
                for use_jokers in 0..=(joker_count - used_jokers) {
                    if *count + use_jokers >= 2 {
                        used_jokers += use_jokers;
                        return true;
                    }
                }

                false
            })
            .count()
        {
            2 => Score::TwoPair,
            1 => Score::OnePair,
            _ => Score::HighCard,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Score {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn part2(input: &str) -> usize {
    let mut scores = parse_games(input)
        .into_iter()
        .map(|game| GameScores {
            score: Game::get_score(&game),
            game,
        })
        .collect::<Vec<_>>();

    scores.sort();

    scores
        .iter()
        .enumerate()
        .fold(0, |acc, x| acc + ((x.0 + 1) * x.1.game.bid as usize))
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|parts| Game {
            hand: parts[0]
                .chars()
                .map(|char| Card::try_from(char).expect("Hand should only contain valid cards"))
                .collect::<Vec<_>>()
                .try_into()
                .expect("Hand should have exactly 5 cards"),
            bid: parts[1]
                .parse::<u32>()
                .expect("bid should be a positive number"),
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp() {
        assert!(Card::J < Card::Two);
        assert!(Card::Two > Card::J);
        assert!(Card::Ten > Card::J);
        assert!(Card::Ace > Card::J);
    }

    #[test]
    fn it_works() {
        let result = part2(include_str!("./test-input.txt"));
        assert_eq!(result, 5905);
    }
}
