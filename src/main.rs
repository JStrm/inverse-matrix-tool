use crate::rational_number::RatNum;

mod rational_number;
mod matrix_manipulator;

fn main() {
    let mut matrix = vec![
        vec![RatNum::from_int(0), RatNum::from_int(1)],
        vec![RatNum::from_int(2), RatNum::from_int(1)]
    ];

    matrix_manipulator::add_rows(&mut matrix, 1,0);

    matrix[0][0] = matrix[0][0].add(&RatNum::new(2, 4));

    let inverse = matrix[0][0].inverse();

    matrix_manipulator::multiply_row(&mut matrix, 0, &inverse);


    println!("{:?}", matrix);
}
