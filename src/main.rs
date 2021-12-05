use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub mod day_one;


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("This is Advent Of Code...");
    println!("Welcome to Day 1");

    // Day one solution
    let path_to_file = "./data/day1.csv";

    // Parse the file to useful solution
    let dataset = day_one::parser(read_lines(path_to_file));
    // Runs the day 1 solution
    let day_one_1_solution = day_one::day1_one(&dataset);
    // Runs the day 2 solution
    let day_one_2_solution = day_one::day1_two(&dataset);

    println!("DAY 1.1 : Count of increases: {}", day_one_1_solution);
    println!("DAY 1.2 : Count of sum increases: {}", day_one_2_solution);

}
