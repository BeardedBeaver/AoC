use std::env;

mod aoc;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;

fn day_matched(arg: i32, day: i32) -> bool {
    arg < 0 || arg == day
}

fn get_input_file_names(day: i32) -> Vec<String> {
    let input_root_path = env::var("AOC_2023_INPUT_PATH").expect("AOC_2023_INPUT_PATH is not set");
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

fn solve<Solver: crate::aoc::Solver>(day: i32) {
    if !day_matched(day, Solver::day()) {
        return;
    }
    let file_names = get_input_file_names(Solver::day());
    for f in file_names.iter() {
        use std::time::Instant;

        let now = Instant::now();
        let answer = Solver::solve(&f);
        let elapsed = now.elapsed();

        println!(
            "Day {:0>2}, part {}: {}\n\tElapsed: {:.2?}\n",
            Solver::day(),
            Solver::part(),
            answer,
            elapsed
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: i32 = if args.len() < 2 { -1 } else { args[1].parse().unwrap() };

    solve::<day_01::part1::Solver>(day);
    solve::<day_01::part2::Solver>(day);

    solve::<day_02::part1::Solver>(day);
    solve::<day_02::part2::Solver>(day);

    solve::<day_03::part1::Solver>(day);
    solve::<day_03::part2::Solver>(day);

    solve::<day_04::part1::Solver>(day);
    solve::<day_04::part2::Solver>(day);

    solve::<day_05::part1::Solver>(day);
    solve::<day_05::part2::Solver>(day);

    solve::<day_06::part1::Solver>(day);
    solve::<day_06::part2::Solver>(day);

    solve::<day_07::part1::Solver>(day);
    solve::<day_07::part2::Solver>(day);

    solve::<day_08::part1::Solver>(day);
    solve::<day_08::part2::Solver>(day);

    solve::<day_09::part1::Solver>(day);
    solve::<day_09::part2::Solver>(day);

    solve::<day_10::part1::Solver>(day);
    solve::<day_10::part2::Solver>(day);
}
