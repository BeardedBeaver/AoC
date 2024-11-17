#[derive(Default, Clone)]
struct Range {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

impl Range {
    pub fn transform(self: &Range, num: u64) -> Option<u64> {
        if num < self.source_start || num > self.source_start + self.length {
            return None;
        }
        Some(num - self.source_start + self.dest_start)
    }
}

#[derive(Default, Clone)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    pub fn append(self: &mut Map, line: &str) {
        let values: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        assert_eq!(values.len(), 3);
        self.ranges.push(Range {
            dest_start: values[0],
            source_start: values[1],
            length: values[2],
        });
    }

    pub fn transform(self: &Map, seed: u64) -> u64 {
        for range in self.ranges.iter() {
            if let Some(value) = range.transform(seed) {
                return value;
            }
        }
        seed
    }
}

#[derive(Default)]
struct Data {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Data {
    pub fn from_file(file_name: &str) -> Data {
        let mut result = Data::default();
        let mut map = Map::default();

        for line in std::fs::read_to_string(file_name).unwrap().lines() {
            if line.starts_with("seeds:") {
                let parts: Vec<&str> = line.split(":").collect();
                assert_eq!(2, parts.len());
                result.seeds = parts[1]
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect();
            } else if line.is_empty() {
            } else if line.as_bytes()[0].is_ascii_digit() {
                map.append(line);
            } else {
                if !map.ranges.is_empty() {
                    result.maps.push(map);
                    map = Map::default();
                }
            }
        }
        result.maps.push(map);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_test() {
        let mut map = Map::default();
        map.append("5 50 9");
        map.append("15 70 9");
        map.append("25 60 9");

        assert_eq!(6, map.transform(51));
        assert_eq!(14, map.transform(59));
        assert_eq!(25, map.transform(60));
        assert_eq!(26, map.transform(61));
    }
}

pub mod part1 {
    use super::*;

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let mut result: Option<u64> = None;

            let puzzle = Data::from_file(file_name);

            for seed in puzzle.seeds {
                let mut seed = seed;
                for map in puzzle.maps.iter() {
                    seed = map.transform(seed);
                }
                match result {
                    None => result = Some(seed),
                    Some(value) => result = Some(u64::min(value, seed)),
                }
            }

            result.unwrap().to_string()
        }

        fn day() -> i32 {
            5
        }

        fn part() -> i32 {
            1
        }

        fn year() -> i32 {
            2023
        }
    }
}

pub mod part2 {
    use super::*;
    use std::sync::Arc;

    fn solve_interval(mappings: &Vec<Map>, start: u64, len: u64) -> u64 {
        let mut result = u64::MAX;

        for i in start..=start + len {
            let mut seed = i;
            for map in mappings.iter() {
                seed = map.transform(seed);
            }
            result = u64::min(result, seed);
        }

        result
    }

    fn split_ranges(ranges: &Vec<(u64, u64)>, range_size: u64) -> Vec<(u64, u64)> {
        let mut result = Vec::new();

        for &(start, length) in ranges.iter() {
            let mut total_length = 0;
            let mut current_start = start;

            while total_length < length {
                let split_length = std::cmp::min(length - total_length, range_size);
                result.push((current_start, split_length));
                current_start += split_length + 1;
                total_length += split_length + 1;
            }
        }

        result
    }

    fn solve_puzzle_parallel(data: &Data) -> u64 {
        let inputs: Vec<(u64, u64)> = data.seeds.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
        let chunks = split_ranges(&inputs, 25_000_000);

        let n_workers = std::thread::available_parallelism().unwrap().get();
        let pool = threadpool::ThreadPool::new(n_workers);
        let (tx, rx) = std::sync::mpsc::channel::<u64>();

        let mappings = Arc::new(data.maps.clone());

        for c in chunks {
            let tx = tx.clone();
            let m = Arc::clone(&mappings);

            pool.execute(move || {
                let result = solve_interval(&m, c.0, c.1);
                tx.send(result).unwrap();
            });
        }
        pool.join();
        drop(tx);

        let result = rx.iter().min().unwrap();
        result
    }

