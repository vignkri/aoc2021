use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub struct UnableToParseToVector;


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


// Define function to compute the distance travelled
pub fn day1_one(dataset: &Vec<String>) -> i64 {

    // Initialise a submarine
    let mut submarine = Submarine::new();

    for row in dataset {

        let action_magnitude: Vec<&str> = row.split(" ").collect();

        let action = action_magnitude[0];
        let magnitude: i64 = action_magnitude[1].parse::<i64>().unwrap();

        if action == "forward" {
            submarine = submarine.move_forward(magnitude);
        } else if action == "up" {
            submarine = submarine.move_up(magnitude);
        } else if action == "down" {
            submarine = submarine.move_down(magnitude);
        }
        // println!("After every action: {:?}", submarine);
    }

    submarine.travelled_units()
}



#[derive(Debug)]
pub struct SubmarineAdvanced {
    x: i64,
    y: i64,
    aim: i64,
    depth: i64,
}



#[derive(Debug)]
pub struct Submarine {
    x: i64,
    y: i64,
}


// Enables control of the submarine by providing
// movement transitions from one position to another
pub trait Controller {
    fn move_forward(&self, units: i64) -> Submarine;
    fn move_down(&self, units: i64) -> Submarine;
    fn move_up(&self, units: i64) -> Submarine;
}



// Implements standard functions for a submarine
impl Submarine {

    // Build a new submarine
    pub fn new() -> Self {
        Submarine { x: 0, y: 0, }
    }

    // Compute units travelled of the submarine
    pub fn travelled_units(&self) -> i64 {

        self.x * self.y
    }

}


impl Controller for Submarine {

    // Move the submarine forward
    fn move_forward(&self, units: i64) -> Submarine {

        let movement = self.x + units;

        Submarine { x: movement, y: self.y.to_owned() }
    }

    // Move the submarine deeper
    fn move_down(&self, units: i64) -> Submarine {

        let movement= self.y + units;

        Submarine { x: self.x.to_owned(), y: movement }
    }

    // Move the submarine upward
    fn move_up(&self, units: i64) -> Submarine {

        let movement= self.y - units;

        Submarine { x: self.x.to_owned(), y: movement }
    }
}