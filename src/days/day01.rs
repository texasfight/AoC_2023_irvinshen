use crate::{Solution, SolutionPair};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

///////////////////////////////////////////////////////////////////////////////

fn  generate_calibration_values(file_contents: &String) -> Vec<u16> {
    let all_lines = file_contents.split("\n");
    let mut numbers: Vec<u16> = Vec::new();

    for line in all_lines {
        let mut first: Option<u32> = None;
        let mut last = 0;
        // let mut prior_string = Vec::new();
        for char in line.chars() {
            if char.is_numeric() {
                if let Some(value) = first {
                    first = Some(value)
                } else {
                    first = Some(char.to_digit(10).unwrap());
                }
                last = char.to_digit(10).unwrap();
            }
        }

        numbers.push((first.unwrap() * 10 + last) as u16);
    }
    numbers
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let path = Path::new("input/day1.txt");
    let path_name = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Unable to open {}: {}", path_name, why),
        Ok(file) => file,
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("couldn't read {}: {}", path_name, why),
        Ok(_) => (),
    }

    let cal_values_1 = generate_calibration_values(&file_contents);


    let sol1: u16 = cal_values_1.iter().sum();

    let modified_file_contents = file_contents.replace("one", "o1e").
        replace("two", "t2o").replace("three", "t3e").replace("four", "f4r").
        replace("five", "f5e").replace("six", "s6x").replace("seven", "s7n").
        replace("eight", "e8t").replace("nine", "n9e").replace("zero", "z0o");

    let cal_values_2 = generate_calibration_values(&modified_file_contents);


    let sol2: u16 = cal_values_2.iter().sum();

    (Solution::from(sol1), Solution::from(sol2))
}
