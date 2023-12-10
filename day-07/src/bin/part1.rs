use std::{cmp::Ordering, collections::HashMap, ops::AddAssign};

lib::day!(07, part1, test => 6440);

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash)]
enum Card {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::Ace,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    hand: [Card; 5],
    bid: u32,
}

#[derive(PartialEq, Eq)]
struct GameScores {
    score: Score,
    game: Game,
}

impl Ord for GameScores {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score.partial_cmp(&other.score) {
            Some(Ordering::Equal) => {}
            Some(ord) => return ord,
            None => {}
        }

        for cards in self.game.hand.iter().zip(other.game.hand.iter()) {
            match cards.0.partial_cmp(cards.1) {
                Some(Ordering::Equal) => {}
                Some(ord) => return ord,
                None => {}
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
        let cards_count = self.hand.iter().fold(HashMap::new(), |mut acc, x| {
            match x {
                Card::J => {}
                _ => acc.entry(x).or_insert(0).add_assign(1),
            };
            acc
        });

        let max_same = *cards_count.values().max().unwrap_or(&0);

        match max_same {
            5 => return Score::FiveOfAKind,
            4 => return Score::FourOfAKind,
            _ => {}
        }

        if cards_count.len() == 2
            && cards_count
                .values()
                .cloned()
                .all(|count| count == 2 || count == 3)
        {
            return Score::FullHouse;
        }

        if max_same == 3 {
            return Score::ThreeOfAKind;
        }

        match cards_count
            .values()
            .cloned()
            .filter(|count| *count == 2)
            .count()
        {
            2 => Score::TwoPair,
            1 => Score::OnePair,
            _ => Score::HighCard,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
enum Score {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[allow(unused_variables)]
fn part1(input: &str) -> usize {
    let games = parse_games(input);

    let mut scores = games
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
