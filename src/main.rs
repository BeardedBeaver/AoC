use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: i32 = if args.len() < 2 { -1 } else { args[1].parse().unwrap() };

    let cur_day = 1;
    if day_matched(day, cur_day) {
        let file_names = get_input_file_names(cur_day);
        for f in file_names.iter() {
            let answer = day_01::part1::solve(&f);
            println!("Day {:0>2}, part 1: {}", cur_day, answer);
            assert_eq!(answer, 54953); // solved, regression check

            let answer = day_01::part2::solve(&f);
            println!("Day {:0>2}, part 2: {}", cur_day, answer);
            assert_eq!(answer, 53868);
        }
    }

    let cur_day = 2;
    if day_matched(day, cur_day) {
        let file_names = get_input_file_names(cur_day);
        for f in file_names.iter() {
            let answer = day_02::part1::solve(&f);
            println!("Day {:0>2}, part 1: {}", cur_day, answer);
            assert_eq!(answer, 1931);

            let answer = day_02::part2::solve(&f);
            println!("Day {:0>2}, part 2: {}", cur_day, answer);
            assert_eq!(answer, 83105);
        }
    }

    let cur_day = 3;
    if day_matched(day, cur_day) {
        let file_names = get_input_file_names(cur_day);
        for f in file_names.iter() {
            let answer = day_03::part1::solve(&f);
            println!("Day {:0>2}, part 1: {}", cur_day, answer);
            assert_eq!(answer, 531561);

            let answer = day_03::part2::solve(&f);
            println!("Day {:0>2}, part 2: {}", cur_day, answer);
            assert_eq!(answer, 83279367);
        }
    }

    let cur_day = 4;
    if day_matched(day, cur_day) {
        let file_names = get_input_file_names(cur_day);
        for f in file_names.iter() {
            let answer = day_04::part1::solve(&f);
            println!("Day {:0>2}, part 1: {}", cur_day, answer);
            assert_eq!(answer, 20407);

            let answer = day_04::part2::solve(&f);
            println!("Day {:0>2}, part 2: {}", cur_day, answer);
            assert_eq!(answer, 23806951);
        }
    }

    let cur_day = 5;
    if day_matched(day, cur_day) {
        let file_names = get_input_file_names(cur_day);
        for f in file_names.iter() {
            use std::time::Instant;

            let answer = day_05::part1::solve(&f);
            println!("Day {:0>2}, part 1: {}", cur_day, answer);
            assert_eq!(answer, 910845529);

            let now = Instant::now();
            let answer = day_05::part2::solve(&f);
            println!("Day {:0>2}, part 2: {}", cur_day, answer);
            assert_eq!(answer, 77435348);
            let elapsed = now.elapsed();
            println!("Elapsed (seq): {:.2?}", elapsed);

            let now = Instant::now();
            let answer = day_05::part2::solve_parallel(&f);
            println!("Day {:0>2}, part 2: {}", cur_day, answer);
            assert_eq!(answer, 77435348);
            let elapsed = now.elapsed();
            println!("Elapsed (par): {:.2?}", elapsed);
        }
    }

    let cur_day = 6;
    if day_matched(day, cur_day) {
        let file_names = get_input_file_names(cur_day);
        for f in file_names.iter() {
            let answer = day_06::part1::solve(&f);
            println!("Day {:0>2}, part 1: {}", cur_day, answer);
            assert_eq!(answer, 220320);

            let answer = day_06::part2::solve(&f);
            println!("Day {:0>2}, part 2: {}", cur_day, answer);
            assert_eq!(answer, 34454850);
        }
    }

    let cur_day = 7;
    if day_matched(day, cur_day) {
        let file_names = get_input_file_names(cur_day);
        for f in file_names.iter() {
            let answer = day_07::part1::solve(&f);
            println!("Day {:0>2}, part 1: {}", cur_day, answer);
            // assert_eq!(answer, 250474325);

            let answer = day_07::part2::solve(&f);
            println!("Day {:0>2}, part 2: {}", cur_day, answer);
            // assert_eq!(answer, 34454850);
        }
    }

    let cur_day = 8;
    if day_matched(day, cur_day) {
        let file_names = get_input_file_names(cur_day);
        for f in file_names.iter() {
            let answer = day_08::part1::solve(&f);
            println!("Day {:0>2}, part 1: {}", cur_day, answer);

            let answer = day_08::part2::solve(&f);
            println!("Day {:0>2}, part 2: {}", cur_day, answer);
        }
    }

    let cur_day = 9;
    if day_matched(day, cur_day) {
        let file_names = get_input_file_names(cur_day);
        for f in file_names.iter() {
            let answer = day_09::part1::solve(&f);
            println!("Day {:0>2}, part 1: {}", cur_day, answer);

            // let answer = day_08::part2::solve(&f);
            // println!("Day {:0>2}, part 2: {}", cur_day, answer);
        }
    }
}
