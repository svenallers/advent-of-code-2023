use std::cmp::max;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::fs_util::read_or_panic;

struct Game {
    number: usize,
    combinations: Vec<CubeCombination>,
}

impl Game {
    fn is_valid_for(&self, cubes_in_bag: &CubeCombination) -> bool {
        self.combinations.iter().all(|combination|
            combination.red <= cubes_in_bag.red
                && combination.green <= cubes_in_bag.green
                && combination.blue <= cubes_in_bag.blue
        )
    }
}

#[derive(Copy, Clone)]
struct CubeCombination {
    red: usize,
    green: usize,
    blue: usize,
}

pub fn aoc_2_1() -> usize {
    let combination_in_bag = CubeCombination {
        red: 12,
        green:13,
        blue: 14
    };
    return parse_and_combine_valid_games(&combination_in_bag, "res/aoc2.txt");
}

pub fn aoc_2_2() -> usize {
    return parse_and_combine_power_of_min_combination_possible("res/aoc2.txt");
}

fn parse_and_combine_valid_games(cubes_in_bag: &CubeCombination, input_file: &str) -> usize {
    let input_data = read_or_panic(input_file);
    return combine_valid_games(cubes_in_bag, &parse_input_data(&input_data));
}

fn combine_valid_games(cubes_in_bag: &CubeCombination, games: &Vec<Game>) -> usize {
    games.iter().filter(|game| game.is_valid_for(cubes_in_bag)).map(|game| game.number).sum()
}

fn parse_and_combine_power_of_min_combination_possible(input_file: &str) -> usize {
    let input_data = read_or_panic(input_file);
    return combine_power_of_min_combination_possible(&parse_input_data(&input_data));
}

fn combine_power_of_min_combination_possible(games: &Vec<Game>) -> usize {
    games.iter().map(|game| {
        power_of_min_combination_possible(&game.combinations)
    }).sum()
}

fn power_of_min_combination_possible(combinations: &Vec<CubeCombination>) -> usize {
    let min_combination = combinations.iter().fold(CubeCombination{red: 0, green: 0, blue: 0},
        |current_min, combination| CubeCombination {
            red: max(current_min.red, combination.red),
            green: max(current_min.green, combination.green),
            blue: max(current_min.blue, combination.blue),
        }
    );
    return min_combination.red * min_combination.green * min_combination.blue;
}

static GAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game\s(?<number>\d+):(?<combinations>[^\n]*)(\n|$)").unwrap());
static COMBINATION_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<amount>\d+)\s(?<color>[a-z]+)").unwrap());

fn parse_input_data(input_data: &str) -> Vec<Game> {
    return GAME_REGEX.captures_iter(input_data).map(|matched_game| {
        let combinations = matched_game.name("combinations").unwrap().as_str().split(";").map(|combination_entry| {
            let combination: HashMap<&str, usize> = COMBINATION_REGEX.captures_iter(combination_entry).map(|matched_combination| {
                let amount: usize = matched_combination.name("amount").unwrap().as_str().parse().unwrap();
                let color = matched_combination.name("color").unwrap().as_str();
                (color, amount)
            }).collect();
            CubeCombination {
                red: *combination.get("red").unwrap_or(&0),
                green: *combination.get("green").unwrap_or(&0),
                blue: *combination.get("blue").unwrap_or(&0),
            }
        }).collect();
        Game {
            number: matched_game.name("number").unwrap().as_str().parse().unwrap(),
            combinations
        }
    }).collect();
}

#[cfg(test)]
mod tests {
    use crate::aoc2::{CubeCombination, parse_and_combine_power_of_min_combination_possible, parse_and_combine_valid_games};

    #[test]
    fn aoc_2_1() {
        let cubes_in_bag = CubeCombination {
            red: 12,
            green: 13,
            blue: 14,
        };
        assert_eq!(parse_and_combine_valid_games(&cubes_in_bag, "res/aoc2-example.txt"), 8)
    }

    #[test]
    fn aoc_2_2() {
        assert_eq!(parse_and_combine_power_of_min_combination_possible("res/aoc2-example.txt"), 2286)
    }
}
