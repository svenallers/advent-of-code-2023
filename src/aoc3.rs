use std::ops::Range;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::fs_util::read_or_panic;

struct Number {
    number: usize,
    position: Range<usize>,
}

struct Symbol {
    symbol: char,
    position: usize,
}

struct Line {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

struct Schematic {
    lines: Vec<Line>
}

struct Gear {
    ratio: usize
}

impl Schematic {
    fn find_parts(&self) -> Vec<&Number>{
        let empty_symbols_vec = Vec::new();
        let mut parts = Vec::new();
        let mut i = 0;
        while i < self.lines.len() {
            let line = &self.lines[i];
            let symbols_in_line_before = if i > 0 {
                self.lines.get(i - 1).map(|l| &l.symbols).unwrap_or(&empty_symbols_vec)
            } else {
                &empty_symbols_vec
            };
            let symbols_in_line_after = self.lines.get(i + 1).map(|l| &l.symbols).unwrap_or(&empty_symbols_vec);
            let symbols_in_adjacent_lines = [
                line.symbols.iter().clone().collect::<Vec<&Symbol>>(),
                symbols_in_line_before.iter().clone().collect::<Vec<&Symbol>>(),
                symbols_in_line_after.iter().clone().collect::<Vec<&Symbol>>()
            ].concat();
            for number in &line.numbers {
                if self.is_symbol_neighbouring_range(&symbols_in_adjacent_lines, &number.position) {
                    parts.push(number)
                }
            }
            i += 1;
        }
        return parts;
    }

    fn find_gears(&self) -> Vec<Gear> {
        let empty_numbers_vec = Vec::new();
        let mut gears = Vec::new();
        let mut i = 0;
        while i < self.lines.len() {
            let line = &self.lines[i];
            let numbers_in_line_before = if i > 0 {
                self.lines.get(i - 1).map(|l| &l.numbers).unwrap_or(&empty_numbers_vec)
            } else {
                &empty_numbers_vec
            };
            let numbers_in_line_after = self.lines.get(i + 1).map(|l| &l.numbers).unwrap_or(&empty_numbers_vec);
            let numbers_in_adjacent_lines = [
                line.numbers.iter().clone().collect::<Vec<&Number>>(),
                numbers_in_line_before.iter().clone().collect::<Vec<&Number>>(),
                numbers_in_line_after.iter().clone().collect::<Vec<&Number>>()
            ].concat();
            line.symbols.iter().filter(|s| s.symbol == '*').for_each(|star| {
                let gear_candidate: Vec<&&Number> = numbers_in_adjacent_lines.iter().filter(|number| self.is_symbol_neighbouring_range(&vec![star], &number.position)).collect();
                if gear_candidate.len() == 2 {
                    gears.push(Gear{
                        ratio: gear_candidate[0].number * gear_candidate[1].number,
                    });
                }
            });
            i += 1;
        }
        return gears;
    }

    fn is_symbol_neighbouring_range(&self, symbols: &Vec<&Symbol>, range: &Range<usize>) -> bool {
        let range_plus_neighbours = if range.start > 0 {
            (range.start - 1)..(range.end + 1)
        } else {
            range.start..(range.end + 1)
        };
        symbols.iter().any(|s| range_plus_neighbours.contains(&s.position))
    }
}

pub fn aoc_3_1() -> usize {
    return parse_and_combine_part_numbers("res/aoc3.txt");
}

pub fn aoc_3_2() -> usize {
    return parse_and_combine_gear_ratios("res/aoc3.txt");
}

fn parse_and_combine_part_numbers(file: &str) -> usize {
    let schematic = parse_input_data(&read_or_panic(file));
    return schematic.find_parts().iter().map(|part| part.number).sum();
}

fn parse_and_combine_gear_ratios(file: &str) -> usize {
    let schematic = parse_input_data(&read_or_panic(file));
    return schematic.find_gears().iter().map(|gear| gear.ratio).sum();
}

static PARTS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<number>\d+)|(?<symbol>[^1-9.\n])").unwrap());

fn parse_input_data(input_data: &str) -> Schematic {
    let lines = input_data.lines().map(|line| {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();
        for capture in PARTS_REGEX.captures_iter(line) {
            if let Some(number_match) = capture.name("number") {
                numbers.push(Number{
                    number: number_match.as_str().parse().unwrap(),
                    position: number_match.start()..number_match.end()
                })
            }
            if let Some(symbol_match) = capture.name("symbol") {
                symbols.push(Symbol {
                    symbol: symbol_match.as_str().chars().nth(0).unwrap(),
                    position: symbol_match.start(),
                })
            }
        }
        Line {
            numbers,
            symbols
        }
    }).collect();
    return Schematic {
        lines
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc3::{parse_and_combine_gear_ratios, parse_and_combine_part_numbers};

    #[test]
    fn aoc_3_1() {
        assert_eq!(parse_and_combine_part_numbers("res/aoc3-example.txt"), 4361)
    }

    #[test]
    fn aoc_3_2() {
        assert_eq!(parse_and_combine_gear_ratios("res/aoc3-example.txt"), 467835)
    }
}
