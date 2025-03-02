use crate::rational_number::RatNum;

pub fn print(matrix: &Vec<Vec<RatNum>>) {
    // TODO: Make this more readable

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
