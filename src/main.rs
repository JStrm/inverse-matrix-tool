use crate::rational_number::RatNum;

mod rational_number;
mod matrix_manipulator;
mod inverse_matrix_finder;

fn main() {
    let mut matrix = vec![
        vec![RatNum::from_int(2), RatNum::from_int(0)],
        vec![RatNum::from_int(0), RatNum::from_int(2)]
    ];

    inverse_matrix_finder::invert(&mut matrix);


    println!("{:?}", matrix);
}
