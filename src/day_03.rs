#[derive(Default)]
struct Matrix {
    data: Vec<String>,
    n_rows: usize,
    n_cols: usize,
}

impl Matrix {
    pub fn add_line(self: &mut Matrix, line: String) {
        self.n_rows += 1;
        if self.n_cols == 0 {
            self.n_cols = line.len();
        } else {
            assert_eq!(self.n_cols, line.len());
        }
        self.data.push(line);
    }

    pub fn get_value(self: &Matrix, i: usize, j: usize) -> char {
        return self.data[i].as_bytes()[j] as char;
    }

    pub fn rows(self: &Matrix) -> usize {
        self.n_rows
    }

    pub fn cols(self: &Matrix) -> usize {
        self.n_cols
    }

    pub fn read_from_file(file_name: &str) -> Matrix {
        let mut result = Matrix::default();
        for line in std::fs::read_to_string(file_name).unwrap().lines() {
            result.add_line(line.to_owned());
        }
        result
    }
}

// Scans left starting from the current position and tries to find a number
// that is strickly to the left of it. Returns 0 if not found (meaning that
// there is a non-number character to the left of i, j.
// Will return 0 in case of 12.*.. (where * is a starting position)
// Will return 12 in case of .12*..
fn scan_left(matrix: &Matrix, i: usize, j: usize) -> u64 {
    let mut current_col: i32 = j as i32 - 1;
    let mut number: u64 = 0;
    let mut pow = 1;
    loop {
        if current_col < 0 {
            break;
        }
        let c = matrix.get_value(i, current_col as usize).to_digit(10);
        match c {
            Some(value) => {
                number += value as u64 * pow;
                pow *= 10;
            }
            None => break,
        }
        current_col -= 1;
    }
    return number;
}

// Scans right starting from the current position and tries to find a number
// that is strickly to the right of it. Returns 0 if not found (meaning that
// there is a non-number character to the right of i, j
fn scan_right(matrix: &Matrix, i: usize, j: usize) -> u64 {
    let mut current_col: i32 = j as i32 + 1;
    let mut number: u64 = 0;
    loop {
        if current_col >= matrix.cols() as i32 {
            break;
        }
        let c = matrix.get_value(i, current_col as usize).to_digit(10);
        match c {
            Some(value) => {
                number *= 10;
                number += value as u64;
            }
            None => break,
        }
        current_col += 1;
    }
    return number;
}

// Scans a line in both directions starting from a given position and returns
// a vector of numbers adjacent to (sitting next to or overlapping with the
// current position)
fn scan_line(matrix: &Matrix, i: usize, j: usize) -> Vec<u64> {
    let c = matrix.get_value(i, j);
    let mut result: Vec<u64> = Vec::new();
    if c == '.' {
        let left = scan_left(matrix, i, j);
        if left > 0 {
            result.push(left);
        }
        let right = scan_right(matrix, i, j);
        if right > 0 {
            result.push(right);
        }
    } else if c.is_digit(10) {
        let mut current_col = j;
        // scan left and find number beginning
        loop {
            let c = matrix.get_value(i, current_col);
            if !c.is_digit(10) {
                current_col += 1;
                break;
            }
            if current_col == 0 {
                break;
            }
            current_col -= 1;
        }

        // now scan left to right and get the number
        let mut number: u64 = 0;
        loop {
            let c = matrix.get_value(i, current_col);
            if !c.is_digit(10) || current_col >= matrix.cols() {
                break;
            }
            number *= 10;
            number += c.to_digit(10).unwrap() as u64;
            current_col += 1;
        }
        result.push(number);
    }
    result
}

