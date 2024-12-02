use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub fn read_input(day_number: i8) -> io::Result<Lines<BufReader<File>>> {
    read_file_by_line(format!("./res/day{day_number}.txt"))
}

fn read_file_by_line<P>(file_path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}

pub trait Puzzle {
    fn new(day_number: i8) -> Self;
    fn solve_part_1(self) -> ();
    fn solve_part_2(self) -> ();
}
