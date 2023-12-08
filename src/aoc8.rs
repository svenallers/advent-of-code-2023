use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::aoc8::Direction::{Left, Right};
use crate::fs_util::read_or_panic;

enum Direction {
    Left,
    Right,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Vertex {
    name: String,
    left: String,
    right: String,
}

struct Map {
    directions: Vec<Direction>,
    vertices: HashMap<String, Vertex>
}

impl Map {
    fn follow_directions(&self, from: &str, to: &str) -> Vec<Vertex> {
        let Some(mut current_vertex) = self.vertices.get(from) else {
            panic!("Cannot find {from}");
        };
        let mut path = Vec::new();
        path.push(current_vertex.clone());
        for direction in &self.directions {
            if current_vertex.name == to {
                return path;
            }
            let next = match direction {
                Left => &current_vertex.left,
                Right => &current_vertex.right,
            };
            current_vertex = self.vertices.get(next).expect(&format!("Cannot find {next}"));
            path.push(current_vertex.clone());
        }
        if current_vertex.name != to {
            let mut next_path = self.follow_directions(&current_vertex.name, to);
            next_path.remove(0);
            path.append(&mut next_path);
        }
        return path;
    }

    fn ghost_directions_length(&self, from: &str, to: &str) -> usize {
        let mut start: HashSet<Vertex> = self.vertices.iter()
            .filter(|(name, _)| name.ends_with(from))
            .map(|(_, vertex)| vertex.clone())
            .collect();
        let mut length = 0;
        loop {
            let last_path = self.follow_ghost_directions_from_vertices(start.clone(), to);
            length += last_path.len();
            if last_path.last().iter().all(|p| p.iter().all(|v| v.name.ends_with(to))) {
                return length;
            }
            start = last_path.last().unwrap().clone();
        }
    }

    fn follow_ghost_directions_from_vertices(&self, from: HashSet<Vertex>, to: &str) -> Vec<HashSet<Vertex>> {
        let mut path = Vec::new();
        let mut current_vertices = from;
        for direction in &self.directions {
            if current_vertices.iter().all(|v| v.name.ends_with(to)) {
                return path;
            }
            let next: Vec<String> = match direction {
                Left => current_vertices.iter().map(|v| v.left.clone()).collect(),
                Right => current_vertices.iter().map(|v| v.right.clone()).collect(),
            };
            current_vertices = next.iter().map(|v |self.vertices.get(v).expect(&format!("Cannot find {v}")).clone()).collect();
            path.push(current_vertices.clone());
        }
        return path;
    }
}

pub fn aoc_8_1() -> usize {
    return parse_and_calculate_path_length("res/aoc8.txt");
}

pub fn aoc_8_2() -> usize {
    return parse_and_calculate_ghost_path_length("res/aoc8.txt");
}

fn parse_and_calculate_path_length(input_file: &str) -> usize {
    let map = parse_input_data(&read_or_panic(input_file));
    return map.follow_directions("AAA", "ZZZ").len() - 1;
}

fn parse_and_calculate_ghost_path_length(input_file: &str) -> usize {
    let map = parse_input_data(&read_or_panic(input_file));
    return map.ghost_directions_length("A", "Z");
}

const MAP_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?<directions>[LR]+)\n\n(?<vertices>(.*(\n|$))*)").unwrap());

const VERTEX_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<name>\w+)\s=\s\((?<left>\w+),\s(?<right>\w+)\)").unwrap());

fn parse_input_data(input_data: &str) -> Map {
    return MAP_REGEX.captures(input_data).map(|matched_map| {
        let vertices_as_str: HashMap<&str, (&str, &str)> = VERTEX_REGEX.captures_iter(matched_map.name("vertices").unwrap().as_str()).map(|matched_vertex|
            (matched_vertex.name("name").unwrap().as_str(), (matched_vertex.name("left").unwrap().as_str(), matched_vertex.name("right").unwrap().as_str()))
        ).collect();
        let mut vertices = HashMap::new();
        for (name, (left, right)) in vertices_as_str {
            vertices.insert(name.to_string(), Vertex {
                name: name.to_string(),
                left: left.to_string(),
                right: right.to_string(),
            });
        }
        Map {
            directions: matched_map.name("directions").unwrap().as_str().chars().map(|char|
                match char {
                    'L' => Left,
                    'R' => Right,
                    _  => panic!("Unknown direction {char}")
                }
            ).collect(),
            vertices
        }
    }).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::aoc8::{parse_and_calculate_ghost_path_length, parse_and_calculate_path_length};

    #[test]
    fn aoc_8_1() {
        assert_eq!(parse_and_calculate_path_length( "res/aoc8-1-example.txt"), 6)
    }

    #[test]
    fn aoc_8_2() {
        assert_eq!(parse_and_calculate_ghost_path_length( "res/aoc8-2-example.txt"), 6)
    }

}
