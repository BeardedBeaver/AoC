#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    dist: u64,
}

fn parse_races(time: &str, dist: &str) -> Vec<Race> {
    let times = time.split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap());
    let dists = dist.split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap());

    std::iter::zip(times, dists)
        .map(|(time, dist)| Race { time, dist })
        .collect()
}

fn parse_single_race(time: &str, dist: &str) -> Race {
    let mut t: u64 = 0;
    let mut d: u64 = 0;
    for c in time.chars() {
        if c == ' ' {
            continue;
        }
        t *= 10;
        t += c.to_digit(10).unwrap() as u64;
    }
    for c in dist.chars() {
        if c == ' ' {
            continue;
        }
        d *= 10;
        d += c.to_digit(10).unwrap() as u64;
    }

    Race { time: t, dist: d }
}

fn solve_race(race: &Race) -> u64 {
    let mut result = 0;

    for t in 0..race.time {
        let d = t * (race.time - t);
        if d > race.dist {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_races_test() {
        let time = "59     79     65     75";
        let dist = "597   1234   1032   1328";
        let races = parse_races(time, dist);

        assert_eq!(4, races.len());

        assert_eq!(Race { time: 59, dist: 597 }, races[0]);
        assert_eq!(Race { time: 79, dist: 1234 }, races[1]);
        assert_eq!(Race { time: 65, dist: 1032 }, races[2]);
        assert_eq!(Race { time: 75, dist: 1328 }, races[3]);
    }

    #[test]
    fn parse_single_race_test() {
        let time = "7  15   30";
        let dist = "9  40  200";
        let race = parse_single_race(time, dist);

        assert_eq!(
            Race {
                time: 71530,
                dist: 940200
            },
            race
        );
    }
}

pub mod part1 {
    use super::*;

    fn solve_puzzle(races: &Vec<Race>) -> u64 {
        let mut result = 1;
        for race in races.iter() {
            result *= solve_race(race);
        }
        result
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let handle = std::fs::read_to_string(file_name).unwrap();
            let lines: Vec<&str> = handle.lines().collect();
            assert_eq!(2, lines.len());

            let time: Vec<&str> = lines[0].split(":").collect();
            let dist: Vec<&str> = lines[1].split(":").collect();
            assert_eq!(2, time.len());
            assert_eq!(2, dist.len());
            let races = parse_races(time[1].trim(), dist[1].trim());
            solve_puzzle(&races).to_string()
        }

        fn day() -> i32 {
            6
        }

        fn part() -> i32 {
            1
        }

        fn year() -> i32 {
            2023
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn solve_puzzle_test() {
            let races = vec![
                Race { time: 7, dist: 9 },
                Race { time: 15, dist: 40 },
                Race { time: 30, dist: 200 },
            ];

            assert_eq!(288, solve_puzzle(&races));
        }
    }
}

pub mod part2 {
    use super::*;

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let handle = std::fs::read_to_string(file_name).unwrap();
            let lines: Vec<&str> = handle.lines().collect();
            assert_eq!(2, lines.len());

            let time: Vec<&str> = lines[0].split(":").collect();
            let dist: Vec<&str> = lines[1].split(":").collect();
            assert_eq!(2, time.len());
            assert_eq!(2, dist.len());
            let race = parse_single_race(time[1].trim(), dist[1].trim());
            solve_race(&race).to_string()
        }

        fn day() -> i32 {
            6
        }

        fn part() -> i32 {
            2
        }

        fn year() -> i32 {
            2023
        }
    }
}
