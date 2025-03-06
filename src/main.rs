extern crate core;

mod console_matrix_input;
mod gaussian_elimination;
mod inverse_matrix_finder;
mod matrix_manipulator;
mod pretty_matrix_printer;
mod rational_number;

fn main() {
    // Uživatelský vstup z konzole
    let mut matrix = console_matrix_input::get();

    // inverse_matrix_finder upraví matici do své inverzní matice.
    // Upravuje ji přímo, hodnota co vrací je boolean. Pokud je true, inverze matice
    // byla úspěšná. Pokud false, nastala chyba a matice není regulární.
    if inverse_matrix_finder::invert(&mut matrix) {
        println!("Inverse matrix found:");

        // Kód na výpis matice do konzole
        pretty_matrix_printer::print(&matrix);
    } else {
        println!("Matrix in not regular, can't invert.");
    }
}
