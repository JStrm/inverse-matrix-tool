use crate::gaussian_elimination;
use crate::rational_number::RatNum;

pub fn invert(matrix: &mut Vec<Vec<RatNum>>) -> bool {

    // Test jestli matice splňuje potřebné podmínky, není prázndá apod.
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

    // Matice je zprava rozšířena o jednotkovou matici.
    // Tato rozšířená matice je uložena do nové proměnné.
    let mut wide_matrix = put_two_matrices_side_by_side(matrix, &unit_matrix(first_row_len));

    // Na rozšířenou matici je použita gausova eliminační metoda.
    // Tato funkce se pokusí z levé poloviny matice udělat jednotkovou matici.
    let success = gaussian_elimination::eliminate(&mut wide_matrix);

    // Pokud byla gausova eliminační metoda úspěšná, vstupní matice je přepsána na svou inverzní matici
    if success {
        // Pravá strana rozšířené matice je inverzní matice
        let inverse_matrix = right_half_of_matrix(&wide_matrix);
        copy_matrix_into_another_matrix(&inverse_matrix, matrix);
    }

    // `success` je boolean, který oznamuje jestli byla eliminace úspěšna, tedy jestli
    // matice byla regulární.
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

fn put_two_matrices_side_by_side(
    left: &Vec<Vec<RatNum>>,
    right: &Vec<Vec<RatNum>>,
) -> Vec<Vec<RatNum>> {
    if left.len() != right.len() {
        panic!("Matrices of different sizes");
    }

    let mut result = left.clone();

    for (a, row) in right.iter().enumerate() {
        result[a].append(&mut row.clone());
    }

    result
}

fn right_half_of_matrix(matrix: &Vec<Vec<RatNum>>) -> Vec<Vec<RatNum>> {
    let mut right_half: Vec<Vec<RatNum>> = Vec::new();
    let n = matrix.len();

    for row in matrix.iter() {
        right_half.push(row[n..].to_vec());
    }

    right_half
}

fn copy_matrix_into_another_matrix(source: &Vec<Vec<RatNum>>, target: &mut Vec<Vec<RatNum>>) {
    for a in 0..target.len() {
        target[a] = source[a].clone();
    }
}
