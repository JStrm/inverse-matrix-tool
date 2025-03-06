use crate::rational_number::RatNum;
use std::io;

pub fn get() -> Vec<Vec<RatNum>> {
    println!("Zadejte rozměr čtvercové matice: ");
    // Vstup velikosti matice
    let n = read_number_from_input();

    println!("Zadejte všechny prvky matice.");
    println!("Zadávejte pouze celá čísla.");
    println!("Na řádku oddělujte čísla mezerami");
    println!("a řádky oddělujte enterem.");

    // Vstup prvků matice
    // předevede data z řádků z konzole do objectu Vec<Vec<RatNum>>
    // `Vec` je ekvivalent listu z pythonu.
    let matrix = read_matrix_from_input(n);

    matrix
}

fn read_number_from_input() -> usize {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Nelze přečíst řádek");

    input
        .trim()
        .parse()
        .expect("Velikost matice musí být přirozené číslo.")
}

fn read_matrix_from_input(n: usize) -> Vec<Vec<RatNum>> {
    let mut matrix: Vec<Vec<RatNum>> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut row_input = String::new();
        io::stdin()
            .read_line(&mut row_input)
            .expect("Nelze přečíst řádek");

        let int_row: Vec<isize> = row_input
            .split_whitespace()
            .map(|s| s.parse().expect("Vstup musí být celé číslo."))
            .collect();

        if int_row.len() != n {
            panic!(
                "Špatný počet prvků na řádku. Očekáváno: {}, vloženo: {}.",
                n,
                int_row.len()
            );
        }

        let mut ratnum_row: Vec<RatNum> = Vec::with_capacity(n);

        for integer in int_row.iter() {
            ratnum_row.push(RatNum::from_int(*integer));
        }

        matrix.push(ratnum_row);
    }

    matrix
}
