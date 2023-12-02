use std::env;

mod day_01;
mod day_02;

fn main() {
    let input_root_path = env::var("AOC_2023_INPUT_PATH").unwrap();

    let day_01_path = format!("{}/day_01.txt", input_root_path);

    let answer = day_01::part1::trebuchet(&day_01_path);
    println!("Day 01, part 1: {}", answer);
    assert_eq!(answer, 54953); // solved, regression check

    let answer = day_01::part2::trebuchet(&day_01_path);
    println!("Day 01, part 2: {}", answer);
    assert_eq!(answer, 53868);

    let day_02_path = format!("{}/day_02.txt", input_root_path);

    let answer = day_02::part1::solve(&day_02_path);
    println!("Day 02, part 1: {}", answer);
    assert_eq!(answer, 1931);

    let answer = day_02::part2::solve(&day_02_path);
    println!("Day 02, part 2: {}", answer);
    assert_eq!(answer, 83105);
}
