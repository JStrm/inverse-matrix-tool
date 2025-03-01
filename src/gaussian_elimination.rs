use crate::matrix_manipulator;
use crate::rational_number::RatNum;

// Eliminates so that there is a unit matrix on the left side
pub fn eliminate(matrix: &mut Vec<Vec<RatNum>>) -> bool {
    let size = matrix.len();

    for a in 0..size {
        if !make_diagonal_non_zero(matrix, a) {
            return false;
        }

        multiply_row_to_eq_one(matrix, a);

        subtract_elements_below_to_zero(matrix, a);
    }

    for a in 0..size {
        subtract_elements_above_to_zero(matrix, a);
    }

    true
}

fn make_diagonal_non_zero(matrix: &mut Vec<Vec<RatNum>>, diagonal_index: usize) -> bool {
    if !(matrix[diagonal_index][diagonal_index].equals(&RatNum::from_int(0))) {
        return true;
    }

    for b in diagonal_index..matrix.len() {
        if matrix[b][diagonal_index].equals(&RatNum::from_int(0)) {
            matrix_manipulator::add_rows(matrix, b, diagonal_index);
            return true;
        }
    }

    false
}

fn multiply_row_to_eq_one(matrix: &mut Vec<Vec<RatNum>>, diagonal_index: usize) {
    let inverse_fraction = matrix[diagonal_index][diagonal_index].inverse();
    matrix_manipulator::multiply_row(matrix, diagonal_index, &inverse_fraction);
}

fn subtract_elements_below_to_zero(matrix: &mut Vec<Vec<RatNum>>, diagonal_index: usize) {
    for c in diagonal_index + 1..matrix.len() {
        let value = matrix[c][diagonal_index].clone();
        let multiplier = value.multiply(&RatNum::from_int(-1));

        if multiplier.equals(&RatNum::from_int(0)) {
            continue;
        }

        matrix_manipulator::multiply_row(matrix, diagonal_index, &multiplier);
        matrix_manipulator::add_rows(matrix, diagonal_index, c);
        matrix_manipulator::multiply_row(matrix, diagonal_index, &multiplier.inverse());
    }
}

fn subtract_elements_above_to_zero(matrix: &mut Vec<Vec<RatNum>>, diagonal_index: usize) {
    for c in 0..diagonal_index {
        let value = matrix[c][diagonal_index].clone();
        let multiplier = value.multiply(&RatNum::from_int(-1));

        if multiplier.equals(&RatNum::from_int(0)) {
            continue;
        }

        matrix_manipulator::multiply_row(matrix, diagonal_index, &multiplier);
        matrix_manipulator::add_rows(matrix, diagonal_index, c);
        matrix_manipulator::multiply_row(matrix, diagonal_index, &multiplier.inverse());
    }
}
