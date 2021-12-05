use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Sub;

#[derive(Debug)]
pub struct UnableToParseToVector;


#[derive(Debug)]
pub struct GammaRate {
    value: i32,
}

#[derive(Debug)]
pub struct EpsilonRate {
    value: i32,
}


// Return strings. We shall use the strings to compute the elements later
pub fn parser(input_data: io::Result<io::Lines<io::BufReader<File>>>) -> Vec<String> {

    let result = match input_data {
        Ok(lines) => {
            let mut vector: Vec<String> = Vec::new();

            for data in lines {

                let content = data.unwrap();

                vector.push(content);
            };

            Ok(vector)
        }
        Err(err) => {
            Err(UnableToParseToVector)
        }
    };

    result.unwrap()
}


pub fn day3_one(dataset: &Vec<String>) -> i32 {


    // This is the iteration counter
    let number_of_elements = dataset[0].len();
    let number_of_measurements = dataset.len();
    let (gamma, epsilon) = (0..number_of_elements)
        .map(|i| dataset.iter().filter(|line| line.as_bytes()[i] == b'1').count())
        .fold((0, 0), |(gamma, epsilon), ones| {
            if ones * 2 > number_of_measurements {
                ((gamma << 1) | 1, epsilon << 1)
            } else {
                (gamma << 1, (epsilon << 1) | 1)
            }
        });

    let power_consumption = gamma * epsilon;

    power_consumption
}
