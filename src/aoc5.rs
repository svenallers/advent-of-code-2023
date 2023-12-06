use once_cell::sync::Lazy;
use regex::{Captures, Regex};

use crate::fs_util::read_or_panic;
use crate::parse_util::parse_numbers;

struct Almanac {
    seeds: Vec<usize>,
    map: AlmanacMap,
}

struct AlmanacWithSeedRange {
    seeds: Vec<SeedRange>,
    map: AlmanacMap,
}

struct SeedRange {
    start: usize,
    length: usize,
}

struct AlmanacMap {
    seed_to_soil: Vec<AlmanacMapping>,
    soil_to_fertilizer: Vec<AlmanacMapping>,
    fertilizer_to_water: Vec<AlmanacMapping>,
    water_to_light: Vec<AlmanacMapping>,
    light_to_temperature: Vec<AlmanacMapping>,
    temperature_to_humidity: Vec<AlmanacMapping>,
    humidity_to_location: Vec<AlmanacMapping>,
}

struct AlmanacMapping {
    source: usize,
    destination: usize,
    length: usize
}

impl AlmanacMapping {
    fn get(&self, source: &usize) -> Option<usize> {
        if *source >= self.source && *source < (self.source + self.length) {
            let diff = source - self.source;
            return Some(self.destination + diff);
        } else {
            return None;
        }
    }
}

impl AlmanacMap {
    fn find_location_for_seed(&self, seed: &usize) -> usize {
        let soil = self.find_destination(&self.seed_to_soil, &seed);
        let fertilizer = self.find_destination(&self.soil_to_fertilizer, &soil);
        let water = self.find_destination(&self.fertilizer_to_water, &fertilizer);
        let light = self.find_destination(&self.water_to_light, &water);
        let temperature = self.find_destination(&self.light_to_temperature, &light);
        let humidity = self.find_destination(&self.temperature_to_humidity, &temperature);
        let location = self.find_destination(&self.humidity_to_location, &humidity);
        return location;
    }

    fn find_destination(&self, mapping: &Vec<AlmanacMapping>, source: &usize) -> usize {
        mapping.iter().find_map(|map| map.get(source)).unwrap_or(*source)
    }
}

pub fn aoc_5_1() -> usize {
    return parse_and_find_nearest_location("res/aoc5.txt");
}

pub fn aoc_5_2() -> usize {
    return parse_with_pairs_and_find_nearest_location("res/aoc5.txt");
}

fn parse_and_find_nearest_location(input_file: &str) -> usize {
    let input_data = read_or_panic(input_file);
    let almanac = parse_input_data(&input_data);
    return almanac.seeds.iter().map(|seed| almanac.map.find_location_for_seed(seed)).min().unwrap();
}

fn parse_with_pairs_and_find_nearest_location(input_file: &str) -> usize {
    let input_data = read_or_panic(input_file);
    let almanac = parse_input_data_with_seed_pairs(&input_data);
    return almanac.seeds.iter().map(|seed_range| {
        let mut min = almanac.map.find_location_for_seed(&seed_range.start);
        for seed in (seed_range.start+1)..(seed_range.start + seed_range.length) {
            let location = almanac.map.find_location_for_seed(&seed);
            if location < min {
                min = location;
            }
        }
        return min;
    }).min().unwrap();
}
static ALMANAC_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(
r"^seeds:(?<seeds>(\s*\d+)+)\n
seed-to-soil map:
(?<seed_to_soil>((\d+\s*)*\n*)*)
soil-to-fertilizer map:
(?<soil_to_fertilizer>((\d+\s*)*\n*)*)
fertilizer-to-water map:
(?<fertilizer_to_water>((\d+\s*)*\n*)*)
water-to-light map:
(?<water_to_light>((\d+\s+)+\n*)*)
light-to-temperature map:
(?<light_to_temperature>((\d+\s*)*\n*)*)
temperature-to-humidity map:
(?<temperature_to_humidity>((\d+\s*)*\n*)*)
humidity-to-location map:
(?<humidity_to_location>((\d+\s*)*\n*)*)").unwrap());

fn parse_input_data(input_data: &str) -> Almanac {
    return ALMANAC_REGEX.captures(input_data).map(|matched_almanac| {
        Almanac {
            seeds: parse_numbers(&matched_almanac, "seeds"),
            map: AlmanacMap {
                seed_to_soil: parse_map(&matched_almanac, "seed_to_soil"),
                soil_to_fertilizer: parse_map(&matched_almanac, "soil_to_fertilizer"),
                fertilizer_to_water: parse_map(&matched_almanac, "fertilizer_to_water"),
                water_to_light: parse_map(&matched_almanac, "water_to_light"),
                light_to_temperature: parse_map(&matched_almanac, "light_to_temperature"),
                temperature_to_humidity: parse_map(&matched_almanac, "temperature_to_humidity"),
                humidity_to_location: parse_map(&matched_almanac, "humidity_to_location"),
            },
        }
    }).unwrap();
}

fn parse_input_data_with_seed_pairs(input_data: &str) -> AlmanacWithSeedRange {
    return ALMANAC_REGEX.captures(input_data).map(|matched_almanac| {
        AlmanacWithSeedRange {
            seeds: create_seed_ranges_from_pairs(&parse_numbers(&matched_almanac, "seeds")),
            map: AlmanacMap {
                seed_to_soil: parse_map(&matched_almanac, "seed_to_soil"),
                soil_to_fertilizer: parse_map(&matched_almanac, "soil_to_fertilizer"),
                fertilizer_to_water: parse_map(&matched_almanac, "fertilizer_to_water"),
                water_to_light: parse_map(&matched_almanac, "water_to_light"),
                light_to_temperature: parse_map(&matched_almanac, "light_to_temperature"),
                temperature_to_humidity: parse_map(&matched_almanac, "temperature_to_humidity"),
                humidity_to_location: parse_map(&matched_almanac, "humidity_to_location"),
            },
        }
    }).unwrap();
}

fn create_seed_ranges_from_pairs(pairs: &Vec<usize>) -> Vec<SeedRange> {
    let mut seeds = Vec::new();
    for i in (0..pairs.len()).step_by(2) {
        let initial_seed = pairs[i];
        let length = pairs[i+1];
        seeds.push(SeedRange {
            start: initial_seed,
            length,
        });
    }
    return seeds;
}

fn parse_map(capture: &Captures, group_name: &str) -> Vec<AlmanacMapping>{
    capture.name(group_name).unwrap().as_str().split("\n").fold(Vec::new(), |mut accu, line| {
        let mapping: Vec<usize> = line.split(" ").filter(|num| !num.is_empty()).map(|num| num.parse().unwrap()).collect();
        if mapping.len() > 0 {
            accu.push(AlmanacMapping {
                source: mapping[1],
                destination: mapping[0],
                length: mapping[2],
            });
        }
        accu
    })
}

#[cfg(test)]
mod tests {
    use crate::aoc5::{parse_and_find_nearest_location, parse_with_pairs_and_find_nearest_location};

    #[test]
    fn aoc_5_1() {
        assert_eq!(parse_and_find_nearest_location( "res/aoc5-example.txt"), 35)
    }

    #[test]
    fn aoc_5_2() {
        assert_eq!(parse_with_pairs_and_find_nearest_location( "res/aoc5-example.txt"), 46)
    }
}
