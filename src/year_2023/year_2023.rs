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
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;

pub struct Solver {}

impl aoc::Solver for Solver {
    fn solve(year: i32, day: i32, part: i32) {
        if !aoc::year_matched(year, 2023) {
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

        aoc::solve::<day_09::part1::Puzzle>(day, part);
        aoc::solve::<day_09::part2::Puzzle>(day, part);

        aoc::solve::<day_10::part1::Puzzle>(day, part);
        aoc::solve::<day_10::part2::Puzzle>(day, part);

        aoc::solve::<day_11::part1::Puzzle>(day, part);
        aoc::solve::<day_11::part2::Puzzle>(day, part);

        aoc::solve::<day_12::part1::Puzzle>(day, part);
        aoc::solve::<day_12::part2::Puzzle>(day, part);

        aoc::solve::<day_13::part1::Puzzle>(day, part);
        aoc::solve::<day_13::part2::Puzzle>(day, part);

        aoc::solve::<day_14::part1::Puzzle>(day, part);
        aoc::solve::<day_14::part2::Puzzle>(day, part);

        aoc::solve::<day_15::part1::Puzzle>(day, part);
        aoc::solve::<day_15::part2::Puzzle>(day, part);

        aoc::solve::<day_16::part1::Puzzle>(day, part);
        aoc::solve::<day_16::part2::Puzzle>(day, part);

        aoc::solve::<day_17::part1::Puzzle>(day, part);
    }
}
