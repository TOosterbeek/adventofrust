use crate::aoc::Puzzle;
use crate::days::day1::Day1;

pub mod aoc;
mod days;

fn main() {
    let day1 = Day1::new(1);
    day1.solve_part_1();
}
