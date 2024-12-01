use clap::Parser;

use aoc::Solver;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = -1)]
    year: i32,

    #[arg(short, long, default_value_t = -1)]
    day: i32,

    #[arg(short, long, default_value_t = -1)]
    part: i32,
}

fn main() {
    let args = Args::parse();

    println!("Let's solve Advent Of Code!");

    year_2023::Solver::solve(args.year, args.day, args.part);
    year_2024::Solver::solve(args.year, args.day, args.part);
}
