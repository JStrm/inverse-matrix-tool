extern crate core;

mod console_matrix_input;
mod gaussian_elimination;
mod inverse_matrix_finder;
mod matrix_manipulator;
mod pretty_matrix_printer;
mod rational_number;

fn main() {
    let mut matrix = console_matrix_input::get();

    if inverse_matrix_finder::invert(&mut matrix) {
        println!("Inverse matrix found:");
        pretty_matrix_printer::print(&matrix);
    } else {
        println!("Matrix in not regular, can't invert.");
    }
}
