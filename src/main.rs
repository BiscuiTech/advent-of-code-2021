use std::fs;

use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

fn main() -> std::io::Result<()> {
    // array of 24 day strings, ie: 'day 1', 'day 2', 'day 3', ...
    let items = [
        "day 1", "day 2", "day 3", "day 4", "day 5", "day 6", "day 7", "day 8", "day 9", "day 10",
        "day 11", "day 12", "day 13", "day 14", "day 15", "day 16", "day 17", "day 18", "day 19",
        "day 20", "day 21", "day 22", "day 23", "day 24",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => println!("User selected item : {:?}", items[index]),
        None => println!("User did not select anything"),
    }

    match selection {
        Some(0) => day_1(),
        Some(1) => day_2(),
        _ => println!("User did not select anything"),
    }

    Ok(())
}

fn day_1() {
    // PART 1
    // read file from disk
    let contents = fs::read_to_string("src/day1/readings.txt").unwrap();
    let split_string: Vec<&str> = contents.split("\n").collect();
    // convert string to int
    let readings: Vec<i32> = split_string
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    // iterate over readings
    let mut total_increases = 0;
    for i in 0..readings.len() {
        if i > 0 && readings[i] > readings[i - 1] {
            total_increases += 1;
        }
    }
    println!("part 1: {:?}", total_increases);

    // PART 2
    // iterate over readings
    let mut total_combined_increases = 0;

    for i in 0..readings.len() {
        // check if there are enough items to compare (4 items)

        if readings.len() - i >= 4 {
            // combine the first 3 items
            let low_combined_readings = readings[i] + readings[i + 1] + readings[i + 2];
            let high_combined_readings = readings[i + 1] + readings[i + 2] + readings[i + 3];
            // check if the high readings are higher than the low readings
            if high_combined_readings > low_combined_readings {
                total_combined_increases += 1;
            }
        } else {
            break;
        }
    }
    println!("part 2: {:?}", total_combined_increases);
}

fn day_2() {
    // PART 1
    // read file from disk
    let contents = fs::read_to_string("src/day2/instructions.txt").unwrap();
    let split_string: Vec<&str> = contents.split("\n").collect();
    // Vector of strings, each string is composed of a string and a int
    let mut instructions: Vec<(String, i32)> = Vec::new();
    for s in split_string {
        let split_string: Vec<&str> = s.split(" ").collect();
        let instruction = split_string[0];
        let value = split_string[1].parse::<i32>().unwrap();
        instructions.push((instruction.to_string(), value));
    }
    let mut depth = 0;
    let mut horizontal = 0;

    for i in 0..instructions.len() {
        match instructions[i].0.as_str() {
            "forward" => horizontal += instructions[i].1,
            "up" => depth += instructions[i].1,
            "down" => depth -= instructions[i].1,
            _ => (),
        };
    }
    println!(
        "part 1: {:?} * {:?} = {:?}",
        horizontal.abs(),
        depth.abs(),
        horizontal.abs() * depth.abs()
    );

    // PART 2
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for i in 0..instructions.len() {
        match instructions[i].0.as_str() {
            "forward" => {
                horizontal += instructions[i].1;
                depth += aim * instructions[i].1;
            }
            "up" => aim -= instructions[i].1,
            "down" => aim += instructions[i].1,
            _ => (),
        };
    }
    println!(
        "part 2: {:?} * {:?} = {:?}",
        horizontal.abs(),
        depth.abs(),
        horizontal.abs() * depth.abs()
    );
}
