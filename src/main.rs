use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;

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

    let day_03_path = format!("{}/day_03.txt", input_root_path);

    let answer = day_03::part1::solve(&day_03_path);
    println!("Day 03, part 1: {}", answer);
    assert_eq!(answer, 531561);

    let answer = day_03::part2::solve(&day_03_path);
    println!("Day 03, part 2: {}", answer);
    assert_eq!(answer, 83279367);

    let day_04_path = format!("{}/day_04.txt", input_root_path);

    let answer = day_04::part1::solve(&day_04_path);
    println!("Day 04, part 1: {}", answer);
    assert_eq!(answer, 20407);

    let answer = day_04::part2::solve(&day_04_path);
    println!("Day 04, part 2: {}", answer);
    assert_eq!(answer, 23806951);
}
