use crate::aoc::{read_input, Puzzle};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufReader, Lines};

pub struct Day1 {
    pub puzzle_input: io::Result<Lines<BufReader<File>>>,
}

impl Day1 {
    fn split_input_to_arrays(self, left_array: &mut [i32; 1000], right_array: &mut [i32; 1000]) {
        if let Ok(lines) = self.puzzle_input {
            for (i, line) in lines.flatten().enumerate() {
                let left: &str = &line[..5];
                let right: &str = &line[8..];

                left_array[i] = left.parse::<i32>().unwrap();
                right_array[i] = right.parse::<i32>().unwrap();
            }
        }
    }
}

impl Puzzle for Day1 {
    fn new(day_number: i8) -> Day1 {
        Day1 {
            puzzle_input: read_input(day_number),
        }
    }

    fn solve_part_1(self) -> () {
        let mut left_array: [i32; 1000] = [0; 1000];
        let mut right_array: [i32; 1000] = [0; 1000];

        self.split_input_to_arrays(&mut left_array, &mut right_array);

        left_array.sort();
        right_array.sort();

        let result: i32 = left_array
            .iter()
            .zip(right_array)
            .map(|(x, y)| (x - y).abs())
            .sum();

        println!("result: {}", result)
    }

    fn solve_part_2(self) -> () {
        let mut left_array: [i32; 1000] = [0; 1000];
        let mut right_array: [i32; 1000] = [0; 1000];

        self.split_input_to_arrays(&mut left_array, &mut right_array);

        // Think there might be a nice one-liner for this, cant be arsed rn though
        let mut result_map: HashMap<i32, i32> = HashMap::new();
        let count_map: HashMap<i32, i32> = HashMap::new();

        for left in left_array {
            let mut count: i32 = 0;
            if count_map.contains_key(&left) {
                count = *count_map.get(&left).unwrap();
            } else {
                count = right_array.iter().filter(|&&right| right == left).count() as i32;
            }

            result_map.insert(left, left * count);
        }

        println!("result2: {}", result_map.values().sum::<i32>())
    }
}
