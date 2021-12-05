use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[derive(Debug)]
pub struct UnableToParseToVector;


pub fn parser(input_data: io::Result<io::Lines<io::BufReader<File>>>) -> Vec<i64> {

    let result = match input_data {
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

    result.unwrap()
}


// Day One.1 solution: take windows of length 2, then iterate and if increasing
// add to the previous day. If not skip it.
pub fn day1_one(dataset: &Vec<i64>) -> u32 {

    let mut counter: u32 = 0;

    for group in dataset.windows(2).into_iter() {

        if group[1] - group[0] > 0 {
            counter += 1
        }

    };

    counter
}


/// Day One.2 solution: take windows of length 3 and compare sums, if the sums
/// increase then there is an increase, else no change. Compute the number
/// of increases in the 3-window sums
pub fn day1_two(dataset: &Vec<i64>) -> u32 {

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
