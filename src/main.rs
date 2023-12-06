use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

fn day_matched(arg: i32, day: i32) -> bool {
    arg < 0 || arg == day
}

fn main() {
    let input_root_path = env::var("AOC_2023_INPUT_PATH").unwrap();

    let args: Vec<String> = env::args().collect();
    let day: i32 = if args.len() < 2 { -1 } else { args[1].parse().unwrap() };

    if day_matched(day, 1) {
        let day_01_path = format!("{}/day_01.txt", input_root_path);

        let answer = day_01::part1::trebuchet(&day_01_path);
        println!("Day 01, part 1: {}", answer);
        assert_eq!(answer, 54953); // solved, regression check

        let answer = day_01::part2::trebuchet(&day_01_path);
        println!("Day 01, part 2: {}", answer);
        assert_eq!(answer, 53868);
    }

    if day_matched(day, 2) {
        let day_02_path = format!("{}/day_02.txt", input_root_path);

        let answer = day_02::part1::solve(&day_02_path);
        println!("Day 02, part 1: {}", answer);
        assert_eq!(answer, 1931);

        let answer = day_02::part2::solve(&day_02_path);
        println!("Day 02, part 2: {}", answer);
        assert_eq!(answer, 83105);
    }

    if day_matched(day, 3) {
        let day_03_path = format!("{}/day_03.txt", input_root_path);

        let answer = day_03::part1::solve(&day_03_path);
        println!("Day 03, part 1: {}", answer);
        assert_eq!(answer, 531561);

        let answer = day_03::part2::solve(&day_03_path);
        println!("Day 03, part 2: {}", answer);
        assert_eq!(answer, 83279367);
    }

    if day_matched(day, 4) {
        let day_04_path = format!("{}/day_04.txt", input_root_path);

        let answer = day_04::part1::solve(&day_04_path);
        println!("Day 04, part 1: {}", answer);
        assert_eq!(answer, 20407);

        let answer = day_04::part2::solve(&day_04_path);
        println!("Day 04, part 2: {}", answer);
        assert_eq!(answer, 23806951);
    }

    if day_matched(day, 5) {
        let day_05_path = format!("{}/day_05.txt", input_root_path);
        use std::time::Instant;

        let answer = day_05::part1::solve(&day_05_path);
        println!("Day 05, part 1: {}", answer);
        assert_eq!(answer, 910845529);

        let now = Instant::now();
        let answer = day_05::part2::solve(&day_05_path);
        println!("Day 05, part 2: {}", answer);
        assert_eq!(answer, 77435348);
        let elapsed = now.elapsed();
        println!("Elapsed (seq): {:.2?}", elapsed);

        let now = Instant::now();
        let answer = day_05::part2::solve_parallel(&day_05_path);
        println!("Day 05, part 2: {}", answer);
        assert_eq!(answer, 77435348);
        let elapsed = now.elapsed();
        println!("Elapsed (par): {:.2?}", elapsed);
    }

    if day_matched(day, 6) {
        let day_06_path = format!("{}/day_06.txt", input_root_path);

        let answer = day_06::part1::solve(&day_06_path);
        println!("Day 06, part 1: {}", answer);
        assert_eq!(answer, 220320);

        let answer = day_06::part2::solve(&day_06_path);
        println!("Day 06, part 2: {}", answer);
        assert_eq!(answer, 34454850);
    }
}
