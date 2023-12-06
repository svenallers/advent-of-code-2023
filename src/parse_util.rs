use regex::Captures;

pub fn parse_numbers(capture: &Captures, group_name: &str) -> Vec<usize>{
    capture.name(group_name).unwrap().as_str().split(" ").filter(|num| !num.is_empty()).map(|num| num.parse().unwrap()).collect()
}