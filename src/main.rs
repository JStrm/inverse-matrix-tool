use crate::rational_number::RatNum;

mod gaussian_elimination;
mod inverse_matrix_finder;
mod matrix_manipulator;
mod rational_number;
mod pretty_matrix_printer;

fn main() {
    // TODO: Implement user input
    let mut matrix = vec![
        vec![
            RatNum::from_int(7),
            RatNum::from_int(2),
            RatNum::from_int(1),
        ],
        vec![
            RatNum::from_int(0),
            RatNum::from_int(3),
            RatNum::from_int(-1),
        ],
        vec![
            RatNum::from_int(-3),
            RatNum::from_int(4),
            RatNum::from_int(-2),
        ],
    ];

    inverse_matrix_finder::invert(&mut matrix);

    pretty_matrix_printer::print(&matrix);
}
