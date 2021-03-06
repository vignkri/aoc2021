use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, self};


pub mod day_one;
pub mod day_two;
pub mod day_three;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("This is Advent Of Code...");

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

    // Day two solution
    let path_to_file = "./data/day2.txt";

    // parsed dataset
    let dataset = day_two::parser(read_lines(path_to_file));
    // runs thee solution for day2.1
    let submarine_units = day_two::day2_one(&dataset);
    // runs the solution for day2.2
    let submarine_aim_depth = day_two::day2_two(&dataset);
    
    println!("DAY 2.1 : Distance travelled by submarine: {}", submarine_units);
    println!("DAY 2.2 : Distance travelled by submarine: {}", submarine_aim_depth);

    // Day three solution
    let path_to_file = "./data/day3.txt";

    // parsed dataset
    let dataset = day_three::parser(read_lines(path_to_file));
    let power_consumption = day_three::day3_one(&dataset);

    println!("DAY 3.1 : Power consumed: {}", power_consumption);


}