// Gets a vector of numbers adjacent to the specified i, j
fn get_adjacent_numbers(matrix: &Matrix, i: usize, j: usize) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();

    // scan left
    if j > 0 && matrix.get_value(i, j - 1).is_digit(10) {
        let value = scan_left(matrix, i, j);
        if value > 0 {
            result.push(value);
        }
    }

    // scan right
    if j < matrix.cols() - 1 && matrix.get_value(i, j + 1).is_digit(10) {
        let value = scan_right(matrix, i, j);
        if value > 0 {
            result.push(value);
        }
    }

    // scan top
    if i > 0 {
        let numbers = scan_line(matrix, i - 1, j);
        result.extend(numbers);
    }

    // scan bottom
    if i < matrix.rows() - 1 {
        let numbers = scan_line(matrix, i + 1, j);
        result.extend(numbers);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_line_test() {
        let mut matrix = Matrix::default();
        matrix.add_line("123".to_owned());
        matrix.add_line("456".to_owned());
        matrix.add_line("789".to_owned());
        matrix.add_line("*.,".to_owned());

        assert_eq!(matrix.n_cols, 3);
        assert_eq!(matrix.n_rows, 4);

        assert_eq!('1', matrix.get_value(0, 0));
        assert_eq!('5', matrix.get_value(1, 1));
        assert_eq!('8', matrix.get_value(2, 1));
        assert_eq!(',', matrix.get_value(3, 2));
    }

    #[test]
    fn get_adjacent_numbers_test() {
        {
            let mut matrix = Matrix::default();
            matrix.add_line("134..45".to_owned());
            matrix.add_line("1.3*.45".to_owned());
            matrix.add_line("7...89.".to_owned());

            let numbers = get_adjacent_numbers(&matrix, 1, 3);
            assert_eq!(3, numbers.len());
            assert!(numbers.contains(&134));
            assert!(numbers.contains(&3));
            assert!(numbers.contains(&89));
        }
        {
            let mut matrix = Matrix::default();
            matrix.add_line("134..45".to_owned());
            matrix.add_line("12.*1.5".to_owned());
            matrix.add_line("7...89.".to_owned());

            let numbers = get_adjacent_numbers(&matrix, 1, 3);
            assert_eq!(3, numbers.len());
            assert!(numbers.contains(&134));
            assert!(numbers.contains(&1));
            assert!(numbers.contains(&89));
        }
        {
            let mut matrix = Matrix::default();
            matrix.add_line("134..45".to_owned());
            matrix.add_line("124*195".to_owned());
            matrix.add_line("7...89.".to_owned());

            let numbers = get_adjacent_numbers(&matrix, 1, 3);
            assert_eq!(4, numbers.len());
            assert!(numbers.contains(&134));
            assert!(numbers.contains(&124));
            assert!(numbers.contains(&195));
            assert!(numbers.contains(&89));
        }
        {
            let mut matrix = Matrix::default();
            matrix.add_line("13...45".to_owned());
            matrix.add_line("12.*.95".to_owned());
            matrix.add_line("7....9.".to_owned());

            let numbers = get_adjacent_numbers(&matrix, 1, 3);
            assert_eq!(0, numbers.len());
        }
        {
            let mut matrix = Matrix::default();
            matrix.add_line(".2375..".to_owned());
            matrix.add_line("12.*.95".to_owned());
            matrix.add_line("...1...".to_owned());

            let numbers = get_adjacent_numbers(&matrix, 1, 3);
            println!("{:?}", numbers);
            assert_eq!(2, numbers.len());
            assert!(numbers.contains(&2375));
            assert!(numbers.contains(&1));
        }
    }
}

pub mod part1 {
    use super::*;

    pub fn solve(file_name: &str) -> u64 {
        let mut result: u64 = 0;
        let matrix = Matrix::read_from_file(file_name);

        for i in 0..matrix.rows() {
            for j in 0..matrix.cols() {
                let c = matrix.get_value(i, j);
                if c.is_digit(10) || c == '.' {
                    continue;
                }
                let numbers = get_adjacent_numbers(&matrix, i, j);
                for num in numbers {
                    result += num;
                }
            }
        }

        result
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file_name: &str) -> u64 {
        let mut result: u64 = 0;
        let matrix = Matrix::read_from_file(file_name);

        for i in 0..matrix.rows() {
            for j in 0..matrix.cols() {
                let c = matrix.get_value(i, j);
                if c != '*' {
                    continue;
                }
                let numbers = get_adjacent_numbers(&matrix, i, j);
                if numbers.len() != 2 {
                    continue;
                }
                result += numbers[0] * numbers[1];
            }
        }

        result
    }
}
