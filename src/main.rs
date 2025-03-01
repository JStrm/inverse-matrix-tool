use crate::rational_number::RatNum;

mod rational_number;
mod matrix_manipulator;

fn main() {
    let mut matrix = vec![
        vec![RatNum::from_int(1), RatNum::from_int(1)],
        vec![RatNum::from_int(1), RatNum::from_int(1)]
    ];

    matrix[0][0] = matrix[0][0].add(&RatNum::new(2, 4));


    println!("{:?}", matrix);
}
