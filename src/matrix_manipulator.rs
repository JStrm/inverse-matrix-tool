use crate::rational_number::RatNum;

pub fn add_rows(matrix: &mut Vec<Vec<RatNum>>, source_row_index: usize, target_row_index: usize) {
    if source_row_index >= matrix.len() || target_row_index >= matrix.len() {
        panic!("Row index out of bounds");
    }

    if matrix[source_row_index].len() != matrix[target_row_index].len() {
        panic!("Rows must have the same length");
    }

    // clone the source row to avoid simultaneous mutable and immutable borrows
    let source_row_cloned = matrix[source_row_index].clone();

    for (source_element, target_element) in source_row_cloned
        .iter()
        .zip(matrix[target_row_index].iter_mut())
    {
        *target_element = target_element.add(source_element);
    }
}

pub fn multiply_row(matrix: &mut Vec<Vec<RatNum>>, row_index: usize, multiplier: &RatNum) {
    if row_index >= matrix.len() {
        panic!("Row index out of bounds");
    }

    for element in matrix[row_index].iter_mut() {
        *element = element.multiply(multiplier);
    }
}
