use std::cmp::Ordering;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::aoc7_1::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};
use crate::fs_util::read_or_panic;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Joker,
    Queen,
    King,
    Ass,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Ord, Eq, PartialEq)]
struct Hand {
    hand: [Card; 5],
    bid: usize
}

impl Hand {
    fn get_hand_type(&self) -> HandType {
        let mut cards = HashMap::new();
        for card in &self.hand {
            let count: &usize = cards.get(&card).unwrap_or(&0);
            cards.insert(card, count+1);
        }
        let mut sorted_cards: Vec<&usize> = cards.values().collect();
        sorted_cards.sort_by(|count_1, count_2| count_2.cmp(count_1));
        return match sorted_cards[0] {
            5 => FiveOfAKind,
            4 => FourOfAKind,
            3 => match sorted_cards[1] {
                2 => FullHouse,
                _ => ThreeOfAKind,
            }
            2 => match sorted_cards[1] {
                2 => TwoPair,
                _ => OnePair,
            }
            _ => HighCard
        }
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.get_hand_type().cmp(&other.get_hand_type()) {
            Ordering::Equal =>
                Some(self.hand.cmp(&other.hand)),
            ordering => Some(ordering),
        }
    }
}

pub fn aoc_7_1() -> usize {
    return parse_and_calculate_total_winnings("res/aoc7.txt");
}

fn parse_and_calculate_total_winnings(input_file: &str) -> usize {
    let mut hands = parse_input_data(&read_or_panic(input_file));
    hands.sort();
    return hands.iter().enumerate().map(|(index, hand)| (index + 1) * hand.bid).sum();
}

const HAND_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<hand>\w{5})\s+(?<bid>\d+)").unwrap());

const HAND_MAPPING: Lazy<HashMap<char, Card>> = Lazy::new(|| HashMap::from([
    ('2', Card::Two),
    ('3', Card::Three),
    ('4', Card::Four),
    ('5', Card::Five),
    ('6', Card::Six),
    ('7', Card::Seven),
    ('8', Card::Eight),
    ('9', Card::Nine),
    ('T', Card::Ten),
    ('J', Card::Joker),
    ('K', Card::King),
    ('Q', Card::Queen),
    ('A', Card::Ass),
]));

fn parse_input_data(input_data: &str) -> Vec<Hand> {
    return HAND_REGEX.captures_iter(input_data).map(|matched_hand| {
        let hand: Vec<Card> = matched_hand.name("hand").unwrap().as_str().chars().map(|char|
            HAND_MAPPING[&char]
        ).collect();
        Hand {
            hand: hand.try_into().unwrap(),
            bid: matched_hand.name("bid").unwrap().as_str().parse().unwrap()
        }
    }).collect();
}

#[cfg(test)]
mod tests {
    use crate::aoc7_1::parse_and_calculate_total_winnings;

    #[test]
    fn aoc_7_1() {
        assert_eq!(parse_and_calculate_total_winnings( "res/aoc7-example.txt"), 6440)
    }

}
