mod problems;
pub mod utils;
use std::{io, string};
use std::io::Write;

fn main() {
    println!("Hello, world!");

    let mut a: [u32; 10] = [0; 10];
    a[0] = 100;
    a[5] = 15;

    // print!("Which fibonacci number do you want: ");
    // io::stdout().flush().unwrap();
    //
    // // I want to read a line from the stdin
    // let mut s = String::new();
    // let num_chars = io::stdin()
    //     .read_line(&mut s)
    //     .expect("Not what we expected!");

    // How would I convert this to an integer?
    // Looks like the \n is still at the end of the line.
    // let b: usize = s.trim().parse().expect("Integer input was expected!");
    //
    // println!("The {} fibonacci number is {}", b, fibonacci(b));

    problems::p01::get_solution();
    problems::p02::get_solution();
    problems::p03::get_solution();
    problems::p04::get_solution();
    problems::p06::get_solution();
    problems::p07::get_solution();
}

fn fibonacci(number: usize) -> usize {
    if number == 0 {
        return 0;
    }

    let mut a: usize = 1; // Each iteration would be C = A + B
    let mut b: usize = 0; // Then A = B
    let mut c: usize = 1; // Then B = C

    for i in 0..number {
        c = a + b;
        a = b;
        b = c;
    }

    a
}
