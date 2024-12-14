use std::env;

mod direction;
mod field;
mod point;

pub use direction::Direction;
pub use field::Field;
pub use point::Point;

pub trait Puzzle {
    fn solve(file_name: &str) -> String;
    fn year() -> i32;
    fn day() -> i32;
    fn part() -> i32;
}

pub trait Solver {
    fn solve(year: i32, day: i32, part: i32);
}

pub fn year_matched(arg: i32, year: i32) -> bool {
    arg < 0 || arg == year
}

pub fn day_matched(arg: i32, day: i32) -> bool {
    arg < 0 || arg == day
}

pub fn part_matched(arg: i32, part: i32) -> bool {
    arg < 0 || arg == part
}

pub fn parse_or_panic<T>(string: &str) -> T
where
    T: std::str::FromStr,
{
    return string
        .parse::<T>()
        .unwrap_or_else(|_| panic!("Failed to convert {}", string));
}

pub fn get_input_file_names(day: i32, year: i32) -> Vec<String> {
    let path_env_var = format!("AOC_{}_INPUT_PATH", year);
    let input_root_path =
        env::var(path_env_var.clone()).unwrap_or_else(|err| panic!("Error: {}: {}", err, path_env_var));
    if !std::fs::metadata(&input_root_path)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
    {
        panic!("The specified directory does not exist: {}", input_root_path);
    }
    let mask = format!("{input_root_path}/day_{day:0>2}*.txt");
    let entities = glob::glob(&mask).expect("Failed to read glob pattern");
    entities.map(|e| e.unwrap().display().to_string()).collect()
}

pub fn solve<Puzzle: crate::Puzzle>(day: i32, part: i32) {
    if !day_matched(day, Puzzle::day()) {
        return;
    }
    if !part_matched(part, Puzzle::part()) {
        return;
    }

    let file_names = get_input_file_names(Puzzle::day(), Puzzle::year());
    if file_names.is_empty() {
        panic!("No input files found for year {} day {}", Puzzle::year(), Puzzle::day());
    }
    for f in file_names.iter() {
        use std::time::Instant;

        let now = Instant::now();
        let answer = Puzzle::solve(&f);
        let elapsed = now.elapsed();

        println!(
            "{} Day {:0>2}, part {}: {}\n\tElapsed: {:.2?}\n",
            Puzzle::year(),
            Puzzle::day(),
            Puzzle::part(),
            answer,
            elapsed
        );
    }
}
