extern crate core;

fn main() {
    // Uživatelský vstup z konzole
    let mut matrix = console_matrix_input::get();

    // inverse_matrix_finder upraví matici do své inverzní matice.
    // Upravuje ji přímo, hodnota co vrací je boolean. Pokud je true, inverze matice
    // byla úspěšná. Pokud false, nastala chyba a matice není regulární.
    if inverse_matrix_finder::invert(&mut matrix) {
        println!("Inverzní matice:");

        // Kód na výpis matice do konzole
        pretty_matrix_printer::print(&matrix);
    } else {
        println!("Matice není regulární, nelze ji invertovat.");
    }
}

mod rational_number {
    // Implementace racionálního čísla
    // Po každé operaci se zlomek převede do svého základního tvaru (`simplify`)
    #[derive(Debug)]
    pub struct RatNum {
        n: isize, // numerator
        d: isize, // denominator
    }

    impl RatNum {
        pub fn new(numerator: isize, denominator: isize) -> Self {
            if denominator == 0 {
                panic!("Denominator cannot be zero!");
            }
            let mut result = RatNum {
                n: numerator,
                d: denominator,
            };

            result.simplify();

            result
        }

        pub fn from_int(int: isize) -> Self {
            RatNum { n: int, d: 1 }
        }

        fn clone(&self) -> Self {
            RatNum {
                n: self.n,
                d: self.d,
            }
        }

        pub fn add(&self, other: &RatNum) -> RatNum {
            let new_n = self.n * other.d + other.n * self.d;
            let new_d = self.d * other.d;
            let mut result = RatNum::new(new_n, new_d);
            result.simplify();
            result
        }

        pub fn multiply(&self, other: &RatNum) -> RatNum {
            let new_n = self.n * other.n;
            let new_d = self.d * other.d;
            let mut result = RatNum::new(new_n, new_d);
            result.simplify();
            result
        }

        pub fn inverse(&self) -> RatNum {
            let mut result = RatNum::new(self.d, self.n);
            result.simplify();
            result
        }

        pub fn equals(&self, other: &RatNum) -> bool {
            (self.n == other.n) && (self.d == other.d)
        }
        pub fn to_string(&self) -> String {
            if self.d == 1 {
                return format!("{}", self.n);
            }
            format!("{}/{}", self.n, self.d)
        }

        fn simplify(&mut self) {
            let gcd = self.greatest_common_divisor(self.n.abs(), self.d.abs());
            self.n /= gcd;
            self.d /= gcd;

            // Move minus sign to numerator
            if self.d < 0 {
                self.n *= -1;
                self.d *= -1;
            }

            // For consistency sake, make all zeroes the same
            if self.n == 0 {
                self.d = 1;
            }
        }
        fn greatest_common_divisor(&self, a: isize, b: isize) -> isize {
            let mut x = a;
            let mut y = b;

            while y != 0 {
                let temp = y;
                y = x % y;
                x = temp;
            }

            x
        }
    }

    impl Clone for RatNum {
        fn clone(&self) -> Self {
            self.clone()
        }
    }
}

mod pretty_matrix_printer {
    use crate::rational_number::RatNum;

    // Tento kód vypíše matici. Vypíše jí jako zlomky. Zarovná prvky do sloupců.
    pub fn print(matrix: &Vec<Vec<RatNum>>) {
        let mut lines = Vec::<String>::new();

        for _ in 0..(matrix.len() * 2) {
            lines.push(String::new());
        }

        for column in 0..matrix.len() {
            let mut widest_number = 0;
            for row in 0..matrix.len() {
                let number_str = matrix[row][column].to_string();
                lines[row * 2].push_str(&number_str);

                if number_str.len() > widest_number {
                    widest_number = number_str.len();
                }
            }

            for row in 0..matrix.len() {
                let number_str = matrix[row][column].to_string();
                lines[row * 2].push_str(",  ");

                for _ in 0..=(widest_number - number_str.len()) {
                    lines[row * 2].push(' ');
                }
            }
        }

        for line in lines {
            println!("{}", line);
        }
    }
}
mod matrix_manipulator {
    use crate::rational_number::RatNum;

    // Základní metody na minupalaci s maticí, přičítání a násobení řádků.
    // Výměna řádků není potřeba

    pub fn add_rows(
        matrix: &mut Vec<Vec<RatNum>>,
        source_row_index: usize,
        target_row_index: usize,
    ) {
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
}

mod inverse_matrix_finder {
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
}

mod gaussian_elimination {
    use crate::matrix_manipulator;
    use crate::rational_number::RatNum;

    // Eliminates so that there is a unit matrix on the left side
    pub fn eliminate(matrix: &mut Vec<Vec<RatNum>>) -> bool {
        let size = matrix.len();

        // Tento loop zaručí, že na diagonále levé strany matice budou jendničky.
        for a in 0..size {
            // Tato funkce převede prvek na diagonále na nenulové číslo přičtením
            // libovolného řádku pod tímto prvkem na diagonále. Pokud žádný takový řádek neexistuje,
            // vrátí false (matice není regulérní)
            if !make_diagonal_non_zero(matrix, a) {
                return false;
            }

            // Tato funkce vydělí řádek tak, aby hodnota na diagonále byla `1`
            multiply_row_to_eq_one(matrix, a);

            // Tato funkce odečte řádek od řádků pod sebou tak, aby hodnoty pod hodnotou na
            // diagonále byly `0`
            subtract_elements_below_to_zero(matrix, a);
        }
        // Po tomto loopu je levá polovina matice v horním trojuhelníkovém tvaru, a všude
        // na diagonále jsou jedničky.

        // Tento loop udělá z levé strany matice jednotkovou matici. Tím je gausova
        // eliminace dokončena
        for a in 0..size {
            subtract_elements_above_to_zero(matrix, a);
        }

        // Vrátí `true`, protože proces eliminace byl úspěšný.
        true
    }

    fn make_diagonal_non_zero(matrix: &mut Vec<Vec<RatNum>>, diagonal_index: usize) -> bool {
        if !(matrix[diagonal_index][diagonal_index].equals(&RatNum::from_int(0))) {
            return true;
        }

        for b in diagonal_index..matrix.len() {
            if !(matrix[b][diagonal_index].equals(&RatNum::from_int(0))) {
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
}

mod console_matrix_input {
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
}
