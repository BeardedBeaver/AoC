#[derive(Debug, Default)]
pub struct Field<Node> {
    nodes: Vec<Node>,
    row_count: usize,
    col_count: usize,
}

impl<Node> Field<Node>
where
    Node: Default + Clone,
{
    pub fn with_size(row_count: usize, col_count: usize) -> Self {
        Field {
            nodes: vec![Node::default(); row_count * col_count],
            row_count: row_count,
            col_count: col_count,
        }
    }

    pub fn get_row_count(&self) -> usize {
        self.row_count
    }

    pub fn get_col_count(&self) -> usize {
        self.col_count
    }

    pub fn node_mut<'a>(&'a mut self, row: usize, col: usize) -> Option<&'a mut Node> {
        let idx = self.flat_index(row, col)?;
        Some(&mut self.nodes[idx])
    }

    pub fn node<'a>(&'a self, row: usize, col: usize) -> Option<&'a Node> {
        let idx = self.flat_index(row, col)?;
        Some(&self.nodes[idx])
    }

    fn flat_index(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.row_count || col >= self.col_count {
            return None;
        }
        let idx = row * self.col_count + col;
        assert!(idx < self.nodes.len());
        Some(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::Field;

    #[test]
    fn flat_index_test() {
        let field: Field<i32> = Field::with_size(4, 9);
        assert_eq!(field.flat_index(0, 0), Some(0));
        assert_eq!(field.flat_index(0, 5), Some(5));
        assert_eq!(field.flat_index(1, 3), Some(12));
        assert_eq!(field.flat_index(3, 7), Some(34));
        assert_eq!(field.flat_index(3, 9), None);
    }

    #[test]
    fn node_test() {
        let mut field: Field<i32> = Field::with_size(4, 9);
        let node = field.node_mut(0, 0);
        assert!(node.is_some());
        *node.unwrap() = 5;

        let node = field.node(0, 0);
        assert!(node.is_some());
        assert_eq!(*node.unwrap(), 5);
    }
}
