use std::cmp::Ordering;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;
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

trait Hand {
    type CardType: Ord;

    fn get_hand(&self) -> &[Self::CardType;5];

    fn get_bid(&self) -> &usize;

    fn get_hand_type(&self) -> HandType;
}

impl<T: Hand> PartialOrd<T> for T {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.get_hand_type().cmp(&other.get_hand_type()) {
            Ordering::Equal =>
                Some(self.get_hand().cmp(&other.get_hand())),
            ordering => Some(ordering),
        }
    }
}

#[derive(Ord, Eq, PartialEq)]
struct ClassicHand {
    hand: [ClassicCard; 5],
    bid: usize
}

impl Hand for ClassicHand {

    type CardType = ClassicCard;
    fn get_hand(&self) -> &[Self::CardType; 5] {
        &self.hand
    }

    fn get_bid(&self) -> &usize {
        &self.bid
    }
    fn get_hand_type(&self) -> HandType {
        let mut cards = HashMap::new();
        for card in &self.hand {
            let count: &usize = cards.get(&card).unwrap_or(&0);
            cards.insert(card, count+1);
        }
        let mut sorted_cards: Vec<&usize> = cards.values().collect();
        sorted_cards.sort_by(|count_1, count_2| count_2.cmp(count_1));
        return get_hand_type_by_counts(sorted_cards[0], sorted_cards.get(1).unwrap_or(&&0));
    }
}

#[derive(Ord, Eq, PartialEq, Debug)]
struct JokerHand {
    hand: [JokerCard; 5],
    bid: usize
}

impl Hand for JokerHand {

    type CardType = JokerCard;
    fn get_hand(&self) -> &[Self::CardType; 5] {
        &self.hand
    }

    fn get_bid(&self) -> &usize {
        &self.bid
    }
    fn get_hand_type(&self) -> HandType {
        let mut cards = HashMap::new();
        for card in &self.hand {
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

pub fn aoc_7_1() -> usize {
    return parse_and_calculate_total_winnings("res/aoc7.txt");
}

pub fn aoc_7_2() -> usize {
    return parse_and_calculate_total_winnings_with_joker("res/aoc7.txt");
}

fn parse_and_calculate_total_winnings(input_file: &str) -> usize {
    let mut hands = parse_input_data(&read_or_panic(input_file));
    hands.sort();
    return hands.iter().enumerate().map(|(index, hand)| (index + 1) * hand.bid).sum();
}

fn parse_and_calculate_total_winnings_with_joker(input_file: &str) -> usize {
    let mut hands = parse_input_data(&read_or_panic(input_file));
    hands.sort();
    return hands.iter().enumerate().map(|(index, hand)| (index + 1) * hand.bid).sum();
}

const HAND_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<hand>\w{5})\s+(?<bid>\d+)").unwrap());

const HAND_MAPPING: Lazy<HashMap<char, ClassicCard>> = Lazy::new(|| HashMap::from([
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

fn parse_input_data(input_data: &str) -> Vec<ClassicHand> {
    return HAND_REGEX.captures_iter(input_data).map(|matched_hand| {
        let hand: Vec<ClassicCard> = matched_hand.name("hand").unwrap().as_str().chars().map(|char|
            HAND_MAPPING[&char]
        ).collect();
        ClassicHand {
            hand: hand.try_into().unwrap(),
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
