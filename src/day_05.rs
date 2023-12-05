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
struct Puzzle {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Puzzle {
    pub fn from_file(file_name: &str) -> Puzzle {
        let mut result = Puzzle::default();
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

    pub fn solve(file_name: &str) -> u64 {
        let mut result: Option<u64> = None;

        let puzzle = Puzzle::from_file(file_name);

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

        result.unwrap()
    }
}

pub mod part2 {
    use super::*;
    use std::sync::{Arc, Mutex};

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

    fn solve_puzzle_parallel(puzzle: &Puzzle) -> u64 {
        let results = Arc::new(Mutex::new(Vec::new()));

        let inputs: Vec<(u64, u64)> = puzzle
            .seeds
            .chunks(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();

        let threads: Vec<_> = inputs
            .into_iter()
            .map(|(start, len)| {
                let mappings = puzzle.maps.clone();
                let results = results.clone();

                std::thread::spawn(move || {
                    let result = solve_interval(&mappings, start, len);
                    results.lock().unwrap().push(result);
                })
            })
            .collect();

        for t in threads {
            t.join().unwrap();
        }

        let answer = results.lock().unwrap().iter().min().unwrap().clone();
        answer
    }

    fn solve_puzzle(puzzle: &Puzzle) -> u64 {
        let mut result = u64::MAX;

        let inputs: Vec<(u64, u64)> = puzzle
            .seeds
            .chunks(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();

        for i in inputs {
            result = u64::min(result, solve_interval(&puzzle.maps, i.0, i.1));
        }
        result
    }

    pub fn solve(file_name: &str) -> u64 {
        let puzzle = Puzzle::from_file(file_name);
        solve_puzzle(&puzzle)
    }

    pub fn solve_parallel(file_name: &str) -> u64 {
        let puzzle = Puzzle::from_file(file_name);
        solve_puzzle_parallel(&puzzle)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

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

            assert_eq!(46, solve_puzzle(&puzzle));
            assert_eq!(46, solve_puzzle_parallel(&puzzle));
        }
    }
}
