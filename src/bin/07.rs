advent_of_code::solution!(7);

use std::collections::LinkedList;
use std::cmp::Ordering;

struct Card {
    strength: u32,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.strength.cmp(&other.strength))
    }
}

impl Card {
    fn new(c: char) -> Card {
        if c.is_digit(10) {
            Card {strength: c.to_digit(10).unwrap()}
        } else {
            let s = match c {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("This can't happen")
            };
            Card { strength: s }
        }
    }
}

#[derive(PartialEq, PartialOrd)]
enum Value {
    High,
    Pair,
    TwoPairs,
    Drill,
    FullHouse,
    Poker,
    Quint
}

struct Hand {
    v: Vec<Card>
}

impl Hand {
    fn new(row: &str) -> Hand {
        let mut v: Vec<Card> = Vec::new();

        Hand {v: row.chars().map(|x: char| {Card::new(x)}).collect() }
    }

}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

    }
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
