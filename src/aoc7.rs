use std::cmp::Ordering;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;
use crate::aoc7::Deck::{ClassicDeck, JokerDeck};
use crate::aoc7::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

use crate::fs_util::read_or_panic;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Debug)]
enum ClassicCard {
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

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Debug)]
enum JokerCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
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

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Deck {
    ClassicDeck([ClassicCard; 5]),
    JokerDeck([JokerCard; 5]),
}

#[derive(Ord, Eq, PartialEq)]
struct Hand {
    hand: Deck,
    bid: usize
}

impl Hand {
    fn get_hand_type(&self) -> HandType {
        match self.hand {
            ClassicDeck(hand) =>
                get_classic_hand_type(&hand),
            JokerDeck(hand) =>
                get_joker_hand_type(&hand)
        }
    }
}

fn get_classic_hand_type(hand: &[ClassicCard; 5]) -> HandType {
    let mut cards = HashMap::new();
    for card in hand {
        let count: &usize = cards.get(&card).unwrap_or(&0);
        cards.insert(card, count+1);
    }
    let mut sorted_cards: Vec<&usize> = cards.values().collect();
    sorted_cards.sort_by(|count_1, count_2| count_2.cmp(count_1));
    return get_hand_type_by_counts(sorted_cards[0], sorted_cards.get(1).unwrap_or(&&0));
}

fn get_joker_hand_type(hand: &[JokerCard; 5]) -> HandType {
    let mut cards = HashMap::new();
    for card in hand {
        let count: &usize = cards.get(&card).unwrap_or(&0);
        cards.insert(card, count+1);
    }
    let mut sorted_cards: Vec<(&&JokerCard, &usize)> = cards.iter().collect();
    sorted_cards.sort_by(|(_, count_1), (_, count_2)| count_2.cmp(count_1));
    return match sorted_cards[0] {
        (_, 5) => get_hand_type_by_counts(&5, &0),
        _ => match (sorted_cards[0], sorted_cards[1]) {
            ((JokerCard::Joker, count_1), (_, count_2))
            | ((_, count_1), (JokerCard::Joker, count_2)) => {
                let count_3 = sorted_cards.get(3).map(|(_, count)| count).unwrap_or(&&0);
                get_hand_type_by_counts(&(count_1 + count_2), count_3)
            },
            ((_, count_1), (_, count_2)) => {
                let count_1_with_joker = count_1 + cards.get(&JokerCard::Joker).unwrap_or(&0);
                get_hand_type_by_counts(&count_1_with_joker, count_2)
            },
        }
    }
}

fn get_hand_type_by_counts(highest_count: &usize, second_highest_count: &usize) -> HandType {
    match highest_count {
        5 => FiveOfAKind,
        4 => FourOfAKind,
        3 => match second_highest_count {
            2 => FullHouse,
            _ => ThreeOfAKind,
        }
        2 => match second_highest_count {
            2 => TwoPair,
            _ => OnePair,
        }
        _ => HighCard,
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

pub fn aoc_7_2() -> usize {
    return parse_and_calculate_total_winnings_with_joker("res/aoc7.txt");
}

fn parse_and_calculate_total_winnings(input_file: &str) -> usize {
    let mut hands = parse_classic_input_data(&read_or_panic(input_file));
    hands.sort();
    return hands.iter().enumerate().map(|(index, hand)| (index + 1) * hand.bid).sum();
}

fn parse_and_calculate_total_winnings_with_joker(input_file: &str) -> usize {
    let mut hands = parse_joker_input_data(&read_or_panic(input_file));
    hands.sort();
    return hands.iter().enumerate().map(|(index, hand)| (index + 1) * hand.bid).sum();
}

const HAND_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<hand>\w{5})\s+(?<bid>\d+)").unwrap());

const CLASSIC_HAND_MAPPING: Lazy<HashMap<char, ClassicCard>> = Lazy::new(|| HashMap::from([
    ('2', ClassicCard::Two),
    ('3', ClassicCard::Three),
    ('4', ClassicCard::Four),
    ('5', ClassicCard::Five),
    ('6', ClassicCard::Six),
    ('7', ClassicCard::Seven),
    ('8', ClassicCard::Eight),
    ('9', ClassicCard::Nine),
    ('T', ClassicCard::Ten),
    ('J', ClassicCard::Joker),
    ('K', ClassicCard::King),
    ('Q', ClassicCard::Queen),
    ('A', ClassicCard::Ass),
]));

const JOKER_HAND_MAPPING: Lazy<HashMap<char, JokerCard>> = Lazy::new(|| HashMap::from([
    ('2', JokerCard::Two),
    ('3', JokerCard::Three),
    ('4', JokerCard::Four),
    ('5', JokerCard::Five),
    ('6', JokerCard::Six),
    ('7', JokerCard::Seven),
    ('8', JokerCard::Eight),
    ('9', JokerCard::Nine),
    ('T', JokerCard::Ten),
    ('J', JokerCard::Joker),
    ('K', JokerCard::King),
    ('Q', JokerCard::Queen),
    ('A', JokerCard::Ass),
]));

fn parse_classic_input_data(input_data: &str) -> Vec<Hand> {
    return HAND_REGEX.captures_iter(input_data).map(|matched_hand| {
        let hand: Vec<ClassicCard> = matched_hand.name("hand").unwrap().as_str().chars().map(|char|
            CLASSIC_HAND_MAPPING[&char]
        ).collect();
        Hand {
            hand: ClassicDeck(hand.try_into().unwrap()),
            bid: matched_hand.name("bid").unwrap().as_str().parse().unwrap()
        }
    }).collect();
}

fn parse_joker_input_data(input_data: &str) -> Vec<Hand> {
    return HAND_REGEX.captures_iter(input_data).map(|matched_hand| {
        let hand: Vec<JokerCard> = matched_hand.name("hand").unwrap().as_str().chars().map(|char|
            JOKER_HAND_MAPPING[&char]
        ).collect();
        Hand {
            hand: JokerDeck(hand.try_into().unwrap()),
            bid: matched_hand.name("bid").unwrap().as_str().parse().unwrap()
        }
    }).collect();
}

#[cfg(test)]
mod tests {
    use crate::aoc7::{parse_and_calculate_total_winnings, parse_and_calculate_total_winnings_with_joker};

    #[test]
    fn aoc_7_1() {
        assert_eq!(parse_and_calculate_total_winnings( "res/aoc7-example.txt"), 6440)
    }

    #[test]
    fn aoc_7_2() {
        assert_eq!(parse_and_calculate_total_winnings_with_joker( "res/aoc7-example.txt"), 5905)
    }

}
