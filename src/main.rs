extern crate core;

use crate::rational_number::RatNum;

mod gaussian_elimination;
mod inverse_matrix_finder;
mod matrix_manipulator;
mod rational_number;
mod pretty_matrix_printer;
mod console_matrix_input;

fn main() {
    // let mut matrix = console_matrix_input::get();

    let mut matrix = vec![
        vec![
            RatNum::from_int(1),
            RatNum::from_int(2),
            RatNum::from_int(3),
        ],
        vec![
            RatNum::from_int(4),
            RatNum::from_int(5),
            RatNum::from_int(6),
        ],
        vec![
            RatNum::from_int(7),
            RatNum::from_int(8),
            RatNum::from_int(9),
        ],
    ];

    // TODO: debug this weird behaviour

    inverse_matrix_finder::invert(&mut matrix);

    pretty_matrix_printer::print(&matrix);
}
