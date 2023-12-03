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
}

pub mod part1 {
    use super::*;

    fn value_if_has_adjacent_cymbols(
        matrix: &Matrix,
        row: i32,
        col_start: i32,
        col_end: i32,
    ) -> u64 {
        let mut neighbors: Vec<(i32, i32)> = Vec::new();

        if row > 0 {
            for i in col_start - 1..=col_end + 1 {
                if i >= 0 && i < matrix.cols() as i32 {
                    neighbors.push((row - 1, i));
                }
            }
        }
        if col_start > 0 {
            neighbors.push((row, col_start - 1));
        }
        if col_end < matrix.cols() as i32 - 1 {
            neighbors.push((row, col_end + 1));
        }
        if row < matrix.rows() as i32 - 1 {
            for i in col_start - 1..=col_end + 1 {
                if i >= 0 && i < matrix.cols() as i32 {
                    neighbors.push((row + 1, i));
                }
            }
        }

        for n in neighbors {
            let c = matrix.get_value(n.0 as usize, n.1 as usize);
            if c.is_digit(10) || c == '.' {
                continue;
            }
            let mut result = 0;
            for i in col_start..=col_end {
                result *= 10;
                result += matrix
                    .get_value(row as usize, i as usize)
                    .to_digit(10)
                    .unwrap();
            }
            return result as u64;
        }
        return 0;
    }

    pub fn solve(file_name: &str) -> u64 {
        let mut result: u64 = 0;
        let matrix = Matrix::read_from_file(file_name);

        let mut start: usize = 0;

        for i in 0..matrix.rows() {
            let mut scanning_number = false;
            for j in 0..matrix.cols() {
                let c = matrix.get_value(i, j);
                if scanning_number {
                    if !c.is_digit(10) {
                        result += value_if_has_adjacent_cymbols(
                            &matrix,
                            i as i32,
                            start as i32,
                            j as i32 - 1,
                        );
                        scanning_number = false;
                    } else if j == matrix.cols() - 1 {
                        result += value_if_has_adjacent_cymbols(
                            &matrix,
                            i as i32,
                            start as i32,
                            j as i32,
                        );
                        scanning_number = false;
                    }
                } else {
                    if c.is_digit(10) {
                        start = j;
                        scanning_number = true;
                    }
                }
            }
        }
        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn has_adjacent_cymbols_test() {
            {
                let mut matrix = Matrix::default();
                matrix.add_line(".....".to_owned());
                matrix.add_line(".123.".to_owned());
                matrix.add_line(".....".to_owned());
                assert_eq!(0, value_if_has_adjacent_cymbols(&matrix, 1, 1, 3))
            }
            {
                let mut matrix = Matrix::default();
                matrix.add_line("*....".to_owned());
                matrix.add_line(".123.".to_owned());
                matrix.add_line(".....".to_owned());
                assert_eq!(123, value_if_has_adjacent_cymbols(&matrix, 1, 1, 3))
            }
        }
    }
}
