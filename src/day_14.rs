#[derive(Default)]
struct Field {
    rocks: Vec<Vec<char>>,
}

// this is awfly similar to day 13, at some point I need to move some of this common stuff to
// a common module
impl Field {
    // rotates a field counter-clockwise
    fn rotated(&self) -> Field {
        let mut rotated_rocks = vec![vec![' '; self.rocks.len()]; self.rocks[0].len()];

        for i in 0..self.rocks.len() {
            for j in 0..self.rocks[0].len() {
                rotated_rocks[self.rocks[0].len() - 1 - j][i] = self.rocks[i][j];
            }
        }

        Field { rocks: rotated_rocks }
    }

    fn tilt_west(&mut self) {
        for line in self.rocks.iter_mut() {
            for i in 1..line.len() {
                if line[i] != 'O' {
                    continue;
                }
                let mut pos = i;
                loop {
                    if pos == 0 {
                        break;
                    }
                    if line[pos - 1] != '.' {
                        break;
                    }
                    pos -= 1;
                }
                line[i] = '.';
                line[pos] = 'O';
            }
        }
    }
}

fn get_load(field: &Field) -> u64 {
    let mut result: u64 = 0;
    for line in field.rocks.iter() {
        for (i, c) in line.iter().enumerate() {
            if *c == 'O' {
                result += (line.len() - i) as u64;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotated_test() {
        let mut field = Field::default();
        field.rocks.push(vec!['#', '.']);
        field.rocks.push(vec!['.', '#']);
        field.rocks.push(vec!['.', '#']);

        let rotated = field.rotated();
        assert_eq!(2, rotated.rocks.len());

        assert_eq!(vec!['.', '#', '#'], rotated.rocks[0]);
        assert_eq!(vec!['#', '.', '.'], rotated.rocks[1]);
    }

    #[test]
    fn tilt_test() {
        let mut field = Field::default();
        field.rocks.push("O....#....".chars().collect());
        field.rocks.push("O.OO#....#".chars().collect());
        field.rocks.push(".....##...".chars().collect());
        field.rocks.push("OO.#O....O".chars().collect());
        field.rocks.push(".O.....O#.".chars().collect());
        field.rocks.push("O.#..O.#.#".chars().collect());
        field.rocks.push("..O..#O..O".chars().collect());
        field.rocks.push(".......O..".chars().collect());
        field.rocks.push("#....###..".chars().collect());
        field.rocks.push("#OO..#....".chars().collect());

        field = field.rotated();
        field.tilt_west();

        let mut expected = Field::default();
        expected.rocks.push("OOOO.#.O..".chars().collect());
        expected.rocks.push("OO..#....#".chars().collect());
        expected.rocks.push("OO..O##..O".chars().collect());
        expected.rocks.push("O..#.OO...".chars().collect());
        expected.rocks.push("........#.".chars().collect());
        expected.rocks.push("..#....#.#".chars().collect());
        expected.rocks.push("..O..#.O.O".chars().collect());
        expected.rocks.push("..O.......".chars().collect());
        expected.rocks.push("#....###..".chars().collect());
        expected.rocks.push("#....#....".chars().collect());

        expected = expected.rotated();

        assert_eq!(expected.rocks, field.rocks);
    }

    #[test]
    fn get_load_test() {
        let mut field = Field::default();
        field.rocks.push("OOOO.#.O..".chars().collect());
        field.rocks.push("OO..#....#".chars().collect());
        field.rocks.push("OO..O##..O".chars().collect());
        field.rocks.push("O..#.OO...".chars().collect());
        field.rocks.push("........#.".chars().collect());
        field.rocks.push("..#....#.#".chars().collect());
        field.rocks.push("..O..#.O.O".chars().collect());
        field.rocks.push("..O.......".chars().collect());
        field.rocks.push("#....###..".chars().collect());
        field.rocks.push("#....#....".chars().collect());

        field = field.rotated();

        assert_eq!(136, get_load(&field));
    }
}

pub mod part1 {
    use super::*;

    fn solve_field(field: &Field) -> u64 {
        let mut field = field.rotated();
        field.tilt_west();
        get_load(&field)
    }

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let mut field = Field::default();
            for line in std::fs::read_to_string(file_name).unwrap().lines() {
                field.rocks.push(line.chars().collect());
            }
            solve_field(&field).to_string()
        }

        fn day() -> i32 {
            14
        }

        fn part() -> i32 {
            1
        }
    }
}
