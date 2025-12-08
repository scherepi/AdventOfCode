use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("Advent of Code 2025: Day 1!");
    println!("Retrieving problem data");

    // Create a path to the file
    let path = Path::new("data/combos.txt");
    let display = path.display();

    // Open the path in read-only mode
    let mut file = match File::open(&path) {
        Err(why) => { panic!("Couldn't open {}: {}", display, why); },
        Ok(file) => file,
    };

    // Read the file contents into a string, returning `io::Result<usize>`
    let mut problemData = String::new();
    match file.read_to_string(&mut problemData) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => countZeroes(problemData), // run the countZeroes function on the program data if we read the file successfully
    };

}

fn countZeroes(problem_data: String) {
    let mut dial_number: i16 = 50; // dial starts at 50
    let mut num_zeroes: i16 = 0; // counter for number of times dial hits '0'

    for line in problem_data.lines() {
        println!("Dial at {}", dial_number);
        let mut combo = String::from(line);
        let direction: char = combo.remove(0); // get the first character, either 'L' or 'R', for direction
        let weight: i16 = match combo.parse::<i16>() {
            Err(_) => panic!("issue converting leftover string to i16"),
            Ok(weight) => weight % 100,
        }; // parse the rest of the line as an i16 number for compatibility with dial_number
        if direction == 'R' {
            dial_number += weight;
            if (dial_number > 99) { dial_number -= 100; }
            if (dial_number == 0) { num_zeroes += 1; }
        } else if (direction == 'L') {
            dial_number -= weight;
            if (dial_number < 0) { dial_number += 100; }
            if (dial_number == 0) { num_zeroes += 1; }
        } else {
            panic!("Something has gone horribly wrong.");
        }
    }

    println!("The number of zeroes is: {}", num_zeroes);
}