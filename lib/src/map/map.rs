use std::{convert::Infallible, str::FromStr};

use super::prelude::{Offset, Pos};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    pub rows: Vec<Vec<char>>,
}

impl Map {
    pub fn get(&self, pos: &Pos) -> Option<&char> {
        self.rows.get(pos.y)?.get(pos.x)
    }

    pub const fn column(&self, col: usize) -> ColumnIter {
        ColumnIter(self, Pos { x: col, y: 0 })
    }
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rows: s
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect(),
        })
    }
}

pub struct ColumnIter<'a>(&'a Map, Pos);

impl<'a> Iterator for ColumnIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0.get(&self.1)?;

        self.1 = self.1.clone().try_add(Offset::y(1))?;

        Some(*current)
    }
}

#[cfg(test)]
mod map_tests {
    use super::*;

    fn get_test_map() -> Map {
        Map {
            rows: vec![
                vec!['1', '2', '3'],
                vec!['4', '5', '6'],
                vec!['7', '8', '9'],
                vec!['a', 'b', 'c'],
                vec!['d', 'e', 'f'],
            ],
        }
    }

    #[test]
    fn create_map() {
        let result = Map::from_str(
            "\
123
456
789
abc
def
",
        )
        .unwrap();

        let expected = get_test_map();

        assert_eq!(result, expected);
    }

    #[test]
    fn get_map() {
        let map = get_test_map();

        assert_eq!(map.get(&Pos { x: 0, y: 0 }), Some(&'1'));
        assert_eq!(map.get(&Pos { x: 1, y: 0 }), Some(&'2'));
        assert_eq!(map.get(&Pos { x: 0, y: 1 }), Some(&'4'));
        assert_eq!(map.get(&Pos { x: 2, y: 4 }), Some(&'f'));
        assert_eq!(map.get(&Pos { x: 3, y: 0 }), None);
        assert_eq!(map.get(&Pos { x: 2, y: 5 }), None);
    }

    #[test]
    fn column_iterator() {
        let map = &get_test_map();
        let mut col_iter = map.column(0);

        assert_eq!(col_iter.next(), Some('1'));
        assert_eq!(col_iter.next(), Some('4'));
        assert_eq!(col_iter.next(), Some('7'));
        assert_eq!(col_iter.next(), Some('a'));
        assert_eq!(col_iter.next(), Some('d'));
        assert_eq!(col_iter.next(), None);

        col_iter = map.column(99);
        assert_eq!(col_iter.next(), None);
    }
}
