use std::collections::HashMap;
use std::hash::Hash;
use crate::utils::prep_input;

pub fn get_solution() {
    let input = prep_input("input/problem_03.txt");
    problem_three_part_one(&input);
}

fn problem_three_part_one(input: &Vec<String>) {

    println!("The sum of the numbers next to symbols: {}", )
}

struct Coordinate {
    row: i64,
    column: i64
}

struct Number {
    id_cord: Coordinate,
    len: usize,
    value: u64
}

fn get_numbers_from_string(line: &str, row: i64) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    let mut column = 0;
    let mut cur_number = String::new();
    for c in line.chars() {
        if c >= '0' && c <= '9' {
            cur_number.push(c);
        } else if !cur_number.is_empty() {
            numbers.push(Number {
                id_cord: Coordinate {
                    row: row.clone(),
                    column: column.clone()
                },
                len: cur_number.len().clone(),
                value:
            })
        }
        column += 1
    }

    numbers
}

