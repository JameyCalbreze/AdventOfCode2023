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

    let collisions: HashSet<NumSym> = HashSet::new();
    row = 0;
    for line in input {
        for sym in get_symbols_from_string(line.as_str(), row.clone()) {

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
    row: usize,
    column: usize
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct NumSym {
    id_cord: Coordinate,
    len: usize,
    value: u64
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Symbol {
    id_cord: Coordinate,
    sym: char
}

struct Grid {
    rows: usize,
    columns: usize,
    data: Vec<NumSym>
}

impl Grid {
    fn create_grid(rows: usize, columns: usize) -> Grid {
        Grid {
            rows: rows.clone(),
            columns: columns.clone(),
            data: Vec::with_capacity(rows * columns)
        }
    }

    fn register_number(&mut self, num: NumSym) {
        // We have to generate all coordinates for this number when we register it...

        let row_adjustment = num.id_cord.row * self.columns;
        for i in 0..num.len {
            self.data.insert(row_adjustment + num.id_cord.column + i, num.clone());
        }

        self.num_map.insert(num.id_cord.clone(), num);
    }

    fn get_nums_near_coordinate(coordinate: Coordinate) {

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
            numbers.push(NumSym {
                id_cord: Coordinate {
                    row: row.clone(),
                    column: column.clone() - cur_number.len()
                },
                len: cur_number.len().clone(),
                value: get_number_from_str(&cur_number[..])
            });
            cur_number.clear();
        }
        column += 1
    }

    numbers
}

fn get_symbols_from_string(line: &str, row: usize) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();

    let mut column:usize = 0;
    for c in line.chars() {
        if !((c >= '0' && c <= '9') || c == '.') {
            symbols.push(Symbol {
                id_cord: Coordinate { row: row.clone(), column: column.clone() }, sym: c.clone()});
        }
        column += 1
    }

    symbols
}

