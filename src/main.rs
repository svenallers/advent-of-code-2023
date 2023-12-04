use crate::aoc1::{aoc_1_1, aoc_1_2};
use crate::aoc2::{aoc_2_1, aoc_2_2};

mod aoc1;
mod aoc2;
mod fs_util;

fn main() {
    println!("AoC 1.1: {}", aoc_1_1());
    println!("AoC 1.2: {}", aoc_1_2());
    println!("AoC 2.1: {}", aoc_2_1());
    println!("AoC 2.2: {}", aoc_2_2());
}
