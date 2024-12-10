#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    #[default]
    Unknown = 0,
    North = 0x1,
    West = 0x2,
    South = 0x4,
    East = 0x8,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Unknown => return Direction::Unknown,
            Direction::North => return Direction::South,
            Direction::West => return Direction::East,
            Direction::South => return Direction::North,
            Direction::East => return Direction::West,
        };
    }
}
