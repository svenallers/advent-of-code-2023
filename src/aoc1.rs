use crate::fs_util::read_or_panic;

pub fn aoc_1_1() -> usize {
    return calculate_calibration_value(&read_or_panic("res/aoc1.txt"));
}

pub fn aoc_1_2() -> usize {
    return calculate_calibration_value_from_data_with_spelled_numbers(&read_or_panic("res/aoc1.txt"));
}

fn calculate_calibration_value(calibration_data: &str) -> usize {
    let mut calibration_value = 0;
    for line in  calibration_data.lines() {
        let digits: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
        if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
            calibration_value += format!("{first}{last}").parse::<usize>().unwrap();
        }
    }
    return calibration_value;
}

const DIGIT_DICTIONARY: [(&str, usize); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn calculate_calibration_value_from_data_with_spelled_numbers(calibration_data: &str) -> usize {
    let mut calibration_value = 0;
    for line in  calibration_data.lines() {
        calibration_value += evaluate_line_with_spelled_numbers(line);
    }
    return calibration_value;
}

fn evaluate_line_with_spelled_numbers(line: &str) -> usize {
    let mut digits = Vec::<usize>::new();
    for pos in 0..line.len() {
        let char_at_pos = line.chars().nth(pos).unwrap();
        if char_at_pos.is_numeric() {
            digits.push(char_at_pos.to_string().parse().unwrap());
        } else {
            for (digit_word, digit) in DIGIT_DICTIONARY {
                if line[pos..].starts_with(digit_word) {
                    digits.push(digit);
                    break;
                }
            }
        }
    }

    if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
        return format!("{first}{last}").parse().unwrap();
    } else {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc1::{calculate_calibration_value, calculate_calibration_value_from_data_with_spelled_numbers};
    use crate::fs_util::read_or_panic;

    #[test]
    fn aoc_1_1() {
        let example_data = read_or_panic("res/aoc1-1-example.txt");
        assert_eq!(calculate_calibration_value(&example_data), 142);
    }

    #[test]
    fn aoc_1_2() {
        let example_data = read_or_panic("res/aoc1-1-example.txt");
        assert_eq!(calculate_calibration_value_from_data_with_spelled_numbers(&example_data), 363);
    }
}
