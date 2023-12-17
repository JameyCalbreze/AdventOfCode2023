use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::utils::{get_number_from_str, prep_input};

pub fn get_solution() {
    let input = prep_input("input/problem_03.txt");
    problem_three_part_one(&input);
}

fn problem_three_part_one(input: &Vec<String>) {
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
    }

    let mut running_sum = 0;
    for num_sym in collisions {
        running_sum += num_sym.value;
    }
    println!("The sum of the numbers next to symbols: {}", running_sum);
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Coordinate {
    row: isize,
    column: isize
}

#[derive(Clone, Hash, Eq, PartialEq)]
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

#[derive(Clone, Hash, Eq, PartialEq)]
struct Symbol {
    id_cord: Coordinate,
    sym: char
}

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

        // We know this has to be valid at thi point
        let position = self.possition_of_coordinate(c);

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
    fn possition_of_coordinate(&self, c: &Coordinate) -> isize {
        (c.row * isize::try_from(self.columns).unwrap()) + c.column
    }

    fn register_number(&mut self, num: NumSym) {
        // We have to generate all coordinates for this number when we register it...

        // First translate the coordinate from 2D to 1D
        let coord_position = self.possition_of_coordinate(&num.id_cord);

        // For each character we register a reference to the number in the map
        for i in 0..(num.len) {
            // For all characters in the number we're going to register the same number
            let value = self.data.get_mut(usize::try_from(coord_position).unwrap() + i).unwrap();
            // This will replace the default None value with Some(num). This is aparently 
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
        
        for r in -1..1 {
            for c in -1..1 {
                coords_to_check.push(Coordinate {
                    row: coordinate.row + r,
                    column: coordinate.column + c
                });
            }
        }

        for c in coords_to_check {
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
            numbers.push(NumSym::create_from_string(&cur_number, row.clone(), column.clone() - cur_number.len()));
            cur_number.clear();
        }
        column += 1
    }

    // There may be a number at the end of the row
    if !cur_number.is_empty() {
        numbers.push(NumSym::create_from_string(&cur_number, row.clone(), column.clone() - cur_number.len()));
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