    pub struct Puzzle {}
    impl aoc::Puzzle for Puzzle {
        fn solve(file_name: &str) -> String {
            let data = Data::from_file(file_name);
            solve_puzzle_parallel(&data).to_string()
        }

        fn day() -> i32 {
            5
        }

        fn part() -> i32 {
            2
        }

        fn year() -> i32 {
            2023
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn split_ranges_test() {
            {
                let input = vec![(1000, 499)];
                let ranges = split_ranges(&input, 99);

                assert_eq!(vec![(1000, 99), (1100, 99), (1200, 99), (1300, 99), (1400, 99)], ranges);
            }
            {
                let input = vec![(1000, 550)];
                let ranges = split_ranges(&input, 99);

                assert_eq!(
                    vec![(1000, 99), (1100, 99), (1200, 99), (1300, 99), (1400, 99), (1500, 50)],
                    ranges
                );
            }
        }

        #[test]
        fn example_test() {
            let mut puzzle = Puzzle::default();

            puzzle.seeds = vec![79, 14, 55, 13];

            // Seed to Soil Map
            puzzle.maps.push(Map {
                ranges: vec![
                    Range {
                        dest_start: 50,
                        source_start: 98,
                        length: 2,
                    },
                    Range {
                        dest_start: 52,
                        source_start: 50,
                        length: 48,
                    },
                ],
            });

            // Soil to Fertilizer Map
            puzzle.maps.push(Map {
                ranges: vec![
                    Range {
                        dest_start: 0,
                        source_start: 15,
                        length: 37,
                    },
                    Range {
                        dest_start: 37,
                        source_start: 52,
                        length: 2,
                    },
                    Range {
                        dest_start: 39,
                        source_start: 0,
                        length: 15,
                    },
                ],
            });

            // Fertilizer to Water Map
            puzzle.maps.push(Map {
                ranges: vec![
                    Range {
                        dest_start: 49,
                        source_start: 53,
                        length: 8,
                    },
                    Range {
                        dest_start: 0,
                        source_start: 11,
                        length: 42,
                    },
                    Range {
                        dest_start: 42,
                        source_start: 0,
                        length: 7,
                    },
                    Range {
                        dest_start: 57,
                        source_start: 7,
                        length: 4,
                    },
                ],
            });

            // Water to Light Map
            puzzle.maps.push(Map {
                ranges: vec![
                    Range {
                        dest_start: 88,
                        source_start: 18,
                        length: 7,
                    },
                    Range {
                        dest_start: 18,
                        source_start: 25,
                        length: 70,
                    },
                ],
            });

            // Light to Temperature Map
            puzzle.maps.push(Map {
                ranges: vec![
                    Range {
                        dest_start: 45,
                        source_start: 77,
                        length: 23,
                    },
                    Range {
                        dest_start: 81,
                        source_start: 45,
                        length: 19,
                    },
                    Range {
                        dest_start: 68,
                        source_start: 64,
                        length: 13,
                    },
                ],
            });

            // Temperature to Humidity Map
            puzzle.maps.push(Map {
                ranges: vec![
                    Range {
                        dest_start: 0,
                        source_start: 69,
                        length: 1,
                    },
                    Range {
                        dest_start: 1,
                        source_start: 0,
                        length: 69,
                    },
                ],
            });

            // Humidity to Location Map
            puzzle.maps.push(Map {
                ranges: vec![
                    Range {
                        dest_start: 60,
                        source_start: 56,
                        length: 37,
                    },
                    Range {
                        dest_start: 56,
                        source_start: 93,
                        length: 4,
                    },
                ],
            });

            assert_eq!(46, solve_puzzle_parallel(&puzzle));
        }
    }
}
