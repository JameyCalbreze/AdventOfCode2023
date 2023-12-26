use std::vec::Vec;
use std::fs;
use std::io;
use std::io::BufRead;

pub fn prep_input(file_path: &str) -> Vec<String> {
    let f = fs::File::open(file_path).unwrap();

    let mut reader = io::BufReader::new(f);

    let mut input: Vec<String> = Vec::new();
    let mut chars_read = 1;
    while chars_read > 0 {
        let mut s = String::new();
        chars_read = reader.read_line(&mut s).unwrap();
        if chars_read > 0 {
            input.push(String::from(s.trim()));
        }
    }

    input
}

pub fn get_number_from_str(num_str: &str) -> u64 {
    let mut num: u64 = 0;
    for c in num_str.chars() {
        num *= 10;
        num += u64::from(c.to_digit(10).unwrap());
    }
    num
}

// Read a number from the string and write it to storage on termination
// will return gracefully from the end of the line
// returns the number of characters read
pub fn read_and_store_nums_until_terminator(input: &str, terminator: char, storage: &mut Vec<u64>) -> usize {
    let mut chars_read = 0;

    let mut num = String::new();
    for c in input.chars() {
        if c == terminator { break; }
        if c >= '0' && c <= '9' {
            num.push(c);
        } else if !num.is_empty() {
            storage.push(get_number_from_str(&num));
            num.clear();
        }
        chars_read += 1
    }

    if !num.is_empty() {
        storage.push(get_number_from_str(&num));
    }

    chars_read
}

/**
 * Find the middle value between two numbers
 * 
 * At the moment this function assumes both numbers are positive
 */
pub fn get_middle_value(min_val: i64, max_val: i64) -> i64 {
    // Check if sum is odd
    let mut total = min_val + max_val;

    if total % 2 == 1 {
        total += 1;
    }

    total / 2
}

/**
 * For simplicity this assumes that each of these strings are the same length
 */
pub fn count_char_diff_between_strings(s1: &str, s2: &str) -> usize {
    let mut differences = 0;
    let length = s1.len();

    let mut chars1 = s1.chars();
    let mut chars2 = s2.chars();

    for _ in 0..length {
        if !chars1.next().unwrap().eq(&chars2.next().unwrap()) {
            differences += 1;
        }
    }

    differences
}
