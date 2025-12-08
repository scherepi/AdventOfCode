use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("Advent of Code 2025: Day 2!");
    println!("Retrieving problem data");

    // Create a path to the file
    let path = Path::new("data/idRanges.txt");
    let display = path.display();

    // Open the path in read-only mode
    let mut file = match File::open(&path) {
        Err(why) => { panic!("Couldn't open {}: {}", display, why); },
        Ok(file) => file,
    };

    // Read the file contents into a string, returning `io::Result<usize>`
    let mut problem_data = String::new();
    match file.read_to_string(&mut problem_data) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => find_invalid_ids(problem_data), // run the countZeroes function on the program data if we read the file successfully
    };

}

fn find_invalid_ids(problem_data: String) {
    let mut result_number: usize = 0;
    println!("problem data: {}", problem_data);

    let vec: Vec<&str> = problem_data.split(",").collect();
    for value in vec {
        println!("Range to examine: {}", value);
        let id_range = String::from(value);
        let range_indices: Vec<&str> = id_range.split("-").collect();
        println!("First number: {}\nSecond number: {}\n", range_indices[0], range_indices[1]);
        // convert to usable numbers (usize)
        let first_number = range_indices[0].parse::<usize>().unwrap();
        let last_number = String::from(range_indices[1]).parse::<usize>().unwrap();

        let mut counter = first_number; // set the counter to the first number in the range

        while counter <= last_number {
            // inclusive search of the range
            if !validate_id(counter) { println!("[!!!] Invalid value: {}", counter); result_number += counter; }
            counter += 1;
        }
        
    }

    println!("The resulting number is: {}", result_number);
}

fn validate_id(id: usize) -> bool {
    // an id is valid if it's not a twice-repeating-sequence pattern like '11' or '123123'
    let id_as_string = id.to_string();
    //println!("String length: {}", id_as_string.len());
    // this means we can quickly discard anything with an odd amount of digits!
    if id_as_string.len() % 2 == 1 { return true; } 

    let (half_1, half_2) = id_as_string
                                       .split_at(id_as_string.len() / 2);
    //println!("Value: {}\nFirst half: {}\nSecond half: {}", id_as_string, half_1, half_2);
    if half_1 == half_2 { return false; }
    return true;
}