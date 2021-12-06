use std::fs;

pub fn main() {
    // PART 1
    // read file from disk
    let contents = fs::read_to_string("src/data/day2.txt").unwrap();
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
