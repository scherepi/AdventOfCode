use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("Advent of Code 2025: Day 3!");
    println!("[!] Retrieving problem data");

    let path: &Path = Path::new("data/joltageratings.txt");
    let display = path.display();

    let mut file: File = match File::open(&path) {
        Err(why) => { panic!("Couldn't open {}: {}", display, why)},
        Ok(file) => file,
    };

    let mut problem_data: String = String::new();
    match file.read_to_string(&mut problem_data) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => sum_joltages(problem_data)
    };
}

fn sum_joltages(problem_data: String) {
    let mut total_output_joltage: usize = 0;

    for line in problem_data.lines() {
        println!("Joltage at {}", total_output_joltage);
        let bank = String::from(line);
        total_output_joltage += find_joltage(bank);
    }
    println!("Total output joltage: {}", total_output_joltage);
}

// Fastest way (that i can think of) to do this is to find the biggest number in the bank and start at that index, then find the biggest number to the right of it
fn find_joltage(bank: String) -> usize {
    let digits: Vec<u32> = bank
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    
    if digits.len() < 2 {
        return 0;
    }
    
    let mut max_joltage: u32 = 0;
    
    // For each digit, find the maximum digit that comes after it
    // and calculate the two-digit number
    for i in 0..digits.len() - 1 {
        // Find max digit after position i
        let max_after = digits[i + 1..].iter().max().unwrap();
        let joltage = digits[i] * 10 + max_after;
        if joltage > max_joltage {
            max_joltage = joltage;
        }
        // Early exit: can't do better than 99
        if max_joltage == 99 {
            break;
        }
    }
    
    max_joltage as usize
}