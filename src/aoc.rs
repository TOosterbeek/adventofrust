use std::fs;


pub fn read_input(day_number : i8) -> String {
    return fs::read_to_string(format!("./res/day{day_number}.txt"))
        .expect("Should have been able to read puzzle input from ./res/day[...].txt");
}

pub trait Puzzle {

    fn new(day_number: i8) -> Self;

    fn solve_part_1(&self) -> ();
    fn solve_part_2() -> ();
}