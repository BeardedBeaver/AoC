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

    pub fn from_flat_vector(nodes: Vec<Node>, col_count: usize) -> Option<Self> {
        if nodes.len() % col_count != 0 {
            return None;
        }
        let row_count = nodes.len() / col_count;
        Some(Field {
            nodes: nodes,
            row_count: row_count,
            col_count: col_count,
        })
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
    fn from_flat_vector_test() {
        let field = Field::from_flat_vector(vec![0, 1, 2, 3, 4, 5], 3);
        assert!(field.is_some());
        let field = field.unwrap();

        assert_eq!(field.row_count, 2);
        assert_eq!(field.col_count, 3);
        assert_eq!(field.get_row_count(), 2);
        assert_eq!(field.get_col_count(), 3);

        let node = field.node(0, 1);
        assert!(node.is_some());
        let node = node.unwrap();
        assert_eq!(*node, 1);

        let node = field.node(1, 2);
        assert!(node.is_some());
        let node = node.unwrap();
        assert_eq!(*node, 5);
    }

    #[test]
    fn node_test() {
        // given
        let mut field: Field<i32> = Field::with_size(4, 9);

        // when: get a mutable node and change it
        let node = field.node_mut(0, 0);
        assert!(node.is_some());
        *node.unwrap() = 5;

        // then: both immutable and mutable nodes should
        //       hold an updated value
        let node = field.node(0, 0);
        assert!(node.is_some());
        assert_eq!(*node.unwrap(), 5);
        let node = field.node_mut(0, 0);
        assert!(node.is_some());
        assert_eq!(*node.unwrap(), 5);

        // when: update another node
        let node = field.node_mut(2, 4);
        assert!(node.is_some());
        *node.unwrap() = 12;

        // then: old value is still the same, the new
        //       value is updated
        let node = field.node(0, 0);
        assert!(node.is_some());
        assert_eq!(*node.unwrap(), 5);
        let node = field.node_mut(0, 0);
        assert!(node.is_some());
        assert_eq!(*node.unwrap(), 5);

        let node = field.node(2, 4);
        assert!(node.is_some());
        assert_eq!(*node.unwrap(), 12);
        let node = field.node_mut(2, 4);
        assert!(node.is_some());
        assert_eq!(*node.unwrap(), 12);

        // when: get both mutable and immutable reference to
        //       the node outside of field bounds
        let node = field.node_mut(4, 0);
        assert!(node.is_none());
        let node = field.node_mut(4, 9);
        assert!(node.is_none());
        let node = field.node_mut(0, 9);
        assert!(node.is_none());

        let node = field.node(4, 0);
        assert!(node.is_none());
        let node = field.node(4, 9);
        assert!(node.is_none());
        let node = field.node(0, 9);
        assert!(node.is_none());
    }
}
