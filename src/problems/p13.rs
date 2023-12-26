use crate::utils;

pub fn get_solution() {
    let input: Vec<String> = utils::prep_input("input/problem_13.txt");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<String>) -> usize {
    let patterns = produce_patterns_from_input(input);
    
    let mut answer = 0;

    for horizontal_pattern in patterns {
        let vertical_pattern = horizontal_pattern.transpose();

        // Search for the mirror over the vertical lines
        match vertical_pattern.mirror_point() {
            Some(point) => {
                answer += point + 1;
            },
            None => {
                match horizontal_pattern.mirror_point() {
                    Some(point) => {
                        answer += 100 * (point + 1);
                    },
                    None => panic!("Expected to find a mirror point!")
                }
            }
        }
    }

    println!("Problem 13 part 1: The sum of mirror points is {}", answer);

    answer
}

fn part_two(input: &Vec<String>) -> usize {
    let patterns = produce_patterns_from_input(input);
    
    let mut answer = 0;

    for horizontal_pattern in patterns {
        let vertical_pattern = horizontal_pattern.transpose();

        // Search for the mirror over the vertical lines
        match vertical_pattern.partial_mirror_point() {
            Some(point) => {
                answer += point + 1;
            },
            None => {
                match horizontal_pattern.partial_mirror_point() {
                    Some(point) => {
                        answer += 100 * (point + 1);
                    },
                    None => panic!("Expected to find a mirror point!")
                }
            }
        }
    }

    println!("Problem 13 part 1: The sum of mirror points is {}", answer);

    answer
}

fn produce_patterns_from_input(input: &Vec<String>) -> Vec<Pattern> {
    let mut patterns: Vec<Pattern> = Vec::new();
    let mut current_rows: Vec<String> = Vec::new();

    // Divide the input into Patterns
    for line in input {
        let line = line.trim();
        if !line.is_empty() {
            current_rows.push(String::from(line));
        } else if !current_rows.is_empty() {
            patterns.push(Pattern {
                rows: current_rows.clone()
            });
            current_rows.clear();
        }
    }

    if !current_rows.is_empty() {
        patterns.push(Pattern {
            rows: current_rows.clone()
        });
    }

    patterns
}

struct Pattern {
    rows: Vec<String>
}

impl Pattern {
    fn transpose(&self) -> Pattern {
        let columns: usize = self.rows.get(0).unwrap().len();

        // First create the vector of strings
        let mut column_strings: Vec<String> = Vec::new();
        for _ in 0..columns {
            column_strings.push(String::new());
        }

        // Second, iterate through the rows character by character and write
        // to the appropriate column
        for row in &self.rows {
            let mut cur_col = 0;
            for c in row.chars() {
                let col_str: &mut String = column_strings.get_mut(cur_col).unwrap();
                col_str.push(c);
                cur_col += 1;
            }
        }

        Pattern {
            rows: column_strings
        }
    }

    fn mirror_point(&self) -> Option<usize> {
        for bottom_index in 1..self.rows.len() {
            let top_index = bottom_index - 1;
            if self.valid_mirror(top_index, bottom_index) {
                return Some(top_index);
            }
        }

        None
    }

    fn partial_mirror_point(&self) -> Option<usize> {
        for bottom_index in 1..self.rows.len() {
            let top_index = bottom_index - 1;
            let is_valid: (bool, usize) = self.valid_partial_mirror(top_index, bottom_index);
            if is_valid.0 && is_valid.1 == 1 {
                return Some(top_index);
            }
        }

        None
    }

    /**
     * Will return true if there is a mirror at this index
     */
    fn valid_mirror(&self, top_index: usize, bottom_index: usize) -> bool {
        let top_row = self.rows.get(top_index).unwrap();
        let bottom_row = self.rows.get(bottom_index).unwrap();

        top_row.eq(bottom_row) && 
        (top_index == 0 || // Base case
            bottom_index == self.rows.len() - 1 || // Base case
            self.valid_mirror(top_index - 1, bottom_index + 1)) // Recursive iteration
    }

    fn valid_partial_mirror(&self, top_index: usize, bottom_index: usize) -> (bool, usize) {
        let mut mismatch: usize = 0;

        // Base case - Recurse until we hit the bottom. Slower, but easier to reason with at the moment
        if !(top_index == 0 || bottom_index == self.rows.len() - 1) {
            let result: (bool, usize) = self.valid_partial_mirror(top_index - 1, bottom_index + 1);

            // No need to continue processing. Just bubble up the failure
            if result.0 {
                mismatch += result.1;
            } else {
                return result;
            }
        }

        let top_row: &String = self.rows.get(top_index).unwrap();
        let bottom_row: &String = self.rows.get(bottom_index).unwrap();

        mismatch += utils::count_char_diff_between_strings(top_row, bottom_row);

        (mismatch <= 1, mismatch)
    }
}
