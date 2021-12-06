use std::fs;
use std::u32;

pub fn main() {
    let contents = fs::read_to_string("src/data/day3.txt").unwrap();
    let split_string: Vec<&str> = contents.split('\n').collect();
    // collect the position of the string into a vector
    let mut matrix = vec![String::new(); split_string[0].chars().count()];
    // fill the matrix with the string
    // println!("chars count : {}", split_string[0].chars().count());
    // println!("lines count : {}", split_string.len());
    // let mut t = String::new();
    // t.push('t');
    // println!("test {:?}", t);
    // println!("split_string : {:?}", split_string);
    for char_i in 0..split_string[0].chars().count() {
        for vec_i in 0..split_string.len() {
            // push to matrix the char at the right position
            matrix[char_i].push(split_string[vec_i].chars().nth(char_i).unwrap());
        }
    }
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    // calculate the rate of gamma rate by checking the most frequent character
    struct BinaryCounter {
        zero: u32,
        one: u32,
    }
    for binary_str in matrix {
        let mut counter = BinaryCounter { zero: 0, one: 0 };
        for binary in binary_str.chars() {
            match binary {
                '0' => counter.zero += 1,
                '1' => counter.one += 1,
                _ => (),
            }
        }
        if counter.zero > counter.one {
            epsilon_rate.push('1');
            gamma_rate.push('0');
        } else {
            epsilon_rate.push('0');
            gamma_rate.push('1');
        }
    }
    println!("Epsilon rate: {}", epsilon_rate);
    println!("Gamma rate: {}", gamma_rate);
    let mut epsilon_decimal: u128 = u128::from_str_radix(&epsilon_rate, 2).unwrap();
    let mut gamma_decimal: u128 = u128::from_str_radix(&gamma_rate, 2).unwrap();

    println!("Epsilon decimal: {}", epsilon_decimal);
    println!("Gamma decimal: {}", gamma_decimal);
    println!("Answer: {}", epsilon_decimal * gamma_decimal);
}
