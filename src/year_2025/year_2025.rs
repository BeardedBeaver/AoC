mod day_01;
mod day_02;

pub struct Solver {}

impl aoc::Solver for Solver {
    fn solve(year: i32, day: i32, part: i32) {
        if !aoc::year_matched(year, 2025) {
            return;
        }
        aoc::solve::<day_01::part1::Puzzle>(day, part);
        aoc::solve::<day_01::part2::Puzzle>(day, part);
        aoc::solve::<day_02::part1::Puzzle>(day, part);
        aoc::solve::<day_02::part2::Puzzle>(day, part);
    }
}
