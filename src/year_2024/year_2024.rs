mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

pub struct Solver {}

impl aoc::Solver for Solver {
    fn solve(year: i32, day: i32, part: i32) {
        if !aoc::year_matched(year, 2024) {
            return;
        }

        aoc::solve::<day_01::part1::Puzzle>(day, part);
        aoc::solve::<day_01::part2::Puzzle>(day, part);

        aoc::solve::<day_02::part1::Puzzle>(day, part);
        aoc::solve::<day_02::part2::Puzzle>(day, part);

        aoc::solve::<day_03::part1::Puzzle>(day, part);
        aoc::solve::<day_03::part2::Puzzle>(day, part);

        aoc::solve::<day_04::part1::Puzzle>(day, part);
        aoc::solve::<day_04::part2::Puzzle>(day, part);

        aoc::solve::<day_05::part1::Puzzle>(day, part);
        aoc::solve::<day_05::part2::Puzzle>(day, part);

        aoc::solve::<day_06::part1::Puzzle>(day, part);
        aoc::solve::<day_06::part2::Puzzle>(day, part);

        aoc::solve::<day_07::part1::Puzzle>(day, part);
        aoc::solve::<day_07::part2::Puzzle>(day, part);

        aoc::solve::<day_08::part1::Puzzle>(day, part);
        aoc::solve::<day_08::part2::Puzzle>(day, part);
    }
}
