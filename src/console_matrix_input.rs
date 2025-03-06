use crate::rational_number::RatNum;
use std::io;

pub fn get() -> Vec<Vec<RatNum>> {
    println!("Enter size of square matrix: ");
    // Vstup velikosti matice
    let n = read_number_from_input();

    println!("Enter each integer element of matrix.");
    println!("Separate number on a row by spaces");
    println!("         and row by newlines.");

    // Vstup prvků matice
    // předevede data z řádků z konzole do objectu Vec<Vec<RatNum>>
    // `Vec` je ekvivalent listu z pythonu.
    let matrix = read_matrix_from_input(n);

    matrix
}

fn read_number_from_input() -> usize {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
        .trim()
        .parse()
        .expect("Please enter a valid positive matrix size")
}

fn read_matrix_from_input(n: usize) -> Vec<Vec<RatNum>> {
    let mut matrix: Vec<Vec<RatNum>> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut row_input = String::new();
        io::stdin()
            .read_line(&mut row_input)
            .expect("Failed to read line");

        let int_row: Vec<isize> = row_input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid integer input"))
            .collect();

        if int_row.len() != n {
            panic!(
                "Invalid number of elements. Expected {}, got {}.",
                n,
                int_row.len()
            );
        }

        let mut ratnum_row: Vec<RatNum> = Vec::with_capacity(n);

        for integer in int_row.iter() {
            ratnum_row.push(RatNum::from_int(*integer));
        }

        matrix.push(ratnum_row);
    }

    matrix
}
