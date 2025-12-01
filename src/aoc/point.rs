// cSpell: words drow dcol

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point<I> {
    pub row: I,
    pub col: I,
}

impl<I> Point<I>
where
    I: num_traits::PrimInt,
{
    pub fn neighbors_orthogonal(&self) -> PointNeighborIterator<'_, I> {
        PointNeighborIterator {
            point: &self,
            current_index: 0,
            deltas: &DELTAS_ORT,
        }
    }

    pub fn neighbors_diagonal(&self) -> PointNeighborIterator<'_, I> {
        PointNeighborIterator {
            point: &self,
            current_index: 0,
            deltas: &DELTAS_DIAG,
        }
    }

    pub fn neighbors_all(&self) -> PointNeighborIterator<'_, I> {
        PointNeighborIterator {
            point: &self,
            current_index: 0,
            deltas: &DELTAS_ALL,
        }
    }
}

pub struct PointNeighborIterator<'a, I> {
    point: &'a Point<I>,
    current_index: usize,
    deltas: &'static [(i32, i32)],
}

static DELTAS_ORT: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
static DELTAS_DIAG: [(i32, i32); 4] = [(-1, -1), (1, -1), (-1, 1), (1, 1)];
static DELTAS_ALL: [(i32, i32); 8] = [
    (-1, 0),  // Up
    (-1, 1),  // Top-right
    (0, 1),   // Right
    (1, 1),   // Bottom-right
    (1, 0),   // Down
    (1, -1),  // Bottom-left
    (0, -1),  // Left
    (-1, -1), // Top-left
];

fn get_next_point<I>(point: &Point<I>, index: usize, deltas: &[(i32, i32)]) -> Option<Point<I>>
where
    I: num_traits::PrimInt,
{
    assert!(index < deltas.len());
    let (drow, dcol) = deltas[index];
    let drow = I::from(drow)?;
    let dcol = I::from(dcol)?;
    Some(Point {
        row: point.row + drow,
        col: point.col + dcol,
    })
}

impl<I> Iterator for PointNeighborIterator<'_, I>
where
    I: num_traits::PrimInt,
{
    type Item = Point<I>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.deltas.len() {
            return None;
        }
        let point = get_next_point(&self.point, self.current_index, self.deltas);
        self.current_index += 1;
        point
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Point;

    #[test]
    fn neighbors_orthogonal_test() {
        let point: Point<i32> = Point { row: 4, col: 8 };
        let mut expected_neighbors: HashSet<Point<i32>> = HashSet::from_iter(
            vec![
                Point { row: 3, col: 8 },
                Point { row: 5, col: 8 },
                Point { row: 4, col: 7 },
                Point { row: 4, col: 9 },
            ]
            .iter()
            .copied(),
        );

        for p in point.neighbors_orthogonal() {
            assert!(expected_neighbors.contains(&p));
            expected_neighbors.remove(&p);
        }

        assert!(expected_neighbors.is_empty());
    }

    #[test]
    fn neighbors_diagonal_test() {
        let point: Point<i32> = Point { row: 4, col: 8 };
        let mut expected_neighbors: HashSet<Point<i32>> = HashSet::from_iter(
            vec![
                Point { row: 3, col: 7 },
                Point { row: 3, col: 9 },
                Point { row: 5, col: 7 },
                Point { row: 5, col: 9 },
            ]
            .iter()
            .copied(),
        );

        for p in point.neighbors_diagonal() {
            assert!(expected_neighbors.contains(&p));
            expected_neighbors.remove(&p);
        }

        assert!(expected_neighbors.is_empty());
    }

    #[test]
    fn neighbors_all_test() {
        let point: Point<i32> = Point { row: 4, col: 8 };
        let mut expected_neighbors: HashSet<Point<i32>> = HashSet::from_iter(
            vec![
                // orth
                Point { row: 3, col: 8 },
                Point { row: 5, col: 8 },
                Point { row: 4, col: 7 },
                Point { row: 4, col: 9 },
                // diag
                Point { row: 3, col: 7 },
                Point { row: 3, col: 9 },
                Point { row: 5, col: 7 },
                Point { row: 5, col: 9 },
            ]
            .iter()
            .copied(),
        );

        for p in point.neighbors_all() {
            assert!(expected_neighbors.contains(&p));
            expected_neighbors.remove(&p);
        }

        assert!(expected_neighbors.is_empty());
    }
}
