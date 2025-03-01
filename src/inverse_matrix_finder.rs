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



    true
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
    let mut rational = true;






    rational
}

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