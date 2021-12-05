use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[derive(Debug)]
pub struct UnableToParseToVector;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Day One solution: take windows of length 2, then iterate and if increasing
// add to the previous day. If not skip it.
fn day_one(dataset: &Vec<i64>) -> u32 {

    let mut counter: u32 = 0;

    for group in dataset.windows(2).into_iter() {

        if group[1] - group[0] > 0 {
            counter += 1
        }

    };

    counter
}


/// Day Two solution: take windows of length 3 and compare sums, if the sums
/// increase then there is an increase, else no change. Compute the number
/// of increases in the 3-window sums
fn day_two(dataset: &Vec<i64>) -> u32 {

    let mut counter: u32 = 0;
    
    for window_data in dataset.windows(4).into_iter() {

        let g1 = window_data[0..3].to_vec();
        let g2 = window_data[1..].to_vec();

        let g1_sum: i64 = g1.iter().sum();
        let g2_sum: i64 = g2.iter().sum();

        if g2_sum - g1_sum > 0 {
            counter += 1;
        }

    }

    counter
}


fn main() {
    println!("This is Advent Of Code...");
    println!("Welcome to Day 1");

    let path_to_file = "./data/d1.csv";

    let result = match read_lines(path_to_file) {
        Ok(lines) => {
            let mut vector: Vec<i64> = Vec::new();

            for data in lines {
                vector.push(data.unwrap().parse::<i64>().unwrap())
            };

            Ok(vector)
        }
        Err(err) => {
            Err(UnableToParseToVector)
        }
    };

    let dataset = result.unwrap();

    // Runs the day 1 solution
    let day_one_solution = day_one(&dataset);

    // Runs the day 2 solution
    let day_two_solution = day_two(&dataset);

    println!("DAY 1 : Count of increases: {}", day_one_solution);
    println!("DAY 2 : Count of sum increases: {}", day_two_solution);

}
