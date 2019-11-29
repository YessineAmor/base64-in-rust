use std::char;
use std::io;
use std::str;

fn decimal_to_binary(number: i8) -> String {
    let mut result = String::new();
    // Need number to be mutable
    let mut new_number = number;
    // Divide number by 2 until result of division = 0
    loop {
        if 0 != new_number {
            result = format!("{}{}", new_number % 2, result);
            new_number = new_number / 2;
        } else {
            // Add zeroes to make it an 8 bit binary representation
            if result.len() < 8 {
                for _ in 1..8 - result.len() + 1 {
                    result = format!("0{}", result);
                }
            }
            break;
        }
    }
    return result;
}
fn main() {
    println!("Input your text");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read text");
    let clean_input = input.trim().replace(' ', "");
    let input_length = clean_input.len();
    if input_length % 3 == 0 {
        let mut binary_string = String::new();
        for c in clean_input.chars() {
            let ascii_value = c as i8;
            binary_string = format!("{}{}", binary_string, decimal_to_binary(ascii_value));
        }
        println!("{}", binary_string);
    } else {
    }
}
