use crate::utils::prep_input;

pub fn get_solution() {
    let input = prep_input("input/problem_01.txt");
    println!("Problem 1: {}", problem_one(&input));
    println!("Problem 1 Part 2: {}", problem_one_part_two(&input));
}

fn problem_one(input: &Vec<String>) -> usize {
    let mut running_sum: usize = 0;

    let mut current_num = String::new();
    for s in input {
        for c in s.chars() {
            if c >= '0' && c <= '9' {
                current_num.push(c);
            }
        }

        // Avoid the case where we may have a trailing empty line or the case where there aren't
        // numbers in each of the input lines
        if current_num.len() > 0 {
            let mut result = String::new();
            result.push(current_num.chars().nth(0).unwrap());
            result.push(current_num.chars().nth(current_num.len()-1).unwrap());

            running_sum += result.trim()
                .parse::<usize>()
                .expect("Failed to parse selected text!");
        }

        current_num.clear();
    }

    running_sum
}

fn problem_one_part_two(input: &Vec<String>) -> u64 {
    let mut running_sum: u64 = 0;

    for s in input {
        if s.trim().is_empty() { continue; }

        let indexes = build_index(s.trim());
        let min_num = indexes.iter().min().unwrap().clone().1;
        let max_num = indexes.iter().max().unwrap().clone().1;
        running_sum += u64::from((min_num*10) + max_num);
    }

    running_sum
}

fn numbers() -> [&'static str; 9] {
    ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
}

fn build_index(input: &str) -> Vec<(usize, u32)> {
    let mut indexes: Vec<(usize, u32)> = Vec::new();

    let mut val = 1;
    for num in numbers() {
        match input.find(num) {
            Some(i) => indexes.push((i, val)),
            None => ()
        }
        match input.rfind(num) {
            Some(i) => indexes.push((i,val)),
            None => ()
        }
        val += 1;
    }

    let mut position = 0;
    for c in input.chars() {
        if c >= '1' && c <= '9' {
            indexes.push((position, c.to_digit(10).unwrap()));
        }
        position += 1
    }

    indexes
}
