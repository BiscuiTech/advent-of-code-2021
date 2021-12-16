use std::u32;

struct BinaryCounter {
    zero: u32,
    one: u32,
}

struct Rates {
    epsilon: String,
    gamma: String,
}

struct Life_Support_Ratings {
    oxygen_generator: String,
    co2_scrubber: String,
}

pub fn main() {
    let split_string = crate::utils::read_file("src/data/day3.txt");
    // collect the position of the string into a vector
    let mut matrix = vec![String::new(); split_string[0].chars().count()];
    // fill the matrix with the string
    for char_i in 0..split_string[0].chars().count() {
        for vec_i in 0..split_string.len() {
            // push to matrix the char at the right position
            matrix[char_i].push(split_string[vec_i].chars().nth(char_i).unwrap());
        }
    }
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    // calculate the rate of gamma rate by checking the most frequent character

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
    let epsilon_decimal: u128 = u128::from_str_radix(&epsilon_rate, 2).unwrap();
    let gamma_decimal: u128 = u128::from_str_radix(&gamma_rate, 2).unwrap();

    println!("Epsilon decimal: {}", epsilon_decimal);
    println!("Gamma decimal: {}", gamma_decimal);
    println!("Answer: {}", epsilon_decimal * gamma_decimal);
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &str = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";

    #[test]
    fn test_part2() {
        let split_content: Vec<String> = input.split('\n').map(|x| x.to_string()).collect();
        for char_index in 0..split_content[0].len() {
            // let mut acc_vec: Vec<u8> = Vec::new();
            let mut counter = BinaryCounter { zero: 0, one: 0 };
            for binary_string in &split_content {
                let char = binary_string
                    .chars()
                    .nth(char_index)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as u8;
                // acc_vec.push(char);
                if char == 0 {
                    counter.zero += 1;
                } else {
                    counter.one += 1;
                }
            }
        }
    }
}
