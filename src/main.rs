use crate::aoc1::{aoc_1_1, aoc_1_2};
use crate::aoc2::{aoc_2_1, aoc_2_2};
use crate::aoc3::{aoc_3_1, aoc_3_2};
use crate::aoc4::{aoc_4_1, aoc_4_2};

mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod fs_util;

fn main() {
    println!("AoC 1.1: {}", aoc_1_1());
    println!("AoC 1.2: {}", aoc_1_2());
    println!("AoC 2.1: {}", aoc_2_1());
    println!("AoC 2.2: {}", aoc_2_2());
    println!("AoC 3.1: {}", aoc_3_1());
    println!("AoC 3.2: {}", aoc_3_2());
    println!("AoC 4.1: {}", aoc_4_1());
    println!("AoC 4.2: {}", aoc_4_2());
}
