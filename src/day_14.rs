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

    fn tilt_north(&mut self) {
        let num_lines = self.rocks.len();

        for i in 1..num_lines {
            for j in 0..self.rocks[i].len() {
                if self.rocks[i][j] != 'O' {
                    continue;
                }
                let mut pos = i;
                loop {
                    if pos == 0 {
                        break;
                    }
                    if self.rocks[pos - 1][j] != '.' {
                        break;
                    }
                    pos -= 1;
                }
                self.rocks[i][j] = '.';
                self.rocks[pos][j] = 'O';
            }
        }
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

    fn tilt_south(&mut self) {
        let num_lines = self.rocks.len();

        for i in (0..num_lines - 1).rev() {
            for j in 0..self.rocks[i].len() {
                if self.rocks[i][j] != 'O' {
                    continue;
                }
                let mut pos = i;
                loop {
                    if pos == num_lines - 1 {
                        break;
                    }
                    if self.rocks[pos + 1][j] != '.' {
                        break;
                    }
                    pos += 1;
                }
                self.rocks[i][j] = '.';
                self.rocks[pos][j] = 'O';
            }
        }
    }

    fn tilt_east(&mut self) {
        for line in &mut self.rocks {
            for i in (0..line.len() - 1).rev() {
                if line[i] != 'O' {
                    continue;
                }
                let mut pos = i;
                loop {
                    if pos == line.len() - 1 {
                        break;
                    }
                    if line[pos + 1] != '.' {
                        break;
                    }
                    pos += 1;
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

fn get_load_north(field: &Field) -> u64 {
    assert!(!field.rocks.is_empty());
    let mut result: u64 = 0;
    let num_lines = field.rocks.len();
    for (i, line) in field.rocks.iter().enumerate() {
        for (_, c) in line.iter().enumerate() {
            if *c == 'O' {
                result += (num_lines - i) as u64;
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
        field.rocks.push(vec!['O', '.']);
        field.rocks.push(vec!['.', 'O']);
        field.rocks.push(vec!['.', 'O']);

        let rotated = field.rotated();
        assert_eq!(2, rotated.rocks.len());

        assert_eq!(vec!['.', 'O', 'O'], rotated.rocks[0]);
        assert_eq!(vec!['O', '.', '.'], rotated.rocks[1]);
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

        // test other directions based on already tested tilt-west
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

        field.tilt_north();
        field = field.rotated();
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

    #[test]
    fn get_load_north_test() {
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

        assert_eq!(136, get_load_north(&field));
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

pub mod part2 {
    use super::*;

    fn tilt_cycle(field: &mut Field) {
        field.tilt_north();
        field.tilt_west();
        field.tilt_south();
        field.tilt_east();
    }

    fn solve_field(field: &mut Field) -> u64 {
        // 1_000_000_000
        for _ in 0..1_000 {
            tilt_cycle(field);
            println!("{}", get_load_north(&field));
        }
        get_load_north(&field)
    }

    pub struct Solver {}
    impl crate::aoc::Solver for Solver {
        fn solve(file_name: &str) -> String {
            let mut field = Field::default();
            for line in std::fs::read_to_string(file_name).unwrap().lines() {
                field.rocks.push(line.chars().collect());
            }
            solve_field(&mut field).to_string()
        }

        fn day() -> i32 {
            14
        }

        fn part() -> i32 {
            2
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn tilt_cycle_test() {
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

            tilt_cycle(&mut field);
            assert_eq!(".....#....".chars().collect::<Vec<char>>(), field.rocks[0]);
            assert_eq!("....#...O#".chars().collect::<Vec<char>>(), field.rocks[1]);
            assert_eq!("...OO##...".chars().collect::<Vec<char>>(), field.rocks[2]);
            assert_eq!(".OO#......".chars().collect::<Vec<char>>(), field.rocks[3]);
            assert_eq!(".....OOO#.".chars().collect::<Vec<char>>(), field.rocks[4]);
            assert_eq!(".O#...O#.#".chars().collect::<Vec<char>>(), field.rocks[5]);
            assert_eq!("....O#....".chars().collect::<Vec<char>>(), field.rocks[6]);
            assert_eq!("......OOOO".chars().collect::<Vec<char>>(), field.rocks[7]);
            assert_eq!("#...O###..".chars().collect::<Vec<char>>(), field.rocks[8]);
            assert_eq!("#..OO#....".chars().collect::<Vec<char>>(), field.rocks[9]);

            tilt_cycle(&mut field);
            assert_eq!(".....#....".chars().collect::<Vec<char>>(), field.rocks[0]);
            assert_eq!("....#...O#".chars().collect::<Vec<char>>(), field.rocks[1]);
            assert_eq!(".....##...".chars().collect::<Vec<char>>(), field.rocks[2]);
            assert_eq!("..O#......".chars().collect::<Vec<char>>(), field.rocks[3]);
            assert_eq!(".....OOO#.".chars().collect::<Vec<char>>(), field.rocks[4]);
            assert_eq!(".O#...O#.#".chars().collect::<Vec<char>>(), field.rocks[5]);
            assert_eq!("....O#...O".chars().collect::<Vec<char>>(), field.rocks[6]);
            assert_eq!(".......OOO".chars().collect::<Vec<char>>(), field.rocks[7]);
            assert_eq!("#..OO###..".chars().collect::<Vec<char>>(), field.rocks[8]);
            assert_eq!("#.OOO#...O".chars().collect::<Vec<char>>(), field.rocks[9]);

            tilt_cycle(&mut field);
            assert_eq!(".....#....".chars().collect::<Vec<char>>(), field.rocks[0]);
            assert_eq!("....#...O#".chars().collect::<Vec<char>>(), field.rocks[1]);
            assert_eq!(".....##...".chars().collect::<Vec<char>>(), field.rocks[2]);
            assert_eq!("..O#......".chars().collect::<Vec<char>>(), field.rocks[3]);
            assert_eq!(".....OOO#.".chars().collect::<Vec<char>>(), field.rocks[4]);
            assert_eq!(".O#...O#.#".chars().collect::<Vec<char>>(), field.rocks[5]);
            assert_eq!("....O#...O".chars().collect::<Vec<char>>(), field.rocks[6]);
            assert_eq!(".......OOO".chars().collect::<Vec<char>>(), field.rocks[7]);
            assert_eq!("#...O###.O".chars().collect::<Vec<char>>(), field.rocks[8]);
            assert_eq!("#.OOO#...O".chars().collect::<Vec<char>>(), field.rocks[9]);
        }
    }
}
