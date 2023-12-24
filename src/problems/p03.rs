use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::utils::{get_number_from_str, prep_input};

pub fn get_solution() {
    let input = prep_input("input/problem_03.txt");
    problem_three_part_one(&input);
    problem_three_part_two(&input);
}

fn problem_three_part_one(input: &Vec<String>) -> u64 {
    // First we'll create the Grid
    let mut grid: Grid =
        Grid::create_grid(input.len(), input.get(0).unwrap().len());

    let mut row = 0;
    for line in input {
        for num_sym in get_numbers_from_string(line.as_str(), row.clone()) {
            grid.register_number(num_sym);
        }
        row += 1;
    }

    let mut collisions: HashSet<NumSym> = HashSet::new();
    row = 0;
    for line in input {
        for sym in get_symbols_from_string(line.as_str(), row.clone()) {
            let nums = grid.get_nums_near_coordinate(&sym.id_cord);
            for num in nums {
                collisions.insert(num);
            }
        }
        row += 1
    }

    let mut running_sum = 0;
    for num_sym in collisions {
        running_sum += num_sym.value;
    }
    println!("Problem 3 Part 1: The sum of the numbers next to symbols: {}", running_sum);

    running_sum
}

fn problem_three_part_two(input: &Vec<String>) -> u64 {
    let mut grid: Grid =
        Grid::create_grid(input.len(), input.get(0).unwrap().len());

    let mut row = 0;
    for line in input {
        for num_sym in get_numbers_from_string(line.as_str(), row.clone()) {
            grid.register_number(num_sym);
        }
        row += 1;
    }

    // Here is where things will differ slightly. We need to check how many numbers
    // are adjacent each time we find the correct symbol. Repeats are expected here
    let mut running_sum: u64 = 0;
    row = 0;
    for line in input {
        for sym in get_symbols_from_string(line.as_str(), row.clone()) {
            if !(sym.sym == '*') { continue; } // We only care about stars

            let nums = grid.get_nums_near_coordinate(&sym.id_cord);
            if !(nums.len() == 2) { continue; } // We only care about pairs

            let mut gear_ratio: u64 = 1;
            for num in nums {
                gear_ratio *= num.value;
            }
            running_sum += gear_ratio;
        }
        row += 1
    }

    println!("Problem 3 Part 2: The sum of gear ratios is {}", running_sum);

    running_sum
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Coordinate {
    row: isize,
    column: isize
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct NumSym {
    id_cord: Coordinate,
    len: usize,
    value: u64
}

impl NumSym {
    fn create_from_string(num: &str, row: usize, column: usize) -> NumSym {
        // How long is the number string
        let num_len = num.len();
        
        // What is the value of the number
        let value = get_number_from_str(num);

        NumSym {
            id_cord: Coordinate {
                row: isize::try_from(row).unwrap(),
                column: isize::try_from(column).unwrap()
            },
            len: num_len,
            value
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Symbol {
    id_cord: Coordinate,
    sym: char
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Grid {
    rows: usize,
    columns: usize,
    data: Vec<Option<NumSym>>
}

impl Grid {
    fn create_grid(rows: usize, columns: usize) -> Grid {
        let mut data = Vec::with_capacity(rows * columns);

        // This is Rust and requires something be written into the vector
        for _ in 0..(rows * columns) {
            data.push(None);
        }

        Grid {
            rows,
            columns,
            data
        }
    }

    fn get_num_at_coord(&self, c: &Coordinate) -> Option<NumSym> {
        // Let's return early in the case the coordinate is out of bounds
        if !self.is_valid_coordinate(c) {
            return None;
        }

        // We know this has to be valid at this point
        let position = self.position_of_coordinate(c);

        // The position is valid, but now we need to see if we have a NumSym
        let possible: &Option<NumSym> = self.data.get(usize::try_from(position).unwrap()).unwrap();
        match possible {
            Some(num_sym) => Some(num_sym.clone()),
            None => None
        }
    }

    // Simple bounding box for Row
    fn is_valid_row(&self, row: isize) -> bool {
        !(row < 0 || row >= isize::try_from(self.rows).unwrap())
    }

    // Simple bounding box for Column
    fn is_valid_column(&self, column: isize) -> bool {
        !(column < 0 || column >= isize::try_from(self.columns).unwrap())
    }

    // Combine both bounding boxes
    fn is_valid_coordinate(&self, c: &Coordinate) -> bool {
        self.is_valid_row(c.row) && self.is_valid_column(c.column)
    }

    // This function will return the 1D position of a coordinate assuming
    // the coordinate is valid
    fn position_of_coordinate(&self, c: &Coordinate) -> isize {
        (c.row * isize::try_from(self.columns).unwrap()) + c.column
    }

    fn register_number(&mut self, num: NumSym) {
        // We have to generate all coordinates for this number when we register it...

        // First translate the coordinate from 2D to 1D
        let coord_position = usize::try_from(self.position_of_coordinate(&num.id_cord)).unwrap();

        // For each character we register a reference to the number in the map
        for i in 0..(num.len) {
            // For all characters in the number we're going to register the same number
            let value = self.data.get_mut(coord_position + i).unwrap();
            // This will replace the default None value with Some(num). This is apparently
            // standard for swapping values in place
            let _ = std::mem::replace(value, Some(num.clone()));
        }
    }

    // This will search the 3x3 matrix around the coordinate in question.
    // This will also check the coordinate, but we expect that to return nothing 100% of the time
    fn get_nums_near_coordinate(&self, coordinate: &Coordinate) -> HashSet<NumSym> {
        let mut num_set = HashSet::new();

        // We need the coordinates above below and diagonal to this position.
        // Converting the coordinates to isize makes this WAYYYY easier... Maybe... 
        // This will probably backfire

        let mut coords_to_check = Vec::new();

        // Off by one errors ruining my life
        for r in (-1)..2 {
            for c in (-1)..2 {
                coords_to_check.push(Coordinate {
                    row: coordinate.row + r,
                    column: coordinate.column + c
                });
            }
        }

        for c in coords_to_check {
            // println!("Checking coordinate row: {}, column: {}", c.row, c.column);
            match self.get_num_at_coord(&c) {
                Some(num) => { num_set.insert(num); },
                None => ()
            };
        }

        num_set
    }
}

fn get_numbers_from_string(line: &str, row: usize) -> Vec<NumSym> {
    let mut numbers: Vec<NumSym> = Vec::new();

    let mut column: usize = 0;
    let mut cur_number = String::new();
    for c in line.chars() {
        if c >= '0' && c <= '9' {
            cur_number.push(c);
        } else if !cur_number.is_empty() {
            numbers.push(
                NumSym::create_from_string(&cur_number, row, column - cur_number.len()));
            cur_number.clear();
        }
        column += 1
    }

    // There may be a number at the end of the row
    if !cur_number.is_empty() {
        numbers.push(
            NumSym::create_from_string(&cur_number, row, column - cur_number.len()));
    }

    numbers
}

fn get_symbols_from_string(line: &str, row: usize) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();

    let mut column:usize = 0;
    for c in line.chars() {
        if !((c >= '0' && c <= '9') || c == '.') {
            symbols.push(Symbol {
                id_cord: Coordinate { 
                    row: isize::try_from(row).unwrap(), 
                    column: isize::try_from(column).unwrap() 
                }, 
                sym: c.clone()
            });
        }
        column += 1
    }

    symbols
}

// Begin Tests

#[test]
fn basic_string_validations() {
    let mut test_input: Vec<String> = Vec::new();
    let test = String::from("123");
    test_input.push(test);
    assert_eq!(0, problem_three_part_one(&test_input));
}

#[test]
fn one_number_and_one_symbol() {
    let mut test_input: Vec<String> = Vec::new();
    test_input.push(String::from("123("));
    assert_eq!(123, problem_three_part_one(&test_input));
    test_input.clear();
    test_input.push(String::from("(123"));
    assert_eq!(123, problem_three_part_one(&test_input));
    test_input.clear();
    test_input.push(String::from("(.123"));
    assert_eq!(0, problem_three_part_one(&test_input));
}

#[test]
fn get_nums_near_symbol_one_number_one_symbol() {
    let mut grid = Grid::create_grid(1, 4);
    let test_num = NumSym {
        id_cord: Coordinate { row: 0, column: 1 },
        len: 3,
        value: 123
    };
    grid.register_number(test_num.clone());
    assert_eq!(test_num, grid.get_num_at_coord(&Coordinate { row: 0, column: 1}).unwrap());
    assert_eq!(test_num, grid.get_num_at_coord(&Coordinate { row: 0, column: 2}).unwrap());
    assert_eq!(test_num, grid.get_num_at_coord(&Coordinate { row: 0, column: 3}).unwrap());
}

#[test]
fn get_num_in_all_locations() {
    let mut grid = Grid::create_grid(3, 3);
    let test_num = NumSym {
        id_cord: Coordinate { row: 1, column: 1 },
        len: 1,
        value: 1
    };
    grid.register_number(test_num.clone());
    let mut expected_set = HashSet::new();
    expected_set.insert(test_num.clone());
    assert_eq!(expected_set, grid.get_nums_near_coordinate(&Coordinate { row: 0, column: 0}));
    assert_eq!(expected_set, grid.get_nums_near_coordinate(&Coordinate { row: 0, column: 1}));
    assert_eq!(expected_set, grid.get_nums_near_coordinate(&Coordinate { row: 0, column: 2}));
    assert_eq!(expected_set, grid.get_nums_near_coordinate(&Coordinate { row: 1, column: 0}));
    assert_eq!(expected_set, grid.get_nums_near_coordinate(&Coordinate { row: 1, column: 1}));
    assert_eq!(expected_set, grid.get_nums_near_coordinate(&Coordinate { row: 1, column: 2}));
    assert_eq!(expected_set, grid.get_nums_near_coordinate(&Coordinate { row: 2, column: 0}));
    assert_eq!(expected_set, grid.get_nums_near_coordinate(&Coordinate { row: 2, column: 1}));
    assert_eq!(expected_set, grid.get_nums_near_coordinate(&Coordinate { row: 2, column: 2}));
}

#[test]
fn get_symbols_from_string_one_symbol() {
    let mut expected_output = Vec::new();
    let expected_symbol = Symbol {
        id_cord: Coordinate {
            row: 0,
            column: 0
        },
        sym: 'v'
    };
    expected_output.push(expected_symbol);
    assert_eq!(expected_output, get_symbols_from_string("v123", 0));
}

#[test]
fn number_from_string_test() {
    let mut expected_output: Vec<NumSym> = Vec::new();
    expected_output.push(NumSym {
        id_cord: Coordinate {
            row: 0,
            column: 0
        },
        len: 3,
        value: 123
    });
    assert_eq!(expected_output, get_numbers_from_string("123", 0));
    assert_eq!(expected_output, get_numbers_from_string("123.", 0));
    assert_eq!(expected_output, get_numbers_from_string("123..", 0));
    assert_eq!(expected_output, get_numbers_from_string("123.(.", 0));
    assert_eq!(expected_output, get_numbers_from_string("123(.", 0));
}

#[test]
fn one_number_from_string_one_symbol() {
    let mut expected_output: Vec<NumSym> = Vec::new();
    expected_output.push(NumSym {
        id_cord: Coordinate {
            row: 0,
            column: 1
        },
        len: 3,
        value: 123
    });
    assert_eq!(expected_output, get_numbers_from_string(".123", 0));
    assert_eq!(expected_output, get_numbers_from_string("(123", 0));
    assert_eq!(expected_output, get_numbers_from_string("*123", 0));
}
