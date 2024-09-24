#[derive(Default)]
struct Field {
    rocks: Vec<Vec<char>>,
}

// this is awfully similar to day 13, at some point I need to move some of this common stuff to
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

    fn tilt_field(field: &mut Field) -> (u64, u64, u64, u64) {
        field.tilt_north();
        let a = get_load_north(field);

        field.tilt_west();
        let b = get_load_north(field);

        field.tilt_south();
        let c = get_load_north(field);

        field.tilt_east();
        let d = get_load_north(field);

        (a, b, c, d)
    }

    // to detect it as a cycle we need to make sure we have two consecutive matched lines
    // it means that we need to be off by one for all further calculations
    fn get_cycle(history: &Vec<(u64, u64, u64, u64)>, current_step: &(u64, u64, u64, u64)) -> Option<usize> {
        if history.is_empty() {
            return None;
        }
        if let Some(second_position) = history.iter().position(|item| item == current_step) {
            if second_position == 0 {
                return None;
            }
            if history[second_position - 1] == *history.last().unwrap() {
                return Some(second_position - 1);
            }
        }
        return None;
    }

    fn get_wrapped_index(cycle_start: usize, current_step: usize, n: usize) -> usize {
        let cycle_len = current_step as usize - cycle_start - 1;
        let index = (n - cycle_start) % cycle_len + cycle_start;
        return index;
    }

    fn solve_field(field: &mut Field) -> u64 {
        let mut history = Vec::new();
        let n = 1_000_000_000;
        for i in 0..n {
            let step_result = tilt_field(field);
            if let Some(cycle_start) = get_cycle(&history, &step_result) {
                // it's 0-based, so we need to find a value on nth - 1 step. damn you off-by-one
                let index = get_wrapped_index(cycle_start, i, n - 1);
                return history[index].3;
            }
            history.push(step_result);
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
        fn tilt_field_test() {
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

            tilt_field(&mut field);
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

            tilt_field(&mut field);
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

            tilt_field(&mut field);
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

        #[test]
        fn get_cycle_test() {
            let mut history = Vec::new();

            history.push((136, 136, 87, 87));
            history.push((129, 129, 69, 69));
            history.push((114, 114, 69, 69));
            history.push((110, 110, 69, 69));
            history.push((110, 110, 65, 65));
            history.push((105, 105, 64, 64));

            assert_eq!(None, get_cycle(&history, &(103, 103, 65, 65)));
            history.push((103, 103, 65, 65));

            assert_eq!(None, get_cycle(&history, &(106, 106, 63, 63)));
            history.push((106, 106, 63, 63));

            assert_eq!(None, get_cycle(&history, &(111, 111, 68, 68)));
            history.push((111, 111, 68, 68));

            assert_eq!(None, get_cycle(&history, &(114, 114, 69, 69)));
            history.push((114, 114, 69, 69));

            assert_eq!(Some(2), get_cycle(&history, &(110, 110, 69, 69)));
        }

        #[test]
        fn get_wrapped_index_test() {
            assert_eq!(6, get_wrapped_index(2, 10, 20));
            assert_eq!(7, get_wrapped_index(2, 10, 21));
            assert_eq!(8, get_wrapped_index(2, 10, 22));
            assert_eq!(2, get_wrapped_index(2, 10, 23));

            assert_eq!(3, get_wrapped_index(2, 10, 24));
            assert_eq!(4, get_wrapped_index(2, 10, 25));
            assert_eq!(5, get_wrapped_index(2, 10, 26));
            assert_eq!(6, get_wrapped_index(2, 10, 27));
            assert_eq!(7, get_wrapped_index(2, 10, 28));
            assert_eq!(8, get_wrapped_index(2, 10, 29));
            assert_eq!(2, get_wrapped_index(2, 10, 30));
        }

        #[test]
        fn solve_test() {
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

            assert_eq!(64, solve_field(&mut field));
        }
    }
}
