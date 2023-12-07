use std::ops::Range;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::fs_util::read_or_panic;
use crate::parse_util::parse_numbers;

struct Race {
    time: usize,
    record_distance: usize
}

pub fn aoc_6_1() -> usize {
    return parse_and_multiply_winning_options("res/aoc6.txt");
}

pub fn aoc_6_2() -> usize {
    return parse_and_count_winning_options("res/aoc6.txt");
}

fn parse_and_multiply_winning_options(input_file: &str) -> usize {
    let input_data = read_or_panic(input_file);
    return parse_input_data(&input_data).iter()
        .map(|race| find_winning_range(race).len())
        .reduce(|left, right| left * right)
        .unwrap_or(0);
}

fn parse_and_count_winning_options(input_file: &str) -> usize {
    let race = parse_input_data_with_single_race(&read_or_panic(input_file));
    return find_winning_range(&race).len();
}

fn find_winning_range(race: &Race) -> Range<usize> {
    let mut lower_bound = 0;
    let mut min_button_press = race.time + 1;
    let mut button_press_candidate = race.time / 4;
    while button_press_candidate != min_button_press {
        let time_left = race.time - button_press_candidate;
        let distance = button_press_candidate * time_left;
        if distance > race.record_distance {
            min_button_press = button_press_candidate;
            button_press_candidate -= half(&lower_bound, &button_press_candidate);
        } else {
            lower_bound = button_press_candidate;
            button_press_candidate += half(&button_press_candidate, &min_button_press);
        }
    }
    return min_button_press..(race.time - min_button_press + 1);
}

fn half(from: &usize, to: &usize) -> usize {
    ((to - from) as f32 / 2.0).ceil() as usize
}

const RACES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(
r"^Time:(?<times>(\s*\d+)+)
Distance:(?<distances>(\s*\d+)+)").unwrap());

fn parse_input_data(input_data: &str) -> Vec<Race> {
    return RACES_REGEX.captures(input_data).map(|matched_races| {
        let times = parse_numbers(&matched_races, "times");
        let distances = parse_numbers(&matched_races, "distances");
        times.iter().zip(distances).map(|(time, distance)|
            Race {
                time: *time,
                record_distance: distance,
            }
        ).collect()
    }).unwrap();
}

fn parse_input_data_with_single_race(input_data: &str) -> Race {
    return RACES_REGEX.captures(input_data).map(|matched_races|
        Race {
            time: matched_races.name("times").unwrap().as_str().replace(" ", "").parse().unwrap(),
            record_distance: matched_races.name("distances").unwrap().as_str().replace(" ", "").parse().unwrap(),
        }
    ).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::aoc6::{parse_and_count_winning_options, parse_and_multiply_winning_options};

    #[test]
    fn aoc_6_1() {
        assert_eq!(parse_and_multiply_winning_options( "res/aoc6-example.txt"), 288)
    }

    #[test]
    fn aoc_6_2() {
        assert_eq!(parse_and_count_winning_options( "res/aoc6-example.txt"), 71503)
    }
}
