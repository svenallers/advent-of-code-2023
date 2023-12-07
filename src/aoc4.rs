use once_cell::sync::Lazy;
use regex::Regex;

use crate::fs_util::read_or_panic;
use crate::parse_util::parse_numbers;

#[derive(Clone)]
struct Card {
    number: usize,
    winning_numbers: Vec<usize>,
    own_numbers: Vec<usize>,
}

impl Card {
    fn points(&self) -> usize {
        let matches = self.matches();
        return if matches > 0 {
            2usize.pow(matches as u32 - 1)
        } else {
            0
        }
    }

    fn matches(&self) -> usize {
        self.winning_numbers.iter().filter(|number| self.own_numbers.contains(number)).collect::<Vec<_>>().len()
    }
}

pub fn aoc_4_1() -> usize {
    return parse_and_sum_up_points("res/aoc4.txt");
}

pub fn aoc_4_2() -> usize {
    return parse_and_count_all_cards_including_won_ones("res/aoc4.txt");
}

fn parse_and_sum_up_points(input_file: &str) -> usize {
    let input_data = read_or_panic(input_file);
    return parse_input_data(&input_data).iter().map(|card| card.points()).sum();
}

fn parse_and_count_all_cards_including_won_ones(input_file: &str) -> usize {
    let input_data = read_or_panic(input_file);
    let original_cards = parse_input_data(&input_data);
    let mut all_cards: Vec<Card> = original_cards.iter().map(|card| card.clone()).collect();
    let mut i = 0;
    while i < all_cards.len() {
        let card = all_cards[i].clone();
        let matches = card.matches();
        for n in 0..matches {
            if let Some(won_card) = original_cards.get(card.number + n) {
                all_cards.insert(i + 1, won_card.clone());
            }
        }
        i += 1;
    }
    return all_cards.len();
}

const CARD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Card\s+(?<number>\d+):\s*(?<winning_numbers>(\d+\s*)*)\|\s*(?<own_numbers>(\d+\s*)*)(\n|$)").unwrap());

fn parse_input_data(input_data: &str) -> Vec<Card> {
    return CARD_REGEX.captures_iter(input_data).map(|matched_card| {
        Card {
            number: matched_card.name("number").unwrap().as_str().parse().unwrap(),
            winning_numbers: parse_numbers(&matched_card, "winning_numbers"),
            own_numbers: parse_numbers(&matched_card, "own_numbers"),
        }
    }).collect();
}

#[cfg(test)]
mod tests {
    use crate::aoc4::{parse_and_count_all_cards_including_won_ones, parse_and_sum_up_points};

    #[test]
    fn aoc_4_1() {
        assert_eq!(parse_and_sum_up_points( "res/aoc4-example.txt"), 14)
    }

    #[test]
    fn aoc_4_2() {
        assert_eq!(parse_and_count_all_cards_including_won_ones( "res/aoc4-example.txt"), 31)
    }
}
