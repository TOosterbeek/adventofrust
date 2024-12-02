use crate::aoc::{read_input, Puzzle};

pub struct Day1 {
    pub puzzle_input: String,
}

impl Puzzle for Day1 {
    fn new(day_number: i8) -> Day1 {
        Day1 { puzzle_input: read_input(day_number) }
    }

    fn solve_part_1(&self) -> () {
        println!("{}", self.puzzle_input);
    }

    fn solve_part_2() -> () {
        todo!()
    }
}
