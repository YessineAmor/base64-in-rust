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

fn binary_to_decimal(binary: &str) -> i8 {
    let mut result: i8 = 0;
    if &binary[0..1] == "1" {
        result += 32;
    }
    if &binary[1..2] == "1" {
        result += 16;
    }
    if &binary[2..3] == "1" {
        result += 8;
    }
    if &binary[3..4] == "1" {
        result += 4;
    }
    if &binary[4..5] == "1" {
        result += 2;
    }
    if &binary[5..6] == "1" {
        result += 1;
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
        for _ in 0..(input_length * 8) / 6 {
            let substring = &binary_string[0..6];
            println!("{}", substring);
            println!("{}", binary_to_decimal(substring));
            binary_string = binary_string[6..].to_string();
        }
        println!("{}", binary_string);
    } else {
    }
}
