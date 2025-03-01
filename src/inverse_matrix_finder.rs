use crate::matrix_manipulator;
use crate::rational_number::RatNum;

pub fn invert(matrix: &mut Vec<Vec<RatNum>>) -> bool {
    if matrix.len() == 0 {
        return false;
    }
    let first_row_len = matrix[0].len();
    if first_row_len != matrix.len() {
        panic!("Matrix is not square");
    }
    for row in matrix.iter() {
        if row.len() != first_row_len {
            panic!("Not a matrix, uneven");
        }
    }


    let mut wide_matrix = put_two_matrices_side_by_side(matrix, &unit_matrix(first_row_len));

    let success = gauss_eliminate(&mut wide_matrix);

    let inverse_matrix = right_half_of_matrix(&wide_matrix);

    if success {
        copy_matrix_into_another_matrix(&inverse_matrix, matrix);
    }

    success
}

fn unit_matrix(size: usize) -> Vec<Vec<RatNum>> {
    let mut result: Vec<Vec<RatNum>>;
    result = Vec::new();

    for a in 0..size {
        result.push(Vec::new());
        for b in 0..size {
            if a == b {
                result[a].push(RatNum::from_int(1));
            } else {
                result[a].push(RatNum::from_int(0));
            }
        }
    }

    result
}

fn put_two_matrices_side_by_side(left: &Vec<Vec<RatNum>>, right: &Vec<Vec<RatNum>>) -> Vec<Vec<RatNum>> {
    if left.len() != right.len() {
        panic!("Matrices of different sizes");
    }

    let mut result = left.clone();

    for (a, row) in right.iter().enumerate() {
        result[a].append(&mut row.clone());
    }

    result
}

fn gauss_eliminate(matrix: &mut Vec<Vec<RatNum>>) -> bool {
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
    if !(matrix[diagonal_index][diagonal_index].equals(&RatNum::from_int(0))){
        return true;
    }

    for b in diagonal_index..matrix.len() {
        if matrix[b][diagonal_index].equals(&RatNum::from_int(0)){
            matrix_manipulator::add_rows(matrix, b, diagonal_index);
            return true
        }
    }

    false
}

fn multiply_row_to_eq_one(matrix: &mut Vec<Vec<RatNum>>, diagonal_index: usize) {
    let inverse_fraction = matrix[diagonal_index][diagonal_index].inverse();
    matrix_manipulator::multiply_row(matrix, diagonal_index, &inverse_fraction);
}

fn subtract_elements_below_to_zero(matrix: &mut Vec<Vec<RatNum>>, diagonal_index: usize) {
    for c in diagonal_index+1..matrix.len() {
        let value = matrix[c][diagonal_index].clone();
        let multiplier = value.multiply(&RatNum::from_int(-1));

        if multiplier.equals(&RatNum::from_int(0)){
            continue;
        }

        matrix_manipulator::multiply_row(matrix, diagonal_index, &multiplier);
        matrix_manipulator::add_rows(matrix, diagonal_index, c);
        matrix_manipulator::multiply_row(matrix, diagonal_index, &multiplier.inverse());
    }
}

fn subtract_elements_above_to_zero(matrix: &mut Vec<Vec<RatNum>>, diagonal_index: usize) {

    // TODO: is this correct?
    for c in 0..diagonal_index {
        let value = matrix[c][diagonal_index].clone();
        let multiplier = value.multiply(&RatNum::from_int(-1));

        if multiplier.equals(&RatNum::from_int(0)){
            continue;
        }

        matrix_manipulator::multiply_row(matrix, diagonal_index, &multiplier);
        matrix_manipulator::add_rows(matrix, diagonal_index, c);
        matrix_manipulator::multiply_row(matrix, diagonal_index, &multiplier.inverse());
    }
}



// Máme matici 2n x n
//
// x x 1 0
// x x 0 1
//
// Opakuje pro 1..n:
// - Podivame se na n-ty radek, n-ty sloupec
//     - Pokud != 0, tak pohoda
//     - Pokud == 0, pricteme k nemu jiny radek (co je pod nim).
//         - Pokud takovy neex., neni to v poho matice
// - vydelime radek tak, aby byla 1
// - od kazdeho radku po nim i nad nim odecteme tolik, aby bylo 0 0 0 ... 1 ... 0 0 0
// - posuneme se na dalsi radek

fn right_half_of_matrix(matrix: &Vec<Vec<RatNum>>) -> Vec<Vec<RatNum>> {
    let mut right_half: Vec<Vec<RatNum>> = Vec::new();
    let n = matrix.len();

    for (i, row) in matrix.iter().enumerate() {
        right_half.push(row[n..].to_vec());
    }

    right_half
}

fn copy_matrix_into_another_matrix(source: &Vec<Vec<RatNum>>, target: &mut Vec<Vec<RatNum>>) {
    // TODO: Copy the rows directly
    for a in 0..target.len() {
        for b in 0..target.len() {
            target[a][b] = source[a][b].clone();
        }
    }
}